use crate::{
	adjacency::*,
	board::*,
	error::*,
	game::*,
	piece::*,
};

#[test]
fn test_game() -> Result<()> {
	let mut game = Game::new(0);
	game.do_moves_builder(vec![
		(White, 3, 3),
		(Black, 2, 3),
		(White, 3, 4),
		(Black, 15, 15),
	])?;
	println!("{}", game);
	Ok(())
}

#[test]
fn test_history() -> Result<()> {
	let mut game = Game::new(0);
	game.do_moves_builder(vec![
		(White, 0, 0),
		(Black, 0, 1),
		(White, 1, 0),
		(Black, 1, 1),
		(White, 2, 0),
		(Black, 2, 1),
		(White, 9, 9),
		(Black, 3, 0),
		(White, 9, 10),
	])?;
	println!("{}", game);

	Ok(())
}

#[test]
fn test_handicap() {
	for i in 0..=9u8 {
		let game = Game::new(i);
		println!("handicap: {}", i);
		println!("{}", game);
		assert_eq!(i as u16, game.board.num_pieces_all());
	}
}

#[test]
fn test_move_timeline() -> Result<()> {
	let mut game = Game::new(0);
	game.do_moves_builder(vec![(Black, 0, 0), (White, 0, 1), (Black, 1, 1)])?;
	println!("{}", game);
	assert_eq!(game.board.num_pieces_all(), 3);

	game.set_position(GamePosition::Past(1))?;
	println!("{}", game);
	assert_eq!(game.board.num_pieces_all(), 1);

	game.do_move(PlacedPiece {
		piece: Piece { color: White },
		pos: (9, 9),
	})?;
	assert_eq!(game.board.num_pieces_all(), 1);

	game.set_position(GamePosition::Current)?;
	assert_eq!(game.board.num_pieces_all(), 4);

	game.do_moves_builder(vec![(Black, 9, 8), (White, 1, 0)])?;
	assert_eq!(game.board.num_pieces_all(), 5);

	game.set_position(GamePosition::Past(2))?;
	assert_eq!(game.board.num_pieces_all(), 2);

	game.set_position(GamePosition::Current)?;
	assert_eq!(game.board.num_pieces_all(), 5);

	Ok(())
}
