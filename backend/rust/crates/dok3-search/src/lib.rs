//! Search orchestration, ACL enforcement, and original/section access.

pub mod in_memory;
pub mod seed;
pub mod service;

pub use in_memory::InMemorySearchRepository;
pub use seed::{SeedDataset, document_heading_id, load_seed_dataset};
pub use service::SearchService;
