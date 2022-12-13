use crate::DateTime;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryDto<E> {
    // #[serde(alias = "entity_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<ulid::Ulid>,
    // #[serde(alias = "last_modified_by")]
    pub last_modified_by: String,
    pub entry: E,
    // #[serde(alias = "entry_id")]
    // pub entry_id: Option<String>,
    pub discarded: bool,
    pub version: u32,
    // #[serde(alias = "last_modified")]
    pub last_modified: DateTime,
    pub message: String,
}
