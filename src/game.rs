use crate::{board::*, error::*, piece::*};
use colored::Colorize;
use std::fmt;

fn handicap_stones() -> [Vec<(u8, u8)>; 9] {
	[
		vec![(3, 3)],
		vec![(3, 3), (15, 15)],
		vec![(3, 3), (15, 15), (3, 15)],
		vec![(3, 3), (15, 15), (3, 15), (15, 3)],
		vec![(3, 3), (15, 15), (3, 15), (15, 3), (9, 9)],
		vec![(3, 3), (15, 15), (3, 15), (15, 3), (3, 9), (15, 9)],
		vec![(3, 3), (15, 15), (3, 15), (15, 3), (9, 9), (3, 9), (15, 9)],
		vec![
			(3, 3),
			(15, 15),
			(3, 15),
			(15, 3),
			(3, 9),
			(15, 9),
			(9, 3),
			(9, 15),
		],
		vec![
			(3, 3),
			(15, 15),
			(3, 15),
			(15, 3),
			(3, 9),
			(15, 9),
			(9, 3),
			(9, 15),
			(9, 9),
		],
	]
}

pub struct Game {
	pub board: Board,
	pub moves: Vec<PlacedPiece>,
	pub handicap: u8,
}

impl Game {
	pub fn new(handicap: u8) -> Self {
		let mut board = Board::new();
		if handicap > 0 {
			let handicap_stones = &handicap_stones()[handicap as usize - 1];

			for pos in handicap_stones {
				board.set(PlacedPiece {
					piece: Piece {
						color: PieceColor::Black,
					},
					pos: *pos,
				});
			}
		}
		Self {
			board,
			moves: vec![],
			handicap,
		}
	}

	pub fn do_move(&mut self, m: PlacedPiece) -> Result<MoveResult> {
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
			PieceColor::Black
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
