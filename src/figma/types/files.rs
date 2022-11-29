use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
  name: String,
  last_modified: String,
  thumbnail_url: String,
  version: String,
  role: String,
  editor_type: String,
  link_access: String
}