use crate::config::Config;
use openssl::rsa::Rsa;
use std::fs::{create_dir_all, File};
use std::io::Write;

mod config;

fn main() {
	env_logger::init();

	let config = Config::new().unwrap();
	let rsa = Rsa::generate(config.crypto.rsa_bits).unwrap();
	let private_key = rsa.private_key_to_pem().unwrap();
	let public_key = rsa.public_key_to_pem().unwrap();

	create_dir_all(config.path.private_key.parent().unwrap()).unwrap();
	File::create(config.path.private_key).unwrap().write_all(&private_key).unwrap();

	create_dir_all(config.path.public_key.parent().unwrap()).unwrap();
	File::create(config.path.public_key).unwrap().write_all(&public_key).unwrap();
}
