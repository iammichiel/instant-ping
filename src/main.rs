#[macro_use]
extern crate serde_derive;

use actix_web::HttpServer;
use actix_web::web;
use actix_web::middleware::Logger;
use actix_web::App;
use actix_files as fs;

use dotenv::dotenv;

// mod actors;
mod controllers;

fn main() -> std::io::Result<()> {
    openssl_probe::init_ssl_cert_env_vars();
    dotenv().ok();
    env_logger::init();

    let port = dotenv::var("PORT").expect("DATABASE_URL must be set");

    HttpServer::new(|| App::new()
        .wrap(Logger::default())
        .service(web::resource("/")
            .route(web::get().to(controllers::index))
            .route(web::post().to(controllers::handle_post))
        )
        // .service(fs::Files::new("/public", "web/dist"))
    )
    .bind(format!("0.0.0.0:{}", port))?
    .run()
}
