//! Reproducible search quality and concurrent load benchmarks.

pub mod evaluator;
pub mod load;
mod suite;

pub use evaluator::{
    CaseResult, EvalConfig, EvalReport, GroupResult, QueryCase, QueryGroup, evaluate,
};
pub use load::{LoadConfig, LoadReport, run_load};
pub use suite::poc_suite_for_builtin_samples;

pub fn init() {}
