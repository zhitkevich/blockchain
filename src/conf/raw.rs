use serde::Deserialize;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
pub struct Conf {
	pub path: path::Conf,
	pub network: network::Conf,
	pub crypto: crypto::Conf,
}

pub mod path {
	use serde::Deserialize;

	#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
	pub struct Conf {
		pub app: String,
		pub private_key: String,
		pub public_key: String,
	}
}

pub mod network {
	use serde::Deserialize;

	#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
	pub struct Conf {
		pub port: u16,
		pub seed_nodes: Vec<String>,
		pub ping_interval: u64,
	}
}

pub mod crypto {
	use serde::Deserialize;

	#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize)]
	pub struct Conf {
		pub rsa_bits: u32,
	}
}
