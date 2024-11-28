use crate::net::Node;
pub use error::Error;
use std::path::PathBuf;
use std::time::Duration;
use std::{env, fs};

mod error;
mod raw;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Config {
	pub path: path::Config,
	pub network: network::Config,
	pub crypto: crypto::Config,
}

impl Config {
	pub fn new() -> Result<Self, Error> {
		let raw_config: raw::Config = toml::from_str(&fs::read_to_string("config.toml")?)?;

		let home = if cfg!(windows) { env::var("USERPROFILE")? } else { env::var("HOME")? };
		let app = PathBuf::from(&home).join(&raw_config.path.app);
		let private_key = app.join(&raw_config.path.private_key);
		let public_key = app.join(&raw_config.path.public_key);

		Ok(Self {
			path: path::Config { app, private_key, public_key },
			network: network::Config {
				port: raw_config.network.port,
				seed_nodes: raw_config.network.seed_nodes.iter().map(|n| Node::new(n)).collect(),
				ping_interval: Duration::from_millis(raw_config.network.ping_interval),
			},
			crypto: crypto::Config { rsa_bits: raw_config.crypto.rsa_bits },
		})
	}
}

pub mod path {
	use std::path::PathBuf;

	#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
	pub struct Config {
		pub app: PathBuf,
		pub private_key: PathBuf,
		pub public_key: PathBuf,
	}
}

pub mod network {
	use crate::net::Node;
	use std::time::Duration;

	#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
	pub struct Config {
		pub port: u16,
		pub seed_nodes: Vec<Node>,
		pub ping_interval: Duration,
	}
}

pub mod crypto {
	#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
	pub struct Config {
		pub rsa_bits: u32,
	}
}
