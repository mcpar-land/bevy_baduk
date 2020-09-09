use crate::{
	adjacency::*,
	error::*,
	piece::*,
	pos::Pos,
};
use colored::Colorize;
use std::{
	collections::HashSet,
	fmt,
};

const DOT_POSITIONS: [(u8, u8); 9] = [
	(3, 3),
	(3, 9),
	(3, 15),
	(9, 3),
	(9, 9),
	(9, 15),
	(15, 3),
	(15, 9),
	(15, 15),
];

const BLACK_CHAR: &'static str = "○";
const WHITE_CHAR: &'static str = "●";

const EMPTY_CHAR: &'static str = "┼";
const DOT_CHAR: &'static str = "+";

const GRID_TOP_RIGHT: &'static str = "┐";
const GRID_TOP_LEFT: &'static str = "┌";
const GRID_BOT_RIGHT: &'static str = "└";
const GRID_BOT_LEFT: &'static str = "┘";

const GRID_TOP: &'static str = "┬";
const GRID_BOT: &'static str = "┴";
const GRID_LEFT: &'static str = "├";
const GRID_RIGHT: &'static str = "┤";

pub fn pos_in_bounds(pos: (u8, u8)) -> bool {
	pos.0 <= 18 && pos.1 <= 18
}

pub type MoveResult = (PlacedPiece, HashSet<(u8, u8)>);

#[derive(Clone)]
pub struct Board {
	display_board: [[Option<Piece>; 19]; 19],
	// for checking Ko
	ko_board: [[Option<Piece>; 19]; 19],
}

impl Board {
	pub fn new() -> Self {
		Self {
			display_board: [[None; 19]; 19],
			ko_board: [[None; 19]; 19],
		}
	}

	pub fn get(&self, pos: (u8, u8)) -> Option<PlacedPieceRef> {
		match self.display_board[pos.0 as usize][pos.1 as usize].as_ref() {
			Some(piece) => Some(PlacedPieceRef { piece, pos }),
			None => None,
		}
	}

	pub fn get_color(&self, pos: (u8, u8)) -> Option<PieceColor> {
		self.get(pos).map(|p| p.piece.color)
	}

	pub fn adjacents(&self, pos: (u8, u8)) -> Adjacency<PieceAdjacency> {
		fn offset(s: &Board, pos: (u8, u8), offset: (i8, i8)) -> PieceAdjacency {
			let newpos = (pos.0 as i8 + offset.0, pos.1 as i8 + offset.1);
			if newpos.0 < 0 || newpos.0 > 18 || newpos.1 < 0 || newpos.1 > 18 {
				return PieceAdjacency::Edge;
			};
			let newpos = (newpos.0 as u8, newpos.1 as u8);

			match s.display_board[newpos.0 as usize][newpos.1 as usize].as_ref() {
				Some(piece) => PieceAdjacency::Piece(PlacedPieceRef {
					piece,
					pos: (newpos.0 as u8, newpos.1 as u8),
				}),
				None => PieceAdjacency::Empty(newpos),
			}
		}
		Adjacency::new(
			offset(self, pos, (0, 1)),
			offset(self, pos, (0, -1)),
			offset(self, pos, (-1, 0)),
			offset(self, pos, (1, 0)),
		)
	}

	pub fn shape<'a>(&'a self, pos: (u8, u8)) -> HashSet<PlacedPieceRef> {
		let mut s: HashSet<PlacedPieceRef> = HashSet::new();
		let root = self.get(pos);
		if root.is_none() {
			return s;
		}
		let root = root.unwrap();

		fn shape_r<'a>(
			b: &'a Board,
			cur: PlacedPieceRef,
			s: &mut HashSet<PlacedPieceRef<'a>>,
		) {
			for p in b.adjacents(cur.pos).iter() {
				if !p.piece().is_none() {
					let piece_ref = p.piece().unwrap();
					if piece_ref.piece.color == cur.piece.color && !s.contains(&piece_ref)
					{
						s.insert(piece_ref);
						shape_r(b, piece_ref, s);
					}
				}
			}
		}

		shape_r(self, root, &mut s);
		s.insert(root);

		s
	}

	pub fn liberties(&self, pos: (u8, u8)) -> HashSet<(u8, u8)> {
		let mut libs: HashSet<(u8, u8)> = HashSet::new();
		for adj_pos in self.adjacents(pos).iter() {
			if let PieceAdjacency::Empty(pos) = adj_pos {
				libs.insert(*pos);
			}
		}
		libs
	}

	/// Get liberties for shape
	pub fn liberties_shape(&self, pos: (u8, u8)) -> HashSet<(u8, u8)> {
		let mut libs: HashSet<(u8, u8)> = HashSet::new();
		let shape = self.shape(pos);
		if self.get_color(pos).is_none() {
			return libs;
		}

		for p in shape.iter() {
			libs.extend(self.liberties(p.pos));
		}

		libs
	}

	pub fn set(&mut self, m: PlacedPiece) -> PlacedPieceRef {
		self.display_board[m.pos.0 as usize][m.pos.1 as usize] = Some(m.piece);
		self.get(m.pos).unwrap()
	}

	pub fn remove(&mut self, pos: (u8, u8)) {
		self.display_board[pos.0 as usize][pos.1 as usize] = None;
	}

	pub fn remove_shape(&mut self, pos: (u8, u8)) -> HashSet<(u8, u8)> {
		let shape = self.shape(pos);
		let positions: HashSet<(u8, u8)> =
			{ shape.into_iter().map(|p| p.pos).collect() };
		for p in positions.clone() {
			self.remove(p);
		}
		positions
	}

	pub fn valid_move(&self, m: PlacedPiece) -> Result<()> {
		if self.get_color(m.pos).is_some() {
			return Err(BadukError::InvalidMove {
				source: InvalidMoveError::AlreadyOccupied,
			});
		}

		let mut future_board = self.clone();
		future_board.set(m);

		// allow for self-captures if they would immediately capture something.

		let adj: Vec<(u8, u8)> = future_board
			.adjacents(m.pos)
			.iter()
			.filter(|a| {
				if let Some(p) = a.piece() {
					p.piece.color != m.piece.color
				} else {
					false
				}
			})
			.map(|a| {
				if let PieceAdjacency::Piece(p) = a {
					p.pos
				} else {
					panic!()
				}
			})
			.collect();
		for pos in adj {
			let has_no_liberties = future_board.liberties_shape(pos).len() == 0;
			if has_no_liberties {
				future_board.remove_shape(pos);
			}
		}

		if future_board.display_board == self.ko_board {
			return Err(BadukError::InvalidMove {
				source: InvalidMoveError::Ko,
			});
		}

		let future_libs = future_board.liberties_shape(m.pos);
		// if it would result in a self capture
		if future_libs.len() == 0 {
			return Err(BadukError::InvalidMove {
				source: InvalidMoveError::SelfCapture,
			});
		}

		Ok(())
	}

	pub fn do_move(&mut self, m: PlacedPiece) -> Result<MoveResult> {
		self.valid_move(m)?;
		self.ko_board = self.display_board;
		self.set(m);

		let adjacents = self
			.adjacents(m.pos)
			.map(|m| m.piece().map(|p| PlacedPiece::from(p)));

		let mut removed_positions: HashSet<(u8, u8)> = HashSet::new();

		for adj in adjacents.iter() {
			if let Some(pr) = adj {
				if pr.piece.color != m.piece.color {
					if self.liberties_shape(pr.pos).len() == 0 {
						removed_positions.extend(self.remove_shape(pr.pos));
					}
				}
			}
		}

		Ok((self.get(m.pos).unwrap().into(), removed_positions))
	}

	pub fn do_moves(&mut self, moves: Vec<PlacedPiece>) -> Result<()> {
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

	pub fn num_pieces(&self, color: PieceColor) -> u16 {
		let mut count: u16 = 0;
		for i in 0..18u8 {
			for j in 0..18u8 {
				if let Some(c) = self.get_color((j, i)) {
					if c == color {
						count += 1;
					}
				}
			}
		}
		count
	}
	pub fn num_pieces_all(&self) -> u16 {
		let mut count: u16 = 0;
		for i in 0..18u8 {
			for j in 0..18u8 {
				if self.get_color((j, i)).is_some() {
					count += 1;
				}
			}
		}
		count
	}

	pub fn all_pieces(&self) -> Vec<PlacedPieceRef> {
		let mut pieces: Vec<PlacedPieceRef> = vec![];
		for (i, s) in self.display_board.iter().enumerate() {
			for (j, p) in s.into_iter().enumerate() {
				if let Some(piece) = p {
					pieces.push(PlacedPieceRef {
						piece,
						pos: (i as u8, j as u8),
					});
				}
			}
		}
		pieces
	}
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut s: String = String::new();
		s.push_str("\n");
		for i in 0..19u8 {
			for j in 0..19u8 {
				let pos = (j, 18 - i);
				s = format!(
					"{}{}",
					s,
					match self.get_color(pos) {
						Some(color) => match color {
							Black => BLACK_CHAR.bold(),
							White => WHITE_CHAR.bold(),
						},
						None => if DOT_POSITIONS.contains(&pos) {
							DOT_CHAR
						} else {
							if pos.1 == 0 && pos.0 == 0 {
								GRID_BOT_RIGHT
							} else if pos.1 == 0 && pos.0 == 18 {
								GRID_BOT_LEFT
							} else if pos.1 == 18 && pos.0 == 0 {
								GRID_TOP_LEFT
							} else if pos.1 == 18 && pos.0 == 18 {
								GRID_TOP_RIGHT
							} else if pos.1 == 0 {
								GRID_BOT
							} else if pos.1 == 18 {
								GRID_TOP
							} else if pos.0 == 0 {
								GRID_LEFT
							} else if pos.0 == 18 {
								GRID_RIGHT
							} else {
								EMPTY_CHAR
							}
						}
						.bright_black(),
					}
				);

				if pos.0 != 18 {
					s = format!("{}{}", s, "─".bright_black())
				}
			}
			s.push_str("\n");
		}
		write!(f, "{}", s)
	}
}
