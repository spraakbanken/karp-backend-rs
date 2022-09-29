use crate::DateTime;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Entry<E> {
    entity_id: uuid::Uuid,
    last_modified_by: String,
    entry: E,
    entry_id: String,
    discarded: bool,
    version: u32,
    last_modified: DateTime,
}

impl<E> Entry<E> {
    pub fn entry(&self) -> &E {
        &self.entry
    }
}
