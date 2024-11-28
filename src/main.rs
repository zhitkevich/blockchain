use crate::conf::Conf;
use crate::net::server;
use log::error;
use openssl::rsa::Rsa;
use std::env::args;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::process::exit;
use std::thread;

mod conf;
mod net;

fn main() {
	env_logger::init();

	let conf = Conf::new().unwrap_or_else(|e| {
		error!("Failed to create config: {e}");
		exit(1);
	});

	if let Some("init") = args().nth(1).as_deref() {
		init(&conf);
		return;
	}

	thread::spawn(move || {
		if let Err(e) = server::start(conf.network.port) {
			error!("Failed to start server: {e}");
			exit(1);
		}
	});
	server::ping(conf.network.seed_nodes, conf.network.ping_interval);
}

fn init(conf: &Conf) {
	let rsa = Rsa::generate(conf.crypto.rsa_bits).unwrap();

	let private_key = rsa.private_key_to_pem().unwrap();
	create_dir_all(conf.path.private_key.parent().unwrap()).unwrap();
	File::create(&conf.path.private_key).unwrap().write_all(&private_key).unwrap();

	let public_key = rsa.public_key_to_pem().unwrap();
	create_dir_all(conf.path.public_key.parent().unwrap()).unwrap();
	File::create(&conf.path.public_key).unwrap().write_all(&public_key).unwrap();
}
