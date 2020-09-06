use crate::error::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Pos {
	pub x: u8,
	pub y: u8,
}

impl Pos {
	pub fn new(x: u8, y: u8) -> Result<Self> {
		let p = Self { x, y };
		if x > 18 || y > 18 {
			Err(BadukError::PosOutOfBounds { pos: p })
		} else {
			Ok(p)
		}
	}
}

impl std::convert::TryFrom<(u8, u8)> for Pos {
	type Error = BadukError;

	fn try_from(value: (u8, u8)) -> Result<Self> {
		Self::new(value.0, value.1)
	}
}
