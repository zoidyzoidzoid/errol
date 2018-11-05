table! {
    authors (id) {
        id -> Int4,
        value -> Nullable<Text>,
    }
}

table! {
    branches (id) {
        id -> Int4,
        value -> Nullable<Text>,
    }
}

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
    paths (id) {
        id -> Int4,
        value -> Nullable<Text>,
    }
}

table! {
    projects (id) {
        id -> Int4,
        value -> Nullable<Text>,
    }
}

table! {
    reply_to (id) {
        id -> Int4,
        value -> Nullable<Text>,
    }
}

table! {
    rules (id) {
        id -> Int4,
    }
}

table! {
    rules_authors (id) {
        id -> Int4,
        key_id -> Nullable<Int4>,
        rule_id -> Nullable<Int4>,
    }
}

table! {
    rules_branches (id) {
        id -> Int4,
        key_id -> Nullable<Int4>,
        rule_id -> Nullable<Int4>,
    }
}

table! {
    rules_paths (id) {
        id -> Int4,
        key_id -> Nullable<Int4>,
        rule_id -> Nullable<Int4>,
    }
}

table! {
    rules_projects (id) {
        id -> Int4,
        key_id -> Nullable<Int4>,
        rule_id -> Nullable<Int4>,
    }
}

table! {
    rules_reply_to (id) {
        id -> Int4,
        key_id -> Nullable<Int4>,
        rule_id -> Nullable<Int4>,
    }
}

table! {
    rules_to (id) {
        id -> Int4,
        key_id -> Nullable<Int4>,
        rule_id -> Nullable<Int4>,
    }
}

table! {
    to (id) {
        id -> Int4,
        value -> Nullable<Text>,
    }
}

joinable!(github_issues_fetches -> github_repos (github_repo_id));
joinable!(github_pulls_fetches -> github_repos (github_repo_id));
joinable!(github_pulls -> github_repos (github_repo_id));
joinable!(rules_authors -> authors (key_id));
joinable!(rules_authors -> rules (rule_id));
joinable!(rules_branches -> branches (key_id));
joinable!(rules_branches -> rules (rule_id));
joinable!(rules_paths -> paths (key_id));
joinable!(rules_paths -> rules (rule_id));
joinable!(rules_projects -> projects (key_id));
joinable!(rules_projects -> rules (rule_id));
joinable!(rules_reply_to -> reply_to (key_id));
joinable!(rules_reply_to -> rules (rule_id));
joinable!(rules_to -> rules (rule_id));
joinable!(rules_to -> to (key_id));

allow_tables_to_appear_in_same_query!(
    authors,
    branches,
    github_issues,
    github_issues_fetches,
    github_pulls_fetches,
    github_pulls,
    github_repos,
    paths,
    projects,
    reply_to,
    rules,
    rules_authors,
    rules_branches,
    rules_paths,
    rules_projects,
    rules_reply_to,
    rules_to,
    to,
);
