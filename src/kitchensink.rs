use std::collections::HashMap;

extern crate actix;
extern crate actix_web;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate sentry;
extern crate serde;
extern crate serde_json;
extern crate tera;

use self::actix_web::{error, http, server, App, Error, HttpRequest, HttpResponse, Query, State};
#[allow(unused_imports)] use self::actix_web::{AsyncResponder, HttpMessage};
use self::actix_web::middleware::Logger;
use self::futures::{future, Future};
use self::tera::compile_templates;
use self::dotenv::dotenv;

use github::GitHubPushEvent;
use gitlab::GitlabPush;

fn github_push(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
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
        "issues" => {
            Box::new(future::err(error::ErrorBadRequest(
                "Issues events not supported yet.",
            )))
        },
        "pull_request" => {
            Box::new(future::err(error::ErrorBadRequest(
                "Pull Request events not supported yet.",
            )))
        },
        "push" => req
            .json()
            .from_err()
            .and_then(|push: GitHubPushEvent| {
                println!("Received push to {:?} at {:?}", push.repository.full_name, push.git_hub_push_event_ref);
                process_github_push(push);
                Ok(HttpResponse::Ok().json("GitHub hook received!"))
            }).responder(),
        _ => Box::new(future::err(error::ErrorBadRequest(
            "Event type not supported yet.",
        ))),
    }
}

fn gitlab_push(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
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
                println!("Received push to {:?} at {:?}", push.project.path_with_namespace, push.git_lab_push_ref);
                // println!("Received push: {:?}", push);
                process_gitlab_push(push);
                Ok(HttpResponse::Ok().json("Gitlab hook received!"))
            }).responder(),
        _ => Box::new(future::err(error::ErrorBadRequest(
            "Event type not supported yet.",
        ))),
    }
}

fn process_github_push(push: GitHubPushEvent) {
    // println!("{}", serde_json::to_string(&push).unwrap());
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
    // println!("{}", serde_json::to_string(&push).unwrap());
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

struct AppState {
    template: tera::Tera, // <- store tera template in application state
}

fn index(state: State<AppState>, query: Query<HashMap<String, String>>) -> Result<HttpResponse, Error> {
    let s = if let Some(name) = query.get("name") {
        // <- submitted form
        let mut ctx = tera::Context::new();
        ctx.insert("name", &name.to_owned());
        ctx.insert("text", &"Welcome!".to_owned());
        state
            .template
            .render("user.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    } else {
        state
            .template
            .render("index.html", &tera::Context::new())
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// fn index(req: &HttpRequest) -> HttpResponse {
//     if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
//         if let Ok(_s) = hdr.to_str() {
//             return HttpResponse::Ok().into()
//         }
//      }
//     HttpResponse::BadRequest().into()
// }

fn create_app() -> App<AppState> {
    let tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));

    App::with_state(AppState{template: tera})
        .middleware(Logger::default())
        .resource("/", |r| r.method(http::Method::GET).with(index))
        .resource("/api/v1/hooks/github", |r| r.f(github_push))
        .resource("/api/v1/hooks/gitlab", |r| r.f(gitlab_push))
}

pub fn launch_server() {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let _guard = sentry::init("https://dd8228af920c4021a163fc65919f72cf@sentry.io/1318694");

    let sys = actix::System::new("errol");

    let addr = "127.0.0.1:8080";

    server::new(|| {
            create_app()
        }).bind(addr)
        .expect("Can not bind to port 8080")
        .start();

    println!("Started http server: http://{}", addr);
    let _ = sys.run();
}

// #[cfg(test)]
// #[macro_use]
// extern crate quickcheck;
// quickcheck! {
//     fn prop(xs: Vec<u32>) -> bool {
//         xs == reverse(&reverse(&xs))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use self::actix_web::test as actix_web_test;

    #[test]
    fn test_index_success() {
        let mut srv = actix_web_test::TestServer::with_factory(create_app);

        let request = srv.client(
            http::Method::GET, "/").finish().unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert!(response.status().is_success());
    }

    #[test]
    fn test_index_bad_request() {
        let mut srv = actix_web_test::TestServer::with_factory(create_app);

        let request = srv.client(
            http::Method::GET, "/").finish().unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert!(response.status().is_success());
    }
}
