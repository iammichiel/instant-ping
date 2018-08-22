use openssl::x509::X509;

pub struct CertificateInformation {
    pub domain: String,
    pub not_before: String, 
    pub not_after: String, 
    pub issuer_name: String,
    pub subject_name: String, 
    pub original: X509
}