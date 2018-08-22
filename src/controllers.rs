use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Form;

use openssl::ssl::SslConnector;
use openssl::ssl::SslMethod;

use std::net::TcpStream;

use askama::Template;
use models::CertificateInformation;

#[derive(Deserialize)]
pub struct DomainForm {
    domain: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    certificate: Option<CertificateInformation>
}

/**
 * Show the index page. 
 */
pub fn index(_req: &HttpRequest) -> HttpResponse {
    let template = IndexTemplate { certificate: None }.render().unwrap();

    HttpResponse::Ok().body(template)
}

/**
 * Get the requested domain from the form and run it through the certificate checker.
 */
pub fn handle_post((_req, params): (HttpRequest, Form<DomainForm>),) -> HttpResponse {
    let certificate = get_certificate_info(params.domain.clone());
    let template = IndexTemplate { certificate: certificate }.render().unwrap();

    HttpResponse::Ok().body(template)
}

/**
 * Try to establish a connection to the remote domain
 */
fn get_certificate_info(domain: String) -> Option<CertificateInformation> {
    let formatted_domain = format!("{}:443", domain);
    // TODO All unwrap should be handled. 
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build(); 
    let stream = TcpStream::connect(formatted_domain).unwrap();
    let connection = connector.connect(&domain, stream).unwrap();
    
    let ssl = connection.ssl();

    let certificate = ssl.peer_certificate();

    let t = certificate.clone().unwrap();
    t.subject_name().entries().for_each(|entry| {
        println!("Subject : {} : {}", entry.object(), entry.data().as_utf8().unwrap());
    });

    t.issuer_name().entries().for_each(|entry| {
        println!("Issuer : {} : {}", entry.object(), entry.data().as_utf8().unwrap());
    });

    match certificate {
        Some(cert) => {
            Some(CertificateInformation {
                domain: domain,
                not_before: format!("{}", cert.not_before()), 
                not_after: format!("{}", cert.not_after()), 
                issuer_name: String::from("QWD"), 
                subject_name: String::from("wqd"), 
                original: cert
            })  
        }, 
        None => None
    }
    
}