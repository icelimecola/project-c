use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub hotkey: String,
    pub count: u32,
    pub color: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Clip {
    pub id: u32,
    pub folder_id: String,
    pub title: String,
    pub content: String,
    pub content_hash: String,
    pub source: String,
    pub source_app: Option<String>,
    pub time: String,
    pub created_at: String,
    pub updated_at: String,
    pub last_used_at: Option<String>,
    pub mime_type: String,
    pub deleted_at: Option<String>,
    pub pinned: bool,
    pub kind: ClipKind,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClipKind {
    Text,
    Code,
    Link,
}

impl ClipKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Code => "code",
            Self::Link => "link",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "code" => Self::Code,
            "link" => Self::Link,
            _ => Self::Text,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClipPayload {
    pub folder_id: String,
    pub title: String,
    pub content: String,
    pub source: String,
    pub source_app: Option<String>,
    pub mime_type: Option<String>,
    pub kind: ClipKind,
}
