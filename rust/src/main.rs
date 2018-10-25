#![feature(proc_macro_hygiene, decl_macro, custom_attribute, plugin)]
#![plugin(rocket_codegen)]
// Needed for serde_derivce and rocket
extern crate serde;
extern crate serde_json;
#[macro_use] 
extern crate serde_derive;

extern crate rocket;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

extern crate rocket_contrib;
use rocket_contrib::Json;

mod gitlab;
use gitlab::models::GitlabPush;

struct Event {
    event_type: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Event {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Event, ()> {
        let events: Vec<_> = request.headers().get("X-Gitlab-Event").collect();
        if events.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let event = events[0];
        if event != "Push Hook" {
            return Outcome::Forward(());
        }

        return Outcome::Success(Event { event_type: event.to_string() } );
    }
}

#[post("/gitlab", format="application/json", data="<push>")]
fn gitlab_push(event: Event, push: Json<GitlabPush>) -> String {
    if event.event_type != "Push Hook" {
        return "Invalid Request".to_string();
    }

    println!("Received push: {:?}", push);
    process_gitlab_event(push);
    "Hello, World!".to_string()
}

fn process_gitlab_event(push: Json<GitlabPush>) {
    if push.total_commits_count > 20 {
        // We'll support this later
        println!("Ignoring push, because too many commits: {}", push.total_commits_count)
    }
    for commit in push.commits.iter() {
       println!("Found commit {}: {}", commit.id, commit.message) 
    }
}

fn main() {
    rocket::ignite().mount("/api/v1/push", routes![gitlab_push]).launch();
}
