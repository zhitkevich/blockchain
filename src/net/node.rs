#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Node {
	pub addr: String,
}

impl Node {
	pub fn new(addr: &str) -> Self {
		Self { addr: addr.to_owned() }
	}
}
