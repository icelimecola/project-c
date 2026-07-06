use serde::Serialize;

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
    pub source: String,
    pub time: String,
    pub pinned: bool,
    pub kind: ClipKind,
}

#[derive(Clone, Serialize)]
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
