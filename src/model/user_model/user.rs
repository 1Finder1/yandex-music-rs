use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub uid: i32,
    pub login: String,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub full_name: Option<String>,
    pub sex: Option<String>,
    pub verified: Option<bool>,
    #[serde(default)]
    pub regions: Vec<i32>,
}
