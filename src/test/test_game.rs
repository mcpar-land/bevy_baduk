use crate::{
	adjacency::*,
	board::*,
	error::*,
	game::*,
	piece::*,
};

#[test]
fn test_game() -> Result<()> {
	let mut game = Game::new(White);
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
	let mut game = Game::new(White);
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
