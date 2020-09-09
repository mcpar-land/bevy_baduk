use crate::{
	piece::{
		Piece,
		PieceColor,
		PlacedPiece,
	},
	ui_piece,
	Game,
	GlobalBoard,
	GlobalEntities,
	GlobalHover,
	MaterialHandles,
	PosValue,
};
use bevy::{
	input::mouse::MouseMotion,
	prelude::*,
};
use ui_piece::UiPiece;

pub struct BoardEvent {
	pub pos: (u8, u8),
	pub event_type: Interaction,
}

#[derive(Default)]
pub struct BoardEventResource(pub EventReader<BoardEvent>);

#[derive(Default)]
pub struct MostRecentButtonResource((u8, u8));

pub fn mouse_system(
	mut events: ResMut<Events<BoardEvent>>,
	mut most_recent_button: ResMut<MostRecentButtonResource>,
	mut button_query: Query<(&Button, Mutated<Interaction>, &PosValue)>,
) {
	for (_, interaction, value) in &mut button_query.iter() {
		let interaction: Mutated<Interaction> = interaction;
		events.send(BoardEvent {
			pos: value.0,
			event_type: *interaction,
		});
		most_recent_button.0 = value.0;
	}
}

pub fn board_events_system(
	mut commands: Commands,
	materials: Res<MaterialHandles>,
	mut state: ResMut<BoardEventResource>,
	mut game: ResMut<Game>,
	events: Res<Events<BoardEvent>>,
	global_entities: Res<GlobalEntities>,
	global_hover: Query<(Entity, &mut GlobalHover)>,
	mut ui_pieces: Query<(Entity, &UiPiece)>,
) {
	let board_entity = global_entities.board;
	let hover_entity = global_entities.hover;
	let mut hover_style = global_hover.get_mut::<Style>(hover_entity).unwrap();
	let mut hover_draw = global_hover.get_mut::<Draw>(hover_entity).unwrap();
	let mut hover_mat = global_hover
		.get_mut::<Handle<ColorMaterial>>(hover_entity)
		.unwrap();

	let current_turn = game.current_turn_color();

	for ev in state.0.iter(&events) {
		let ev: &BoardEvent = ev;
		match ev.event_type {
			Interaction::Hovered => {
				// println!("Hovered on {:?}", ev.pos);
				ui_piece::set_ui_piece_pos(&mut hover_style, ev.pos);
				// hover_draw.is_visible = game.board.get_color(ev.pos).is_none();
				*hover_mat = if game
					.board
					.valid_move(PlacedPiece {
						piece: Piece {
							color: current_turn,
						},
						pos: ev.pos,
					})
					.is_ok()
				{
					materials.piece_mat(current_turn, true).as_handle()
				} else {
					materials.red.as_handle()
				};
			}
			Interaction::Clicked => {
				// println!("Clicked on {:?}", ev.pos);
				match game.do_move(PlacedPiece {
					piece: Piece {
						color: current_turn,
					},
					pos: ev.pos,
				}) {
					Ok(_) => {
						crate::ui_board::redraw_board(
							&mut commands,
							&materials,
							board_entity,
							&game,
							&mut ui_pieces,
						);
					}
					Err(err) => {
						println!("{}", err);
					}
				};
			}
			Interaction::None => {
				// println!("Exited {:?}", ev.pos);
			}
		}
	}
}

pub fn keyboard_events_system(
	mut commands: Commands,
	materials: Res<MaterialHandles>,
	global_entities: Res<GlobalEntities>,
	mut game: ResMut<Game>,
	keyboard_input: Res<Input<KeyCode>>,
	mut ui_pieces: Query<(Entity, &UiPiece)>,
) {
	let delta = if keyboard_input.just_pressed(KeyCode::Left) {
		-1
	} else if keyboard_input.just_pressed(KeyCode::Right) {
		1
	} else {
		0
	};
	if delta == 0 {
		return;
	}
	println!("Changed turn by {}", delta);
	let old_turn = game.current_turn();
	game.offset_turn(delta);
	if old_turn != game.current_turn() {
		crate::ui_board::redraw_board(
			&mut commands,
			&materials,
			global_entities.board,
			&game,
			&mut ui_pieces,
		);
	}
}
