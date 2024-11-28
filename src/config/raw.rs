use serde::Deserialize;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
pub struct Config {
	pub path: path::Config,
	pub network: network::Config,
	pub crypto: crypto::Config,
}

pub mod path {
	use serde::Deserialize;

	#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
	pub struct Config {
		pub app: String,
		pub private_key: String,
		pub public_key: String,
	}
}

pub mod network {
	use serde::Deserialize;

	#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
	pub struct Config {
		pub port: u16,
		pub seed_nodes: Vec<String>,
		pub ping_interval: u64,
	}
}

pub mod crypto {
	use serde::Deserialize;

	#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
	pub struct Config {
		pub rsa_bits: u32,
	}
}
