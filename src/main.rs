#[macro_use]
extern crate clap;              // CLI arguments
extern crate hyper;             // HTTP/HTTPS client
extern crate serde;             // JSON
extern crate serde_json;        // JSON
extern crate rustc_serialize;   // base64
extern crate openssl;           // RSA
extern crate openssl_sys;       // RSA

use clap::App;
use hyper::client::Client;
use serde_json::Map;
use rustc_serialize::base64::{ToBase64, STANDARD};
use openssl::crypto::rsa::RSA;
use openssl_sys::RSA_public_encrypt;

use std::io::prelude::*;    // .read_to_string


fn get_public_key(repo: &str) -> String {
    let url = format!("https://api.travis-ci.org/repos/{}/key", repo);
    let client = Client::new();
    let mut result = "".to_string();
    client.get(&url)
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
    let rsa = RSA::public_key_from_pem(&mut public_key).unwrap();

    ////////////////////
    // Encryption
    ////////////////////

    // https://www.openssl.org/docs/manmaster/crypto/RSA_public_encrypt.html
    // https://sfackler.github.io/rust-openssl/doc/v0.7.14/openssl_sys/fn.RSA_public_encrypt.html
    const RSA_PKCS1_PADDING: i32 = 1;
    let mut result = vec![0; rsa.size().unwrap() as usize];
    unsafe {
        RSA_public_encrypt((&data).len() as i32,
                           data.as_ptr(),
                           result.as_mut_ptr(),
                           rsa.as_ptr(),
                           RSA_PKCS1_PADDING);
    }
    result.to_base64(STANDARD)
}


fn main() {

    ////////////////////
    // Parse Arguments
    ////////////////////

    let yml = load_yaml!("arguments.yml");
    let arguments = App::from_yaml(yml).get_matches();
    let repo = arguments.value_of("repo").unwrap();
    let data = arguments.value_of("data").unwrap();

    ////////////////////
    // Encrypt
    ////////////////////

    let encrypted = encrypt(repo, data);
    println!("secure: \"{}\"", encrypted);
}
