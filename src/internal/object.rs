use std::collections::HashMap;
use super::super::*;

#[derive(Debug, Clone, Default)]
pub struct Object {
    content_length: Option<i64>,
    content_md5: Option<String>,
    content_type: Option<String>,
    etag: Option<String>,
    /// ID is the unique key in storage.
    pub id: String,
    last_modified: Option<chrono::DateTime<chrono::Utc>>,
    /// LinkTarget is the symlink target for link object.
    link_target: Option<String>,
    pub mode: ObjectMode,
    /// MultipartID is the part id of part object.
    multipart_id: Option<String>,
    /// Path is either the absolute path or the relative path towards storage's WorkDir depends on user's input.
    pub path: String,
    /// ServiceMetadata stores service defined metadata
    service_metadata: HashMap<String,String>,
    /// UserMetadata stores user defined metadata
    user_metadata: HashMap<String,String>,
}