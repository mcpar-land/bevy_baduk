use colored::Colorize;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PieceColor {
	Black,
	White,
}

impl fmt::Display for PieceColor {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			PieceColor::Black => write!(f, "Black"),
			PieceColor::White => write!(f, "White"),
		}
	}
}

impl PieceColor {
	pub fn opposite(&self) -> Self {
		match self {
			PieceColor::Black => PieceColor::White,
			PieceColor::White => PieceColor::Black,
		}
	}
}

pub use PieceColor::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Piece {
	pub color: PieceColor,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct PlacedPiece {
	pub piece: Piece,
	pub pos: (u8, u8),
}

impl PlacedPiece {
	pub fn new(color: PieceColor, pos: (u8, u8)) -> Self {
		Self {
			piece: Piece { color },
			pos,
		}
	}
}

impl From<PlacedPieceRef<'_>> for PlacedPiece {
	fn from(r: PlacedPieceRef) -> Self {
		PlacedPiece {
			piece: *r.piece,
			pos: r.pos,
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct PlacedPieceRef<'a> {
	pub piece: &'a Piece,
	pub pos: (u8, u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PieceAdjacency<'a> {
	Piece(PlacedPieceRef<'a>),
	Empty((u8, u8)),
	Edge,
}

impl<'a> PieceAdjacency<'a> {
	pub fn piece(&self) -> Option<PlacedPieceRef<'a>> {
		match self {
			Self::Piece(p) => Some(p.clone()),
			_ => None,
		}
	}
	pub fn as_type(self) -> PieceAdjacencyType {
		PieceAdjacencyType::from(self)
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PieceAdjacencyType {
	Piece,
	Empty,
	Edge,
}

impl From<PieceAdjacency<'_>> for PieceAdjacencyType {
	fn from(pa: PieceAdjacency) -> Self {
		match pa {
			PieceAdjacency::Piece(_) => Self::Piece,
			PieceAdjacency::Empty(_) => Self::Empty,
			PieceAdjacency::Edge => Self::Edge,
		}
	}
}
