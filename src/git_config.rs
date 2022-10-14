use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct GitConfig {
    pub repository_url: String,
    pub repository_local_dir: String,

    pub remote_name: String,
    pub remote_branch: String,
}
