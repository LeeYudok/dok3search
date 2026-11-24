//! Windows SCM service support.
//!
//! The platform-neutral lifecycle core (state machine + shutdown controller) is compiled and
//! tested on every target; only the `windows-service` glue is `cfg(windows)`.
#![cfg_attr(not(windows), allow(dead_code))]

use std::future::Future;
use std::sync::Arc;

use tokio::sync::watch;

/// Service name registered with the SCM (matches `deploy/windows/install-service.ps1`).
pub const SERVICE_NAME: &str = "dok3search";

/// Lifecycle states reported to the SCM.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceLifecycleState {
    StartPending,
    Running,
    StopPending,
    Stopped,
}

struct ControllerInner {
    stop_tx: watch::Sender<bool>,
    state_tx: watch::Sender<ServiceLifecycleState>,
}

/// Cloneable handle that requests shutdown and tracks the lifecycle state via `watch` channels.
#[derive(Clone)]
pub struct ServiceShutdownController {
    inner: Arc<ControllerInner>,
}

impl Default for ServiceShutdownController {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceShutdownController {
    pub fn new() -> Self {
        let (stop_tx, _) = watch::channel(false);
        let (state_tx, _) = watch::channel(ServiceLifecycleState::StartPending);
        Self {
            inner: Arc::new(ControllerInner { stop_tx, state_tx }),
        }
    }

    /// Requests a graceful shutdown (idempotent).
    pub fn stop(&self) {
        self.inner.stop_tx.send_replace(true);
    }

    pub fn is_stop_requested(&self) -> bool {
        *self.inner.stop_tx.borrow()
    }

    /// Future that resolves once `stop()` has been called (immediately if it already was).
    pub fn shutdown_signal(&self) -> impl Future<Output = ()> + Send + 'static {
        let mut rx = self.inner.stop_tx.subscribe();
        async move {
            let _ = rx.wait_for(|stop| *stop).await;
        }
    }

    pub fn state(&self) -> ServiceLifecycleState {
        *self.inner.state_tx.borrow()
    }

    pub fn set_state(&self, state: ServiceLifecycleState) {
        self.inner.state_tx.send_replace(state);
    }

    pub fn subscribe_state(&self) -> watch::Receiver<ServiceLifecycleState> {
        self.inner.state_tx.subscribe()
    }
}

/// Callback invoked on each lifecycle transition; `Err` carries the failure for `Stopped`.
pub type StatusReporter = Arc<dyn Fn(ServiceLifecycleState, bool) + Send + Sync>;

/// Drives the lifecycle: `StartPending` → (`on_ready`) `Running` → (stop) `StopPending` →
/// server returns (workers stopped by the server fn) → `Stopped`. The reporter's bool is `true`
/// on success and only meaningful for `Stopped`.
pub async fn run_lifecycle<S, Fut>(
    controller: &ServiceShutdownController,
    report: StatusReporter,
    server: S,
) -> Result<(), String>
where
    S: FnOnce(Box<dyn Future<Output = ()> + Send>, Box<dyn FnOnce() + Send>) -> Fut,
    Fut: Future<Output = Result<(), String>>,
{
    let set = {
        let controller = controller.clone();
        let report = Arc::clone(&report);
        move |state: ServiceLifecycleState| {
            controller.set_state(state);
            report(state, true);
        }
    };
    set(ServiceLifecycleState::StartPending);

    let stop_wait = controller.shutdown_signal();
    let shutdown = {
        let set = set.clone();
        async move {
            stop_wait.await;
            set(ServiceLifecycleState::StopPending);
        }
    };
    let on_ready = {
        let set = set.clone();
        move || set(ServiceLifecycleState::Running)
    };

    let result = server(Box::new(shutdown), Box::new(on_ready)).await;
    controller.set_state(ServiceLifecycleState::Stopped);
    report(ServiceLifecycleState::Stopped, result.is_ok());
    result
}

#[cfg(not(windows))]
pub fn run_as_service() -> Result<(), String> {
    Err("--windows-service is only supported on Windows targets".to_string())
}

#[cfg(windows)]
pub fn run_as_service() -> Result<(), String> {
    windows_impl::run()
}

#[cfg(windows)]
mod windows_impl {
    use std::ffi::OsString;
    use std::sync::Arc;
    use std::time::Duration;

    use windows_service::service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    };
    use windows_service::service_control_handler::{
        self, ServiceControlHandlerResult, ServiceStatusHandle,
    };
    use windows_service::{define_windows_service, service_dispatcher};

    use super::{
        SERVICE_NAME, ServiceLifecycleState, ServiceShutdownController, StatusReporter,
        run_lifecycle,
    };

    const WAIT_HINT: Duration = Duration::from_secs(30);

    define_windows_service!(ffi_service_main, service_main);

    pub fn run() -> Result<(), String> {
        service_dispatcher::start(SERVICE_NAME, ffi_service_main)
            .map_err(|err| format!("failed to start service dispatcher: {err}"))
    }

    fn service_main(_arguments: Vec<OsString>) {
        if let Err(err) = run_service() {
            eprintln!("dok3search service error: {err}");
        }
    }

    fn status(
        state: ServiceState,
        accepted: ServiceControlAccept,
        exit: u32,
        checkpoint: u32,
        wait_hint: Duration,
    ) -> ServiceStatus {
        ServiceStatus {
            service_type: ServiceType::OWN_PROCESS,
            current_state: state,
            controls_accepted: accepted,
            exit_code: ServiceExitCode::Win32(exit),
            checkpoint,
            wait_hint,
            process_id: None,
        }
    }

    fn report_status(handle: &ServiceStatusHandle, state: ServiceLifecycleState, ok: bool) {
        let next = match state {
            ServiceLifecycleState::StartPending => status(
                ServiceState::StartPending,
                ServiceControlAccept::empty(),
                0,
                1,
                WAIT_HINT,
            ),
            ServiceLifecycleState::Running => status(
                ServiceState::Running,
                ServiceControlAccept::STOP | ServiceControlAccept::SHUTDOWN,
                0,
                0,
                Duration::default(),
            ),
            ServiceLifecycleState::StopPending => status(
                ServiceState::StopPending,
                ServiceControlAccept::empty(),
                0,
                1,
                WAIT_HINT,
            ),
            ServiceLifecycleState::Stopped => status(
                ServiceState::Stopped,
                ServiceControlAccept::empty(),
                if ok { 0 } else { 1 },
                0,
                Duration::default(),
            ),
        };
        if let Err(err) = handle.set_service_status(next) {
            eprintln!("dok3search failed to report service status: {err}");
        }
    }

    fn run_service() -> Result<(), String> {
        let controller = ServiceShutdownController::new();
        let handler_controller = controller.clone();
        let handle = service_control_handler::register(SERVICE_NAME, move |event| match event {
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            ServiceControl::Stop | ServiceControl::Shutdown => {
                handler_controller.stop();
                ServiceControlHandlerResult::NoError
            }
            _ => ServiceControlHandlerResult::NotImplemented,
        })
        .map_err(|err| format!("failed to register service control handler: {err}"))?;

        let report: StatusReporter = Arc::new(move |state, ok| report_status(&handle, state, ok));
        let runtime = match tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
        {
            Ok(runtime) => runtime,
            Err(err) => {
                report_status(&handle, ServiceLifecycleState::Stopped, false);
                return Err(format!("failed to build tokio runtime: {err}"));
            }
        };
        runtime.block_on(run_lifecycle(&controller, report, |shutdown, on_ready| {
            crate::run_server(false, Box::into_pin(shutdown), on_ready)
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use axum::Router;
    use tokio::net::TcpListener;

    use super::*;

    type RecordedStatuses = Arc<Mutex<Vec<(ServiceLifecycleState, bool)>>>;

    fn recorder() -> (StatusReporter, RecordedStatuses) {
        let log = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&log);
        let report: StatusReporter =
            Arc::new(move |state, ok| sink.lock().unwrap().push((state, ok)));
        (report, log)
    }

    #[tokio::test]
    async fn controller_signals_and_tracks_state() {
        let controller = ServiceShutdownController::new();
        assert_eq!(controller.state(), ServiceLifecycleState::StartPending);
        assert!(!controller.is_stop_requested());
        let mut rx = controller.subscribe_state();
        controller.set_state(ServiceLifecycleState::Running);
        rx.changed().await.unwrap();
        assert_eq!(*rx.borrow(), ServiceLifecycleState::Running);
        controller.stop();
        controller.stop();
        // Signal obtained after stop() resolves immediately.
        controller.shutdown_signal().await;
        assert!(controller.is_stop_requested());
    }

    #[tokio::test]
    async fn lifecycle_reports_states_in_order_with_real_listener() {
        let controller = ServiceShutdownController::new();
        let (report, log) = recorder();
        let stopper = controller.clone();
        let result = run_lifecycle(&controller, report, |shutdown, on_ready| async move {
            let listener = TcpListener::bind("127.0.0.1:0")
                .await
                .map_err(|e| e.to_string())?;
            on_ready();
            stopper.stop();
            dok3_api::serve_with_shutdown(listener, Router::new(), Box::into_pin(shutdown))
                .await
                .map_err(|e| e.to_string())
        })
        .await;
        assert_eq!(result, Ok(()));
        assert_eq!(
            *log.lock().unwrap(),
            vec![
                (ServiceLifecycleState::StartPending, true),
                (ServiceLifecycleState::Running, true),
                (ServiceLifecycleState::StopPending, true),
                (ServiceLifecycleState::Stopped, true),
            ]
        );
        assert_eq!(controller.state(), ServiceLifecycleState::Stopped);
    }

    #[tokio::test]
    async fn lifecycle_reports_failure_on_server_error() {
        let controller = ServiceShutdownController::new();
        let (report, log) = recorder();
        let result = run_lifecycle(&controller, report, |_shutdown, _ready| async {
            Err("bind failed".to_string())
        })
        .await;
        assert!(result.is_err());
        let log = log.lock().unwrap();
        assert_eq!(log.last(), Some(&(ServiceLifecycleState::Stopped, false)));
    }

    #[cfg(not(windows))]
    #[test]
    fn windows_service_flag_errors_off_windows() {
        let err = run_as_service().unwrap_err();
        assert_eq!(
            err,
            "--windows-service is only supported on Windows targets"
        );
    }
}
