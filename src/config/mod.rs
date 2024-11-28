pub use error::Error;
use std::path::PathBuf;
use std::{env, fs};

mod error;
mod raw;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Config {
	pub path: path::Config,
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

pub mod crypto {
	#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
	pub struct Config {
		pub rsa_bits: u32,
	}
}
