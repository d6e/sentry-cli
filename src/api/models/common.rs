use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Actor {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    #[serde(rename = "type")]
    pub actor_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectRef {
    pub id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub detail: String,
}
