// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }
//
// Thanks quicktype.io
//
// TODO(zoidbergwill): Trim some of these fields, since we only need some of them.
// https://github.com/serde-rs/serde/pull/201
#[derive(Debug, Serialize, Deserialize)]
pub struct GitlabPush {
    pub object_kind: String,
    pub before: String,
    pub after: String,
    #[serde(rename = "ref")]
    pub git_lab_push_ref: String,
    pub checkout_sha: String,
    pub user_id: i64,
    pub user_name: String,
    pub user_username: String,
    pub user_email: String,
    pub user_avatar: String,
    pub project_id: i64,
    pub project: Project,
    pub repository: Repository,
    pub commits: Vec<Commit>,
    pub total_commits_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub url: String,
    pub author: Author,
    pub added: Vec<String>,
    pub modified: Vec<String>,
    pub removed: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub web_url: String,
    pub avatar_url: Option<serde_json::Value>,
    pub git_ssh_url: String,
    pub git_http_url: String,
    pub namespace: String,
    pub visibility_level: i64,
    pub path_with_namespace: String,
    pub default_branch: String,
    pub homepage: String,
    pub url: String,
    pub ssh_url: String,
    pub http_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub description: String,
    pub homepage: String,
    pub git_http_url: String,
    pub git_ssh_url: String,
    pub visibility_level: i64,
}
