use crate::DateTime;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryDto<E> {
    pub entity_id: Option<uuid::Uuid>,
    pub last_modified_by: String,
    pub entry: E,
    pub entry_id: Option<String>,
    pub discarded: bool,
    pub version: u32,
    pub last_modified: DateTime,
    pub message: String,
}
