#[derive(Queryable)]
pub struct GitHubIssue {
    pub id: i32,
    pub number: i32,
    pub title: String,
    pub body: String,
    pub labels: String,
    pub url: String,
}
