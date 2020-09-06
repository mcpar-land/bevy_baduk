use crate::{
	board::*,
	error::*,
	piece::*,
};
use colored::Colorize;
use std::fmt;

pub struct Game {
	pub board: Board,
	pub moves: Vec<PlacedPiece>,
	pub first: PieceColor,
}

impl Game {
	pub fn new(first: PieceColor) -> Self {
		Self {
			board: Board::new(),
			moves: vec![],
			first,
		}
	}

	pub fn do_move(&mut self, m: PlacedPiece) -> Result<PlacedPieceRef> {
		if self.current_turn() != m.piece.color {
			return Err(BadukError::InvalidMove {
				source: InvalidMoveError::NotYourTurn {
					turn: self.current_turn(),
				},
			});
		}
		let res = self.board.do_move(m)?;
		self.moves.push(m);
		Ok(res)
	}
	fn do_moves(&mut self, moves: Vec<PlacedPiece>) -> Result<()> {
		for m in moves {
			self.do_move(m)?;
		}
		Ok(())
	}
	pub fn do_moves_builder(
		&mut self,
		moves: Vec<(PieceColor, u8, u8)>,
	) -> Result<()> {
		for (color, x, y) in moves {
			self.do_move(PlacedPiece {
				piece: Piece { color },
				pos: (x, y),
			})?;
		}
		Ok(())
	}

	/// get the historical state of the board at a specific move
	/// (Starts at 1, not at 0)
	pub fn get_board_at_move(&self, i: usize) -> Result<Board> {
		if i >= self.moves.len() {
			return Err(BadukError::MoveIndexOutOfBounds {
				history_size: self.moves.len(),
			});
		}
		let mut board = Board::new();
		let moves_slice = self.moves.split_at(i).0;
		for m in moves_slice {
			board.do_move(m.clone())?;
		}
		Ok(board)
	}
	pub fn current_turn(&self) -> PieceColor {
		if self.moves.len() == 0 {
			self.first
		} else {
			self.moves.last().unwrap().piece.color.opposite()
		}
	}
	pub fn len(&self) -> usize {
		self.moves.len()
	}
}

impl fmt::Display for Game {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let info =
			format!("Move {} - {}'s turn", self.moves.len(), self.current_turn());
		write!(f, "{}{}", self.board, info.italic().bright_black())
	}
}
