use crate::DateTime;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
// #[serde(rename_all = "camelCase")]
pub struct Entry<E> {
    id: ulid::Ulid,
    last_modified_by: String,
    entry: E,
    // entry_id: String,
    discarded: bool,
    version: u32,
    last_modified: DateTime,
}

impl<E> Entry<E> {
    pub fn entry(&self) -> &E {
        &self.entry
    }
}
