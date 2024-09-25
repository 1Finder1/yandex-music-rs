use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub id: i32,
    pub name: String,
}
