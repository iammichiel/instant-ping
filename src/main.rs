extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate askama;
extern crate futures;

extern crate openssl;
extern crate openssl_probe;

extern crate dotenv;
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;

use actix_web::App;
use actix_web::fs;
use actix_web::http;
use actix_web::middleware;
use actix_web::server;

use dotenv::dotenv;

pub mod actors;
pub mod controllers;

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    dotenv().ok();
    env_logger::init();

    server::new(|| 
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/", |r| {
                r.method(http::Method::GET).f(controllers::index);
                r.method(http::Method::POST).with(controllers::handle_post);
            })
            .handler("/public", fs::StaticFiles::new("web/dist").unwrap())
        )
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();

    // Start the async actors
    // actors::start_system();
}