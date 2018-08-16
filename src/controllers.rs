use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Form;

use openssl::ssl::SslConnector;
use openssl::ssl::SslMethod;
use openssl::x509::X509;
use std::net::TcpStream;

#[derive(Deserialize)]
pub struct DomainForm {
    domain: String,
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
    
    // if certificate.is_some() {
    //     let cert = certificate.unwrap();
    //     let entries = cert.issuer_name().entries();
    //     entries.for_each(|entry| {

    //         println!("{} : {}", entry.object(), entry.data().as_utf8().unwrap());
    //     })
    // }



    let mut reg = Handlebars::new();
    // render without register
    println!(
        "{}",
        reg.render_template("Hello {{name}}", &json!({"name": "foo"}))?
    );

    // register template using given name
    reg.register_template_string("tpl_1", "Good afternoon, {{name}}")?
    println!("{}", reg.render("tpl_1", &json!({"name": "foo"}))?);


    let template = IndexTemplate { certificate: certificate }.render().unwrap();

    HttpResponse::Ok().body(template)
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