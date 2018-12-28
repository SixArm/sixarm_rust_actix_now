extern crate chrono;
use chrono::prelude::*;

extern crate actix_web;
use actix_web::{server, App, HttpRequest};

fn now() -> String {
  let f = "%Y-%m-%dT%H:%M:%S.%fZ";
  let d: DateTime<Utc> = Utc::now();
  let s: String = d.format(f).to_string();
  s
}

fn index(_req: &HttpRequest) -> String {
  now()
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8000")
        .unwrap()
        .run();
}