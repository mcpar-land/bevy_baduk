#![allow(dead_code)]
#![allow(unused_imports)]

mod adjacency;
mod board;
mod error;
mod game;
mod piece;
mod pos;

#[cfg(test)]
mod test {
	mod test_board;
	mod test_game;
}

mod systems;

use bevy::prelude::*;

use game::Game;
use piece::PieceColor;
use systems::*;

#[derive(Bundle)]
pub struct ButtonValue {
	pos: (u8, u8),
}

pub struct BoardEntity(pub Entity);

pub struct MaterialHandles {
	board: Handle<ColorMaterial>,
	piece_b: Handle<ColorMaterial>,
	piece_b_alpha: Handle<ColorMaterial>,
	piece_w: Handle<ColorMaterial>,
	piece_w_alpha: Handle<ColorMaterial>,
	red: Handle<ColorMaterial>,
	transparent: Handle<ColorMaterial>,
}

impl FromResources for MaterialHandles {
	fn from_resources(resources: &Resources) -> Self {
		let asset_server = resources.get::<AssetServer>().unwrap();
		let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();

		let texture_board = asset_server.load("assets/board.png").unwrap();
		let texture_piece_b = asset_server.load("assets/b.png").unwrap();
		let texture_piece_w = asset_server.load("assets/w.png").unwrap();

		MaterialHandles {
			board: materials.add(texture_board.into()),
			piece_b: materials.add(texture_piece_b.into()),
			piece_b_alpha: materials.add(ColorMaterial {
				texture: Some(texture_piece_b),
				color: Color::rgba(1.0, 1.0, 1.0, 0.5),
			}),
			piece_w: materials.add(texture_piece_w.into()),
			piece_w_alpha: materials.add(ColorMaterial {
				texture: Some(texture_piece_w),
				color: Color::rgba(1.0, 1.0, 1.0, 0.5),
			}),
			red: materials.add(ColorMaterial {
				color: Color::RED,
				..Default::default()
			}),
			transparent: materials.add(ColorMaterial {
				color: Color::rgba(1.0, 1.0, 1.0, 0.0),
				..Default::default()
			}),
		}
	}
}

fn setup(
	mut commands: Commands,
	mat_handles: Res<MaterialHandles>,
	board_entity: Res<BoardEntity>,
) {
	commands
		.spawn(UiCameraComponents::default())
		.spawn_as_entity(
			board_entity.0,
			NodeComponents {
				style: Style {
					size: Size::new(Val::Px(441.0), Val::Px(441.0)),
					position: Rect {
						bottom: Val::Px(0.0),
						left: Val::Px(0.0),
						..Default::default()
					},
					position_type: PositionType::Absolute,
					..Default::default()
				},
				material: mat_handles.board,
				..Default::default()
			},
		)
		.with_children(|parent| {
			for i in 0..19 {
				for j in 0..19 {
					parent
						.spawn(ButtonComponents {
							style: Style {
								size: Size::new(Val::Px(23.0), Val::Px(23.0)),
								position_type: PositionType::Absolute,
								position: Rect {
									bottom: Val::Px(2.0 + (j * 23) as f32),
									left: Val::Px(2.0 + (i * 23) as f32),
									..Default::default()
								},
								..Default::default()
							},
							draw: Draw {
								is_visible: false,
								..Default::default()
							},
							..Default::default()
						})
						.with(ButtonValue {
							pos: (j as u8, i as u8),
						});
				}
			}
		});
}

fn main() {
	App::build()
		.add_resource(WindowDescriptor {
			width: 441,
			height: 441,
			resizable: false,
			..Default::default()
		})
		.add_default_plugins()
		.add_event::<BoardEvent>()
		.init_resource::<MaterialHandles>()
		.add_resource(Game::new(PieceColor::White))
		.add_resource(BoardEntity(Entity::new()))
		.add_resource(BoardEventResource(EventReader::default()))
		.add_resource(MostRecentButtonResource::default())
		.add_startup_system(setup.system())
		.add_system(mouse_system.system())
		.add_system(board_events_system.system())
		.run();
}
