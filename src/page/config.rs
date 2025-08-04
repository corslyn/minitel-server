use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PageConfig {
    pub name: String,
    pub path: String,
    pub routes: Option<HashMap<String, String>>,
}
