use std::env::VarError;
use std::fmt::{Display, Formatter};
use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
	Io(io::Error),
	TomlParse(toml::de::Error),
	EnvVar(VarError),
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Io(e) => write!(f, "{e}"),
			Self::TomlParse(e) => write!(f, "{e}"),
			Self::EnvVar(e) => write!(f, "{e}"),
		}
	}
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
	fn from(e: io::Error) -> Self {
		Error::Io(e)
	}
}

impl From<toml::de::Error> for Error {
	fn from(e: toml::de::Error) -> Self {
		Error::TomlParse(e)
	}
}

impl From<VarError> for Error {
	fn from(e: VarError) -> Self {
		Error::EnvVar(e)
	}
}
