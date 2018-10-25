#![feature(proc_macro_hygiene, decl_macro, custom_attribute, plugin)]
#![plugin(rocket_codegen)]
// Needed for serde_derivce and rocket
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

extern crate rocket;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

extern crate rocket_contrib;
use rocket_contrib::{Json, Value};

mod gitlab;
use gitlab::models::GitlabPush;
mod github;
use github::models::GitHubPushEvent;

enum EventType {
    Issue,
    PullRequest,
    Push,
    Unsupported,
}

enum Provider {
    GitHub,
    Gitlab,
}

struct Event {
    provider: Provider,
    event_type: EventType,
}

impl<'a, 'r> FromRequest<'a, 'r> for Event {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Event, ()> {
        let github_event_type: Vec<_> = request.headers().get("X-GitHub-Event").collect();
        let gitlab_event_type: Vec<_> = request.headers().get("X-Gitlab-Event").collect();

        if github_event_type.len() != 1 && gitlab_event_type.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let event_type: EventType;
        let provider: Provider;
        if github_event_type.len() == 1 {
            provider = Provider::GitHub;
            let event = github_event_type[0];
            event_type = match event {
                "issues" => EventType::Issue,
                "pull_request" => EventType::PullRequest,
                "push" => EventType::Push,
                _ => EventType::Unsupported
            };
        } else if gitlab_event_type.len() == 1 {
            provider = Provider::Gitlab;
            let event = gitlab_event_type[0];
            event_type = match event {
                "Push Hook" => EventType::Push,
                _ => EventType::Unsupported
            };
        } else {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        match event_type {
            EventType::Unsupported => Outcome::Forward(()),
            _ => Outcome::Success(Event { event_type, provider } )
        }
    }
}

#[post("/github", format="application/json", data="<payload>")]
fn github_push(event: Event, payload: Json<Value>) -> String {
    match event.event_type {
        EventType::Unsupported => return "Invalid Request".to_string(),
        _ => {}
    }

    println!("Received push: {:?}", payload);
    match event.provider {
        Provider::GitHub => match event.event_type {
            EventType::Push => {
                process_github_push(payload);
            },
            _ => return "Unsupported Event Type".to_string(),
        },
        _ => return "Unsupported Provider".to_string(),
    };
    "GitHub hook received 🙏!".to_string()
}

#[post("/gitlab", format="application/json", data="<payload>")]
fn gitlab_push(event: Event, payload: Json<Value>) -> String {
    match event.event_type {
        EventType::Unsupported => return "Invalid Request".to_string(),
        _ => {}
    }

    println!("Received push: {:?}", payload);
    match event.provider {
        Provider::GitHub => match event.event_type {
            EventType::Push => process_github_push(payload),
            _ => return "Unsupported Event Type".to_string(),
        },
        Provider::Gitlab => match event.event_type {
            EventType::Push => process_gitlab_push(payload),
            _ => return "Unsupported Event Type".to_string(),
        },
    };
    "Hello, World!".to_string()
}

fn process_github_push(payload: Json<Value>) {
    let push: GitHubPushEvent = serde_json::from_value(payload.0).unwrap();
    if push.commits.len() >= 20 {
        // We'll support this later. GitHub and Gitlab both don't include more
        // than 20 commits in a push event.
        // GitHub's Events API docs talk about .size and .distinct_size, but I 
        // haven't got them in a payload yet.
        // https://developer.github.com/v3/activity/events/types/#pushevent
        println!("There might have been too many many commits: {}", push.commits.len());
    }
    for commit in push.commits.iter() {
       println!("Found commit {}: {}", commit.id, commit.message);
    }
}

fn process_gitlab_push(payload: Json<Value>) {
    let push: GitlabPush = serde_json::from_value(payload.0).unwrap();
    if push.total_commits_count > 20 {
        // We'll support this later. GitHub and Gitlab both don't include more
        // than 20 commits in a push event.
        // https://docs.gitlab.com/ee/user/project/integrations/webhooks.html#push-events
        println!("Ignoring push, because too many commits: {}", push.total_commits_count)
    }
    for commit in push.commits.iter() {
       println!("Found commit {}: {}", commit.id, commit.message) 
    }
}

#[get("/")]
fn home() -> String {
    "Hello, World!".to_string()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![home])
        .mount("/api/v1/hooks", routes![github_push])
        .mount("/api/v1/hooks", routes![gitlab_push])
        .launch();
}
