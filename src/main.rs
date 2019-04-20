#[macro_use]
extern crate clap;              // CLI arguments

use std::collections::HashMap;
use clap::App;
use rustc_serialize::base64::{ToBase64, STANDARD};
use openssl::rsa::{Rsa, Padding};


fn get_public_key(repo: &str) -> String {
    let url = format!("https://api.travis-ci.org/repos/{}/key", repo);
    let resp: HashMap<String, String> =
        reqwest::get(&url).unwrap().json().unwrap();
    resp["key"].clone()
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

    let mut result = vec![0; rsa.size() as usize];
    let _ = rsa.public_encrypt(data.as_bytes(),
                               &mut result,
                               Padding::PKCS1);
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
