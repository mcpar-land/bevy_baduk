use crate::{
	adjacency::*,
	board::*,
	error::*,
	piece::*,
};

#[test]
fn test_display() -> Result<()> {
	let mut board = Board::new();
	board.do_moves_builder(vec![(White, 9, 9), (Black, 8, 9)])?;

	println!("{}", board);
	Ok(())
}

#[test]
fn test_adjacents() -> Result<()> {
	use PieceAdjacencyType as PAT;
	let mut board = Board::new();

	board.do_moves_builder(vec![
		(White, 9, 9),
		(Black, 9, 8),
		(White, 9, 10),
		(White, 0, 0),
		(White, 9, 0),
	])?;

	println!("{}", board);

	println!(
		"{:?}",
		board.adjacents((9, 10)).map(|a| a.as_type()).to_array()
	);
	println!(
		"{:?}",
		board.adjacents((9, 9)).map(|a| a.as_type()).to_array()
	);
	println!(
		"{:?}",
		board.adjacents((9, 8)).map(|a| a.as_type()).to_array()
	);
	println!("=====");
	println!(
		"{:?}",
		board.adjacents((0, 0)).map(|a| a.as_type()).to_array()
	);
	println!(
		"{:?}",
		board.adjacents((9, 0)).map(|a| a.as_type()).to_array()
	);

	assert_eq!(
		board.adjacents((9, 9)).map(|a| a.as_type()).to_array(),
		[PAT::Piece, PAT::Piece, PAT::Empty, PAT::Empty]
	);
	assert_eq!(
		board.adjacents((9, 8)).map(|a| a.as_type()).to_array(),
		[PAT::Piece, PAT::Empty, PAT::Empty, PAT::Empty]
	);
	assert_eq!(
		board.adjacents((9, 10)).map(|a| a.as_type()).to_array(),
		[PAT::Empty, PAT::Piece, PAT::Empty, PAT::Empty]
	);
	assert_eq!(
		board.adjacents((0, 0)).map(|a| a.as_type()).to_array(),
		[PAT::Empty, PAT::Edge, PAT::Edge, PAT::Empty]
	);
	assert_eq!(
		board.adjacents((9, 0)).map(|a| a.as_type()).to_array(),
		[PAT::Empty, PAT::Edge, PAT::Empty, PAT::Empty]
	);

	assert_eq!(
		board.adjacents((15, 15)).map(|a| a.as_type()).to_array(),
		[PAT::Empty, PAT::Empty, PAT::Empty, PAT::Empty]
	);

	Ok(())
}
#[test]
fn test_shape() -> Result<()> {
	let mut board = Board::new();
	board.do_moves_builder(vec![
		(White, 9, 9),
		(White, 9, 8),
		(White, 9, 7),
		(White, 8, 7),
		(White, 7, 6),
		(White, 0, 0),
		(White, 0, 1),
		(White, 15, 15),
		(White, 15, 16),
		(Black, 16, 15),
		(Black, 16, 16),
		(Black, 15, 17),
		(White, 16, 17),
	])?;

	println!("{}", board);
	assert_eq!(board.shape((9, 9)).len(), 4);
	assert_eq!(board.shape((0, 1)).len(), 2);
	assert_eq!(board.shape((7, 6)).len(), 1);
	assert_eq!(board.shape((15, 16)).len(), 2);
	assert_eq!(board.shape((16, 16)).len(), 2);
	assert_eq!(board.shape((15, 17)).len(), 1);
	assert_eq!(board.shape((16, 17)).len(), 1);

	Ok(())
}

#[test]
fn test_remove_shape() -> Result<()> {
	let mut board = Board::new();

	for i in 0..10u8 {
		for j in 0..10u8 {
			board.do_move(PlacedPiece::new(White, (i, j)))?;
		}
		board.do_move(PlacedPiece::new(Black, (i, 10)))?;
		board.do_move(PlacedPiece::new(White, (i, 11)))?;
	}
	println!("{}", board);
	assert_eq!(board.num_pieces(White), 110);
	board.remove_shape((0, 0));
	println!("{}", board);
	assert_eq!(board.num_pieces(White), 10);
	assert_eq!(board.num_pieces(Black), 10);
	board.remove_shape((9, 10));
	println!("{}", board);
	assert_eq!(board.num_pieces(Black), 0);
	board.remove_shape((9, 11));
	println!("{}", board);
	assert_eq!(board.num_pieces_all(), 0);

	Ok(())
}

#[test]
fn test_liberties() -> Result<()> {
	let mut board = Board::new();

	board.do_moves_builder(vec![(White, 0, 0), (White, 0, 9), (White, 9, 9)])?;
	assert_eq!(board.liberties_shape((0, 0)).len(), 2);
	assert_eq!(board.liberties_shape((0, 9)).len(), 3);
	assert_eq!(board.liberties_shape((9, 9)).len(), 4);
	println!("{}", board);

	board.do_moves_builder(vec![(Black, 0, 1), (Black, 1, 0)])?;
	assert_eq!(board.liberties_shape((0, 0)).len(), 0);
	println!("{}", board);

	board.do_moves_builder(vec![
		(Black, 18, 4),
		(Black, 18, 5),
		(Black, 18, 6),
		(Black, 18, 7),
		(White, 17, 4),
		(White, 17, 5),
		(White, 17, 6),
		(White, 17, 7),
	])?;
	println!("{}", board);
	assert_eq!(board.liberties_shape((18, 4)).len(), 2);
	assert_eq!(board.liberties_shape((18, 5)).len(), 2);
	assert_eq!(board.liberties_shape((18, 6)).len(), 2);
	assert_eq!(board.liberties_shape((18, 7)).len(), 2);
	board.do_moves_builder(vec![(White, 18, 8)])?;
	assert_eq!(board.liberties_shape((18, 4)).len(), 1);
	assert_eq!(board.liberties_shape((18, 5)).len(), 1);
	assert_eq!(board.liberties_shape((18, 6)).len(), 1);
	assert_eq!(board.liberties_shape((18, 7)).len(), 1);
	board.set(PlacedPiece::new(White, (18, 3)));
	assert_eq!(board.liberties_shape((18, 4)).len(), 0);
	assert_eq!(board.liberties_shape((18, 5)).len(), 0);
	assert_eq!(board.liberties_shape((18, 6)).len(), 0);
	assert_eq!(board.liberties_shape((18, 7)).len(), 0);
	println!("{}", board);

	Ok(())
}

#[test]
fn test_capture() -> Result<()> {
	let mut board = Board::new();
	board.do_moves_builder(vec![
		(White, 9, 9),
		(Black, 9, 8),
		(Black, 9, 10),
		(Black, 8, 9),
	])?;
	assert_eq!(board.get_color((9, 9)), Some(White));
	println!("{}", board);

	board.do_moves_builder(vec![(Black, 10, 9)])?;
	assert_eq!(board.get_color((9, 9)), None);
	println!("{}", board);

	Ok(())
}

#[test]
fn test_no_self_capture() -> Result<()> {
	let mut board = Board::new();
	board.do_moves_builder(vec![
		(White, 0, 1),
		(White, 1, 1),
		(White, 2, 0),
		(Black, 1, 0),
	])?;

	println!("{}", board);

	board
		.do_moves_builder(vec![(Black, 0, 0)])
		.expect_err("Expected to be an invalid move");

	println!("{}", board);
	board
		.do_moves_builder(vec![(Black, 2, 1), (Black, 3, 0), (Black, 0, 0)])
		.expect("Expected capture with no liberties to be valid");

	println!("{}", board);
	Ok(())
}
