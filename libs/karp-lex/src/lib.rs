pub mod application;
pub mod domain;

type DateTime = chrono::DateTime<chrono::Utc>;

pub use domain::entities::Entry;
