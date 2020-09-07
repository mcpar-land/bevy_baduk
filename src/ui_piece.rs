use crate::{
	piece::PieceColor,
	PosValue,
};
use bevy::prelude::*;

const PIECE_SIZE: f32 = 23.0;

pub struct UiPiece;

pub fn place_ui_piece(
	commands: &mut Commands,
	board_entity: Entity,
	material: Handle<ColorMaterial>,
	pos: (u8, u8),
	visible: bool,
) {
	let piece_entity = Entity::new();
	commands
		.spawn_as_entity(piece_entity, ui_piece(material, pos, visible))
		.with(UiPiece)
		.with(PosValue(pos))
		.push_children(board_entity, &[piece_entity]);
}

pub fn ui_piece(
	material: Handle<ColorMaterial>,
	pos: (u8, u8),
	visible: bool,
) -> NodeComponents {
	NodeComponents {
		style: Style {
			display: Display::Flex,
			size: Size::new(Val::Px(PIECE_SIZE), Val::Px(PIECE_SIZE)),
			position_type: PositionType::Absolute,
			position: Rect {
				bottom: Val::Px(2.0 + (pos.1 as f32 * PIECE_SIZE)),
				left: Val::Px(2.0 + (pos.0 as f32 * PIECE_SIZE)),
				..Default::default()
			},
			..Default::default()
		},
		material,
		draw: Draw {
			is_transparent: true,
			is_visible: visible,
			..Default::default()
		},
		..Default::default()
	}
}

pub fn set_ui_piece_pos(piece_style: &mut Style, pos: (u8, u8)) {
	piece_style.position = Rect {
		bottom: Val::Px(2.0 + (pos.1 as f32 * PIECE_SIZE)),
		left: Val::Px(2.0 + (pos.0 as f32 * PIECE_SIZE)),
		..Default::default()
	};
}
