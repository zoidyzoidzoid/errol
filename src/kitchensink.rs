extern crate actix_web;
use self::actix_web::{error, server, App, Error, HttpRequest, HttpResponse};
#[allow(unused_imports)]
use self::actix_web::{AsyncResponder, HttpMessage};
extern crate futures;
use self::futures::future;
use self::futures::Future;

extern crate serde;
extern crate serde_json;

use github::GitHubPushEvent;
use gitlab::GitlabPush;

// enum EventType {
//     Issue,
//     PullRequest,
//     Push,
//     Unsupported,
// }

// enum Provider {
//     GitHub,
//     Gitlab,
// }

// struct Event {
//     provider: Provider,
//     event_type: EventType,
// }

fn github_push(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let content_type = req.headers().get(http::header::CONTENT_TYPE).unwrap().to_str().unwrap();
    if content_type != "application/json" {
        return Box::new(future::err(error::ErrorBadRequest(
            "Incorrect content-type",
        )));
    }
    let event_type = req
        .headers()
        .get("X-GitHub-Event")
        .unwrap()
        .to_str()
        .unwrap();
    match event_type {
        // "issues" => {
        // },
        // "pull_request" => {
        // },
        "push" => req
            .json()
            .from_err()
            .and_then(|push: GitHubPushEvent| {
                println!("Received push: {:?}", push);
                process_github_push(push);
                Ok(HttpResponse::Ok().json("GitHub hook received!"))
            }).responder(),
        _ => Box::new(future::err(error::ErrorBadRequest(
            "Event type not supported yet.",
        ))),
    }
}

fn gitlab_push(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let content_type = req.headers().get(http::header::CONTENT_TYPE).unwrap().to_str().unwrap();
    if content_type != "application/json" {
        return Box::new(future::err(error::ErrorBadRequest(
            "Incorrect content-type",
        )));
    }
    let event_type = req
        .headers()
        .get("X-Gitlab-Event")
        .unwrap()
        .to_str()
        .unwrap();
    match event_type {
        "Push Hook" => req
            .json()
            .from_err()
            .and_then(|push: GitlabPush| {
                println!("Received push: {:?}", push);
                process_gitlab_push(push);
                Ok(HttpResponse::Ok().json("Gitlab hook received!"))
            }).responder(),
        _ => Box::new(future::err(error::ErrorBadRequest(
            "Event type not supported yet.",
        ))),
    }
}

fn process_github_push(push: GitHubPushEvent) {
    if push.commits.len() >= 20 {
        // We'll support this later. GitHub and Gitlab both don't include more
        // than 20 commits in a push event.
        // GitHub's Events API docs talk about .size and .distinct_size, but I
        // haven't got them in a payload yet.
        // https://developer.github.com/v3/activity/events/types/#pushevent
        println!(
            "There might have been too many many commits: {}",
            push.commits.len()
        );
    }
    for commit in push.commits.iter() {
        println!("Found commit {}: {}", commit.id, commit.message);
    }
}

fn process_gitlab_push(push: GitlabPush) {
    if push.total_commits_count > 20 {
        // We'll support this later. GitHub and Gitlab both don't include more
        // than 20 commits in a push event.
        // https://docs.gitlab.com/ee/user/project/integrations/webhooks.html#push-events
        println!(
            "Ignoring push, because too many commits: {}",
            push.total_commits_count
        )
    }
    for commit in push.commits.iter() {
        println!("Found commit {}: {}", commit.id, commit.message)
    }
}

fn index(req: &HttpRequest) -> HttpResponse {
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            return HttpResponse::Ok().into()
        }
     }
    HttpResponse::BadRequest().into()
}

pub fn launch_server() {
    let addr = "127.0.0.1:8080";
    println!("Starting Errol server on http://{}", addr);
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(index))
            .resource("/api/v1/hooks/github", |r| r.f(github_push))
            .resource("/api/v1/hooks/gitlab", |r| r.f(gitlab_push))
    }).bind(addr)
    .expect("Can not bind to port 8080")
    .run();
}

// #[cfg(test)]
// #[macro_use]
// extern crate quickcheck;
// #[cfg(test)]
// mod tests {
//   quickcheck! {
//       fn prop(xs: Vec<u32>) -> bool {
//           xs == reverse(&reverse(&xs))
//       }
//   }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use self::actix_web::test as actix_web_test;

    #[test]
    fn test_index_success() {
        let resp = actix_web_test::TestRequest::with_header("content-type", "text/plain")
            .run(&index)
            .unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[test]
    fn test_index_bad_request() {
        let resp = actix_web_test::TestRequest::default()
            .run(&index)
            .unwrap();
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }
}