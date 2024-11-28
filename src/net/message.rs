#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Message {
	#[default]
	Ping,
	Pong,
	Block(String),
}

impl Message {
	pub fn from_le_bytes<T: AsRef<[u8]>>(bytes: T) -> Option<Self> {
		let bytes = bytes.as_ref();
		let header = Header::new(bytes[0])?;
		Some(match header {
			Header::Ping => Self::Ping,
			Header::Pong => Self::Pong,
			Header::Block => Self::Block(String::from_utf8_lossy(&bytes[1..]).to_string()),
		})
	}

	pub fn to_le_bytes(&self) -> Vec<u8> {
		match self {
			Message::Ping => vec![Header::Ping as u8],
			Message::Pong => vec![Header::Pong as u8],
			Message::Block(block) => {
				let mut bytes = Vec::new();
				bytes.push(Header::Block as u8);
				bytes.extend_from_slice(block.as_bytes());
				bytes
			}
		}
	}
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Header {
	#[default]
	Ping = 1,
	Pong = 2,
	Block = 3,
}

impl Header {
	pub fn new(byte: u8) -> Option<Self> {
		match byte {
			1 => Some(Self::Ping),
			2 => Some(Self::Pong),
			3 => Some(Self::Block),
			_ => None,
		}
	}
}
