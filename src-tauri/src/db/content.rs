use crate::models::ClipKind;

pub fn content_hash(content: &str) -> String {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = FNV_OFFSET;

    for byte in content.trim().as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    format!("{hash:016x}")
}

pub fn mime_type_for_kind(kind: &ClipKind) -> &'static str {
    match kind {
        ClipKind::Link => "text/uri-list",
        ClipKind::Code | ClipKind::Text => "text/plain",
    }
}
