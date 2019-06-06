use actix_web::Form;
use actix_web::HttpRequest;
use actix_web::HttpResponse;

use openssl::ssl::SslConnector;
use openssl::ssl::SslMethod;
use openssl::x509::X509;

use std::collections::HashMap;
use std::net::TcpStream;

use askama::Template;

#[derive(Deserialize)]
pub struct DomainForm {
    domain: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    certificate: Option<CertificateInformation>,
}

pub struct CertificateInformation {
    pub domain: String,
    pub not_before: String,
    pub not_after: String,
    pub issuer_name: HashMap<String, String>,
    pub subject_name: HashMap<String, String>,
    pub original: X509,
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
pub fn handle_post((_req, params): (HttpRequest, Form<DomainForm>)) -> HttpResponse {
    let certificate = get_certificate_info(params.domain.clone());
    let template = IndexTemplate {
        certificate: certificate,
    }
    .render()
    .unwrap();

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

    match certificate {
        Some(cert) => {
            let mut subject_name: HashMap<String, String> = HashMap::new();
            let mut issuer_name: HashMap<String, String> = HashMap::new();

            cert.subject_name().entries().for_each(|entry| {
                subject_name.insert(
                    entry.object().to_string(),
                    entry.data().as_utf8().unwrap().to_string(),
                );
            });

            cert.issuer_name().entries().for_each(|entry| {
                issuer_name.insert(
                    entry.object().to_string(),
                    entry.data().as_utf8().unwrap().to_string(),
                );
            });

            Some(CertificateInformation {
                domain: domain,
                not_before: format!("{}", cert.not_before()),
                not_after: format!("{}", cert.not_after()),
                issuer_name: issuer_name,
                subject_name: subject_name,
                original: cert,
            })
        }
        None => None,
    }
}
