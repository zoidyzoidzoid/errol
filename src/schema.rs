table! {
    github_issues (id) {
        github_repo_id -> Nullable<Int4>,
        id -> Int4,
        number -> Int4,
        title -> Varchar,
        body -> Text,
        labels -> Text,
        url -> Text,
        data -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    github_issues_fetches (id) {
        github_repo_id -> Nullable<Int4>,
        id -> Int4,
        repo -> Varchar,
        data -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    github_pulls (id) {
        github_repo_id -> Nullable<Int4>,
        id -> Int4,
        number -> Int4,
        title -> Varchar,
        repo -> Varchar,
        body -> Text,
        labels -> Text,
        url -> Text,
        data -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    github_pulls_fetches (id) {
        github_repo_id -> Nullable<Int4>,
        id -> Int4,
        repo -> Varchar,
        data -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    github_repos (id) {
        id -> Int4,
        repo -> Varchar,
        url -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    rules (id) {
        id -> Int4,
        authors -> Nullable<Array<Text>>,
        branches -> Nullable<Array<Text>>,
        paths -> Nullable<Array<Text>>,
        projects -> Nullable<Array<Text>>,
        reply_to -> Nullable<Text>,
        to -> Nullable<Array<Text>>,
    }
}

joinable!(github_issues -> github_repos (github_repo_id));
joinable!(github_issues_fetches -> github_repos (github_repo_id));
joinable!(github_pulls -> github_repos (github_repo_id));
joinable!(github_pulls_fetches -> github_repos (github_repo_id));

allow_tables_to_appear_in_same_query!(
    github_issues,
    github_issues_fetches,
    github_pulls,
    github_pulls_fetches,
    github_repos,
    rules,
);
