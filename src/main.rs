extern crate actix_web;
#[macro_use]
extern crate askama;

extern crate openssl;
extern crate openssl_probe;

use actix_web::server;
use actix_web::App;
use actix_web::HttpRequest;
use openssl::ssl::SslMethod;
use openssl::ssl::SslConnector;
use std::net::TcpStream;

use askama::Template; 

struct CertificateInformation {
    domain: String, 
    not_before: String, 
    not_after: String
}

#[derive(Template)] 
#[template(path = "index.html")]
struct HelloTemplate<> {
    certificate: Option<CertificateInformation>, 
}

fn get_certificate_info(domain: String) -> Option<CertificateInformation> {
    Some(CertificateInformation{
        domain: "ouistock.fr".to_string(), 
        not_after: "01/01/2017".to_string(), 
        not_before: "01/02/2017".to_string()
    })
}

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();

    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build(); 
    let stream = TcpStream::connect("www.ouistock.fr:443").unwrap();
    let connection = connector.connect("www.ouistock.fr", stream).unwrap();
      
    
    let ssl = connection.ssl();
    let certificate = ssl.peer_certificate().unwrap();

    println!("{}", certificate.not_before());
    println!("{}", certificate.not_after());

    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}