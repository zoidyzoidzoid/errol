table! {
    github_issues (id) {
        id -> Int4,
        number -> Int4,
        title -> Varchar,
        body -> Text,
        labels -> Text,
        url -> Text,
    }
}
