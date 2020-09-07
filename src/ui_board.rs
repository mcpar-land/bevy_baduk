use bevy::prelude::*;

use crate::{
	game::Game,
	ui_piece::*,
	GlobalBoard,
	MaterialHandles,
};

pub fn redraw_board(
	commands: &mut Commands,
	materials: &MaterialHandles,
	board: Entity,
	game: &Game,
	ui_pieces: &mut Query<(Entity, &UiPiece)>,
) {
	for piece in &mut ui_pieces.iter() {
		commands.despawn(piece.0);
	}
	for piece in game.board.all_pieces() {
		place_ui_piece(
			commands,
			board,
			materials.piece_mat(piece.piece.color, false),
			piece.pos,
			true,
		);
	}
	println!("========== Redrew board ==========");
}
