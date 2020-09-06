use crate::{
	piece::*,
	pos::*,
};
use snafu::{
	ResultExt,
	Snafu,
};

#[derive(Debug, Snafu)]
pub enum BadukError {
	#[snafu(display("Invalid move: {}", source))]
	InvalidMove { source: InvalidMoveError },
	#[snafu(display("Position: {:?} is out of bounds", pos))]
	PosOutOfBounds { pos: Pos },
	#[snafu(display(
		"Index out of bounds: game is only {} moves long",
		history_size
	))]
	MoveIndexOutOfBounds { history_size: usize },
}

#[derive(Debug, Snafu)]
pub enum InvalidMoveError {
	#[snafu(display("position already occupied"))]
	AlreadyOccupied,
	#[snafu(display("move would result in self-capture"))]
	SelfCapture,
	#[snafu(display("invalid Ko"))]
	Ko,
	#[snafu(display("it is {}'s turn", turn))]
	NotYourTurn { turn: PieceColor },
}

pub type Result<T> = std::result::Result<T, BadukError>;
