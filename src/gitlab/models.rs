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
#[derive(Debug, Serialize, Deserialize)]
pub struct GitlabPush {
    object_kind: String,
    before: String,
    after: String,
    #[serde(rename = "ref")]
    git_lab_push_ref: String,
    checkout_sha: String,
    user_id: i64,
    user_name: String,
    user_username: String,
    user_email: String,
    user_avatar: String,
    project_id: i64,
    project: Project,
    repository: Repository,
    commits: Vec<Commit>,
    total_commits_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    id: String,
    message: String,
    timestamp: String,
    url: String,
    author: Author,
    added: Vec<String>,
    modified: Vec<String>,
    removed: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    name: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    id: i64,
    name: String,
    description: String,
    web_url: String,
    avatar_url: Option<serde_json::Value>,
    git_ssh_url: String,
    git_http_url: String,
    namespace: String,
    visibility_level: i64,
    path_with_namespace: String,
    default_branch: String,
    homepage: String,
    url: String,
    ssh_url: String,
    http_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    name: String,
    url: String,
    description: String,
    homepage: String,
    git_http_url: String,
    git_ssh_url: String,
    visibility_level: i64,
}
