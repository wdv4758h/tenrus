#[macro_use]
extern crate clap;              // CLI arguments
extern crate hyper;             // HTTP client
extern crate hyper_native_tls;  // HTTPS implementation
extern crate serde;             // JSON
extern crate serde_json;        // JSON
extern crate rustc_serialize;   // base64
extern crate openssl;           // RSA

use clap::App;
use hyper::client::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json::Map;
use rustc_serialize::base64::{ToBase64, STANDARD};
use openssl::rsa::{Rsa, PKCS1_PADDING};

use std::io::prelude::*;    // .read_to_string


fn get_public_key(repo: &str) -> String {
    let url = format!("https://api.travis-ci.org/repos/{}/key", repo);

    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    let mut result = "".to_string();
    let _ = client.get(&url)
                  .send().unwrap()
                  .read_to_string(&mut result);
    let map: Map<String, String> =
        serde_json::from_str(result.as_str()).unwrap();
    map["key"].clone()
}

fn encrypt(repo: &str, data: &str) -> String {
    let public_key = get_public_key(repo);

    ////////////////////
    // LOAD RSA Public Key
    ////////////////////

    let mut public_key = public_key.as_bytes();
    let rsa = Rsa::public_key_from_pem(&mut public_key).unwrap();

    ////////////////////
    // Encryption
    ////////////////////

    let mut result = vec![0; rsa.size()];
    let _ = rsa.public_encrypt(data.as_bytes(),
                               &mut result,
                               PKCS1_PADDING);
    result.to_base64(STANDARD)
}


fn main() {

    ////////////////////
    // Parse Arguments
    ////////////////////

    let yml = load_yaml!("cli.yml");
    let arguments = App::from_yaml(yml).get_matches();
    let repo = arguments.value_of("repo").unwrap();
    let data = arguments.value_of("data").unwrap();

    ////////////////////
    // Encrypt
    ////////////////////

    let encrypted = encrypt(repo, data);
    println!("secure: \"{}\"", encrypted);
}
