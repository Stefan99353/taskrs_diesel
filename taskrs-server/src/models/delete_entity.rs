use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteEntityParams {
    pub id: i32,
    #[serde(alias = "c")]
    pub cascade: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeleteEntityResult<T> {
    Ok,
    NotFound,
    Referenced(Vec<T>),
}
