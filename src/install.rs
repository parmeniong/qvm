use crate::http::{HttpRequest, HttpMethod};
use semver::Version;

const QVM_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn install(version: Version) {
    println!("Installing version {}...", version);

    let mut request = HttpRequest::new(HttpMethod::Get, String::new());

    request.add_header("Host", "localhost:8080");
    request.add_header("User-Agent", format!("QVM/{}", QVM_VERSION).as_str());
    request.add_header("Accepts", "application/json");
    
    let response = request.send("127.0.0.1:8080".to_string());

    println!("{} {}\n\n{}", response.status_code, response.reason, response.body);
}