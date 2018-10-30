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
// TODO(zoidbergwill): Trim some of these fields, since we only need some of them.
// https://github.com/serde-rs/serde/pull/201
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubPushEvent {
    #[serde(rename = "ref")]
    pub git_hub_push_event_ref: String,
    pub before: String,
    pub after: String,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub base_ref: Option<serde_json::Value>,
    pub compare: String,
    pub commits: Vec<Commit>,
    pub head_commit: Option<Commit>,
    pub repository: Repository,
    pub pusher: Pusher,
    pub sender: Sender,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub id: String,
    pub tree_id: String,
    pub distinct: bool,
    pub message: String,
    pub timestamp: String,
    pub url: String,
    pub author: Author,
    pub committer: Author,
    pub added: Vec<String>,
    pub removed: Vec<Option<serde_json::Value>>,
    pub modified: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub username: Name,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pusher {
    pub name: Name,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub owner: Sender,
    pub private: bool,
    pub html_url: String,
    pub description: Option<serde_json::Value>,
    pub fork: bool,
    pub url: String,
    pub forks_url: String,
    pub keys_url: String,
    pub collaborators_url: String,
    pub teams_url: String,
    pub hooks_url: String,
    pub issue_events_url: String,
    pub events_url: String,
    pub assignees_url: String,
    pub branches_url: String,
    pub tags_url: String,
    pub blobs_url: String,
    pub git_tags_url: String,
    pub git_refs_url: String,
    pub trees_url: String,
    pub statuses_url: String,
    pub languages_url: String,
    pub stargazers_url: String,
    pub contributors_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub commits_url: String,
    pub git_commits_url: String,
    pub comments_url: String,
    pub issue_comment_url: String,
    pub contents_url: String,
    pub compare_url: String,
    pub merges_url: String,
    pub archive_url: String,
    pub downloads_url: String,
    pub issues_url: String,
    pub pulls_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub labels_url: String,
    pub releases_url: String,
    pub deployments_url: String,
    pub created_at: i64,
    pub updated_at: String,
    pub pushed_at: i64,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub svn_url: String,
    pub homepage: Option<String>,
    pub size: i64,
    pub stargazers_count: i64,
    pub watchers_count: i64,
    pub language: Option<String>,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_downloads: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub forks_count: i64,
    pub mirror_url: Option<serde_json::Value>,
    pub archived: bool,
    pub open_issues_count: i64,
    pub license: Option<serde_json::Value>,
    pub forks: i64,
    pub open_issues: i64,
    pub watchers: i64,
    pub default_branch: String,
    pub stargazers: i64,
    pub master_branch: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sender {
    pub name: Option<Name>,
    pub email: Option<String>,
    pub login: Name,
    pub id: i64,
    pub node_id: NodeId,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: FollowingUrl,
    pub gists_url: GistsUrl,
    pub starred_url: StarredUrl,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: EventsUrl,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub sender_type: Type,
    pub site_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubIssuesEvent {
    pub action: String,
    pub issue: Issue,
    pub changes: Changes,
    pub repository: Repo,
    pub sender: Sender,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Changes {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub url: String,
    pub repository_url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub number: i64,
    pub title: String,
    pub user: Sender,
    pub labels: Vec<Label>,
    pub state: String,
    pub locked: bool,
    pub assignee: Option<serde_json::Value>,
    pub assignees: Vec<Option<serde_json::Value>>,
    pub milestone: Option<serde_json::Value>,
    pub comments: i64,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<serde_json::Value>,
    pub author_association: String,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub name: String,
    pub color: String,
    #[serde(rename = "default")]
    pub label_default: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repo {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub owner: Sender,
    pub private: bool,
    pub html_url: String,
    pub description: Option<serde_json::Value>,
    pub fork: bool,
    pub url: String,
    pub forks_url: String,
    pub keys_url: String,
    pub collaborators_url: String,
    pub teams_url: String,
    pub hooks_url: String,
    pub issue_events_url: String,
    pub events_url: String,
    pub assignees_url: String,
    pub branches_url: String,
    pub tags_url: String,
    pub blobs_url: String,
    pub git_tags_url: String,
    pub git_refs_url: String,
    pub trees_url: String,
    pub statuses_url: String,
    pub languages_url: String,
    pub stargazers_url: String,
    pub contributors_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub commits_url: String,
    pub git_commits_url: String,
    pub comments_url: String,
    pub issue_comment_url: String,
    pub contents_url: String,
    pub compare_url: String,
    pub merges_url: String,
    pub archive_url: String,
    pub downloads_url: String,
    pub issues_url: String,
    pub pulls_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub labels_url: String,
    pub releases_url: String,
    pub deployments_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub svn_url: String,
    pub homepage: Option<serde_json::Value>,
    pub size: i64,
    pub stargazers_count: i64,
    pub watchers_count: i64,
    pub language: Option<serde_json::Value>,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_downloads: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub forks_count: i64,
    pub mirror_url: Option<serde_json::Value>,
    pub archived: bool,
    pub open_issues_count: i64,
    pub license: Option<serde_json::Value>,
    pub forks: i64,
    pub open_issues: i64,
    pub watchers: i64,
    pub default_branch: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubPullRequestEvent {
    pub action: String,
    pub number: i64,
    pub pull_request: PullRequest,
    pub repository: Repo,
    pub sender: Sender,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequest {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub issue_url: String,
    pub number: i64,
    pub state: String,
    pub locked: bool,
    pub title: String,
    pub user: Sender,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: String,
    pub merged_at: Option<serde_json::Value>,
    pub merge_commit_sha: String,
    pub assignee: Option<serde_json::Value>,
    pub assignees: Vec<Option<serde_json::Value>>,
    pub requested_reviewers: Vec<Option<serde_json::Value>>,
    pub requested_teams: Vec<Option<serde_json::Value>>,
    pub labels: Vec<Option<serde_json::Value>>,
    pub milestone: Option<serde_json::Value>,
    pub commits_url: String,
    pub review_comments_url: String,
    pub review_comment_url: String,
    pub comments_url: String,
    pub statuses_url: String,
    pub head: Base,
    pub base: Base,
    #[serde(rename = "_links")]
    pub links: Links,
    pub author_association: String,
    pub merged: bool,
    pub mergeable: bool,
    pub rebaseable: bool,
    pub mergeable_state: String,
    pub merged_by: Option<serde_json::Value>,
    pub comments: i64,
    pub review_comments: i64,
    pub maintainer_can_modify: bool,
    pub commits: i64,
    pub additions: i64,
    pub deletions: i64,
    pub changed_files: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Base {
    pub label: String,
    #[serde(rename = "ref")]
    pub base_ref: String,
    pub sha: String,
    pub user: Sender,
    pub repo: Repo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub links_self: Comments,
    pub html: Comments,
    pub issue: Comments,
    pub comments: Comments,
    pub review_comments: Comments,
    pub review_comment: Comments,
    pub commits: Comments,
    pub statuses: Comments,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comments {
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Name {
    Codertocat,
    #[serde(rename = "zoidbergwill")]
    Zoidbergwill,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventsUrl {
    #[serde(rename = "https://api.github.com/users/Codertocat/events{/privacy}")]
    HttpsApiGithubComUsersCodertocatEventsPrivacy,
    #[serde(rename = "https://api.github.com/users/zoidbergwill/events{/privacy}")]
    HttpsApiGithubComUsersZoidbergwillEventsPrivacy,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FollowingUrl {
    #[serde(rename = "https://api.github.com/users/Codertocat/following{/other_user}")]
    HttpsApiGithubComUsersCodertocatFollowingOtherUser,
    #[serde(rename = "https://api.github.com/users/zoidbergwill/following{/other_user}")]
    HttpsApiGithubComUsersZoidbergwillFollowingOtherUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GistsUrl {
    #[serde(rename = "https://api.github.com/users/Codertocat/gists{/gist_id}")]
    HttpsApiGithubComUsersCodertocatGistsGistId,
    #[serde(rename = "https://api.github.com/users/zoidbergwill/gists{/gist_id}")]
    HttpsApiGithubComUsersZoidbergwillGistsGistId,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NodeId {
    #[serde(rename = "MDQ6VXNlcjI1NzI0OTM=")]
    Mdq6VxNlcjI1NzI0Otm,
    #[serde(rename = "MDQ6VXNlcjIxMDMxMDY3")]
    Mdq6VxNlcjIxMdMxMdy3,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StarredUrl {
    #[serde(rename = "https://api.github.com/users/Codertocat/starred{/owner}{/repo}")]
    HttpsApiGithubComUsersCodertocatStarredOwnerRepo,
    #[serde(rename = "https://api.github.com/users/zoidbergwill/starred{/owner}{/repo}")]
    HttpsApiGithubComUsersZoidbergwillStarredOwnerRepo,
}
