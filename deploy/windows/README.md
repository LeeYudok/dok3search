# dok3search Windows 온프레미스 서비스 배포 (P7-1)

폐쇄망 Windows Server / PC에서 `dok3search.exe`를 Windows Service로 등록·운영·검증하는 절차입니다.

## 사전 준비

1. [오프라인 번들 안내](../../docs/offline-bundle.md) 절차로 `dok3search.exe`와 `frontend/dist`를 빌드합니다.
2. 한국어 형태소 분석을 켜려면 `ko-dic.bin`을 로컬 경로(예: `C:\dok3search\dict\ko-dic.bin`)에 배치합니다.

## 서비스 등록 및 기동

관리자 PowerShell에서 실행합니다.

```powershell
.\deploy\windows\install-service.ps1 `
  -BinaryPath "C:\dok3search\bin\dok3search.exe" `
  -SourceDir "C:\dok3search\knowledge" `
  -DataDir "C:\dok3search\data" `
  -StaticDir "C:\dok3search\frontend\dist" `
  -AnalyzerProfile "nori-mixed" `
  -DictPath "C:\dok3search\dict\ko-dic.bin"
```

- `sc.exe failure`로 장애 시 5초/15초/30초 자동 재시작 복구 정책이 등록됩니다.
- NSSM을 사용하는 경우 `-NssmPath "C:\tools\nssm.exe"`를 지정하면 stdout/stderr 파일 로그(`%DataDir%\logs\`)도 함께 연결됩니다.

## 네이티브 SCM 모드와 NSSM 대안

- **네이티브 SCM(기본)**: `install-service.ps1`은 `sc.exe create ... binPath= "\"<exe>\" --windows-service"`로 등록합니다. `--windows-service` 없이 일반 콘솔 바이너리를 `sc.exe`로 등록하면 SCM 응답이 없어 `Error 1053`으로 기동에 실패합니다. 이 플래그가 있으면 바이너리가 SCM 핸들러를 등록해 즉시 `START_PENDING`(대기 30초)을 보고하고, 포트 바인드 직후 `RUNNING`, 중지·시스템 종료 요청 시 `STOP_PENDING` 후 주기 워커를 멈추고 `STOPPED`를 보고합니다. 환경 변수(`DOK3_*`)는 머신 수준으로 저장됩니다.
- `--windows-service`는 `--mcp-stdio`와 함께 쓸 수 없습니다. Windows가 아닌 대상에서는 오류로 종료합니다.
- **NSSM 대안**: `-NssmPath`를 주면 NSSM이 콘솔 바이너리를 감싸므로 `--windows-service`를 붙이지 않습니다.

## 기동·재시작 복구 Smoke 테스트

```powershell
.\deploy\windows\smoke-windows.ps1 -BaseUrl "http://127.0.0.1:8080"
```

## 서비스 제거 (데이터 보존)

```powershell
.\deploy\windows\uninstall-service.ps1
```

`DataDir`(`generations/`, `storage/`, `index_jobs.json`, `sources.json`)은 삭제하지 않습니다.
