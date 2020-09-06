use crate::{
	piece::PieceColor,
	BoardEntity,
	ButtonValue,
	Game,
	MaterialHandles,
};
use bevy::{
	input::mouse::MouseMotion,
	prelude::*,
};

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
	mut button_query: Query<(&Button, Mutated<Interaction>, &ButtonValue)>,
) {
	for (_, interaction, value) in &mut button_query.iter() {
		let interaction: Mutated<Interaction> = interaction;
		events.send(BoardEvent {
			pos: value.pos,
			event_type: *interaction,
		});
		most_recent_button.0 = value.pos;
	}
}

pub fn board_events_system(
	mut commands: Commands,
	materials: Res<MaterialHandles>,
	mut state: ResMut<BoardEventResource>,
	events: Res<Events<BoardEvent>>,
	board_entity: Res<BoardEntity>,
) {
	let board_ui = board_entity.0;

	for ev in state.0.iter(&events) {
		let ev: &BoardEvent = ev;
		match ev.event_type {
			Interaction::Hovered => {
				println!("Hovered on {:?}", ev.pos);
			}
			Interaction::Clicked => {
				println!("Clicked on {:?}", ev.pos);
			}
			Interaction::None => {
				println!("Exited {:?}", ev.pos);
			}
		}
	}
}

fn add_hover_stone(
	commands: &mut Commands,
	board_entity: Entity,
	pos: (u8, u8),
	color: PieceColor,
) {
}

fn remove_hover_stone(
	commands: &mut Commands,
	board_entity: Entity,
	pos: (u8, u8),
) {
}
