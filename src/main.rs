mod handlers;
mod global_state;

extern crate actix_web;
extern crate dotenv;

use actix_web::{server, App};
use dotenv::dotenv;
use std::env;


fn main() {
    dotenv().ok();

    server::new(
        || App::with_state(global_state::State {})
            .prefix("/")
            .resource(
            "/",
            |r| r.f(handlers::index::index)
        )
    ).bind(
        env::var("SERVICE_URL").expect(
            "SERVICE_URL is a required env var"
        )
    ).unwrap().run();
}