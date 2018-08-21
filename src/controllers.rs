use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Form;

use openssl::ssl::SslConnector;
use openssl::ssl::SslMethod;
use openssl::x509::X509;

use std::net::TcpStream;

use handlebars::Handlebars;

#[derive(Deserialize)]
pub struct DomainForm {
    domain: String,
}

/**
 * Show the index page. 
 */
pub fn index(_req: &HttpRequest) -> HttpResponse {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("index", "./templates/index.hbs");


    let content = handlebars.render("index", &json!({})).unwrap();



    // let template = IndexTemplate { certificate: None }.render().unwrap();

    HttpResponse::Ok().body(content)
}

/**
 * Get the requested domain from the form and run it through the certificate checker.
 */
pub fn handle_post((_req, params): (HttpRequest, Form<DomainForm>),) -> HttpResponse {
    let certificate = get_certificate_info(params.domain.clone());
    // let template = IndexTemplate { certificate: certificate }.render().unwrap();

    HttpResponse::Ok().body("OK")
}

/**
 * Try to establish a connection to the remote domain
 */
fn get_certificate_info(domain: String) -> Option<X509> {
    let formatted_domain = format!("{}:443", domain);
    // TODO All unwrap should be handled. 
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build(); 
    let stream = TcpStream::connect(formatted_domain).unwrap();
    let connection = connector.connect(&domain, stream).unwrap();
    
    let ssl = connection.ssl();

    ssl.peer_certificate()
}