//! Run the built-in search benchmarks and print a JSON report.

use std::error::Error;
use std::io::{self, Write};
use std::process::ExitCode;
use std::sync::Arc;

use dok3_benchmark::{EvalConfig, LoadConfig, evaluate, poc_suite_for_builtin_samples, run_load};
use dok3_core::Dok3Error;
use dok3_domain::{Principal, SearchRequest};
use dok3_search::{InMemorySearchRepository, SearchService};
use serde::Serialize;

const USAGE: &str = "Usage: dok3-benchmark eval [k]\n       dok3-benchmark load <threads> <iterations_per_thread> <warmup_iterations> [query]";

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(passed) => ExitCode::from(u8::from(!passed)),
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: Vec<String>) -> Result<bool, Box<dyn Error>> {
    let repo = Arc::new(InMemorySearchRepository::from_seed(None));
    let service = Arc::new(SearchService::new(repo.clone(), repo));
    match args.as_slice() {
        [help] if help == "--help" || help == "-h" => println!("{USAGE}"),
        [command, rest @ ..] if command == "eval" && rest.len() <= 1 => {
            let mut config = EvalConfig::default();
            if let Some(k) = rest.first() {
                config.k = parse_count("k", k)?;
            }
            let report = evaluate(&service, &poc_suite_for_builtin_samples(), config)?;
            print_report(&report)?;
            return Ok(report.acl_violations == 0);
        }
        [command, threads, iterations, warmup, rest @ ..]
            if command == "load" && rest.len() <= 1 =>
        {
            let config = LoadConfig {
                threads: parse_count("threads", threads)?,
                iterations_per_thread: parse_count("iterations_per_thread", iterations)?,
                warmup_iterations: parse_count("warmup_iterations", warmup)?,
            };
            let request = SearchRequest {
                query: rest.first().cloned().unwrap_or_else(|| "Rust".into()),
                ..SearchRequest::default()
            };
            let report = run_load(service, &Principal::anonymous_public(), &request, config)?;
            print_report(&report)?;
            return Ok(report.error_count == 0);
        }
        _ => return Err(Dok3Error::InvalidInput(USAGE.into()).into()),
    }
    Ok(true)
}

fn parse_count(name: &str, value: &str) -> Result<usize, Dok3Error> {
    value
        .parse()
        .map_err(|_| Dok3Error::InvalidInput(format!("{name} must be a nonnegative integer")))
}

fn print_report(report: &impl Serialize) -> Result<(), Box<dyn Error>> {
    let mut output = io::stdout().lock();
    serde_json::to_writer_pretty(&mut output, report)?;
    writeln!(output)?;
    Ok(())
}
