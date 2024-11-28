use crate::config::Config;
use crate::net::server;
use log::error;
use openssl::rsa::Rsa;
use std::env::args;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::process::exit;
use std::thread;

mod config;
mod net;

fn main() {
	env_logger::init();

	let config = Config::new().unwrap_or_else(|e| {
		error!("Failed to create config: {e}");
		exit(1);
	});

	if let Some("init") = args().nth(1).as_deref() {
		init(&config);
		return;
	}

	thread::spawn(move || {
		if let Err(e) = server::start(config.network.port) {
			error!("Failed to start server: {e}");
			exit(1);
		}
	});
	server::ping(config.network.seed_nodes, config.network.ping_interval);
}

fn init(config: &Config) {
	let rsa = Rsa::generate(config.crypto.rsa_bits).unwrap();

	let private_key = rsa.private_key_to_pem().unwrap();
	create_dir_all(config.path.private_key.parent().unwrap()).unwrap();
	File::create(&config.path.private_key).unwrap().write_all(&private_key).unwrap();

	let public_key = rsa.public_key_to_pem().unwrap();
	create_dir_all(config.path.public_key.parent().unwrap()).unwrap();
	File::create(&config.path.public_key).unwrap().write_all(&public_key).unwrap();
}
