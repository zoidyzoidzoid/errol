extern crate diesel;
extern crate errol;

use errol::models::GitHubIssue;
#[allow(unused_imports)] use diesel::prelude::*;
use errol::*;

fn main() {
    use errol::schema::github_issues::dsl::*;
    let connection = establish_connection();
    let results = github_issues
        .limit(5)
        .load::<GitHubIssue>(&connection)
        .expect("Error loading GitHub issues");

    println!("Displaying {} GitHub issues", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
