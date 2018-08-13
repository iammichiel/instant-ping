extern crate actix_web;
#[macro_use]
extern crate askama;

extern crate openssl;
extern crate openssl_probe;

extern crate dotenv;
extern crate log;
extern crate env_logger;

use actix_web::server;
use actix_web::App;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Form;
use actix_web::http;
use actix_web::middleware;

use dotenv::dotenv;

use openssl::ssl::SslMethod;
use openssl::ssl::SslConnector;

use std::net::TcpStream;

use askama::Template; 

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
struct DomainForm {
    domain: String,
}

struct CertificateInformation {
    domain: String, 
    not_before: String, 
    not_after: String
}

#[derive(Template)] 
#[template(path = "index.html")]
struct IndexTemplate<> {
    certificate: Option<CertificateInformation>, 
}

fn get_certificate_info(domain: String) -> Option<CertificateInformation> {
    let formatted_domain = format!("{}:443", domain);
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build(); 
    let stream = TcpStream::connect(formatted_domain).unwrap();
    let connection = connector.connect(&domain, stream).unwrap();
    
    let ssl = connection.ssl();
    let certificate = ssl.peer_certificate().unwrap();

    println!("{}", certificate.not_before());
    println!("{}", certificate.not_after());

    Some(CertificateInformation{
        domain: domain, 
        not_after: certificate.not_after().to_string(), 
        not_before: certificate.not_before().to_string()
    })
}

fn index(_req: &HttpRequest) -> HttpResponse {
    let template = IndexTemplate { certificate: None,}.render().unwrap();

    HttpResponse::Ok().body(template)
}

fn handle_post((_req, params): (HttpRequest, Form<DomainForm>),) -> HttpResponse {
    let certificate = get_certificate_info(params.domain.clone());
    let template = IndexTemplate { certificate: certificate }.render().unwrap();

    HttpResponse::Ok().body(template)
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    dotenv().ok();
    env_logger::init();
 
    server::new(|| 
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/", |r| {
                r.method(http::Method::GET).f(index);
                r.method(http::Method::POST).with(handle_post);
            })
        )
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}