#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(clamp)]

mod adjacency;
mod board;
mod error;
mod game;
mod piece;
mod pos;
mod ui_board;
mod ui_piece;

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

pub struct PosValue((u8, u8));

pub struct GlobalHover;
pub struct GlobalBoard;

pub struct GlobalEntities {
	pub board: Entity,
	pub hover: Entity,
	pub text: Entity,
}

impl std::default::Default for GlobalEntities {
	fn default() -> Self {
		Self {
			board: Entity::new(),
			hover: Entity::new(),
			text: Entity::new(),
		}
	}
}

pub struct MaterialHandles {
	pub board: Handle<ColorMaterial>,
	pub piece_b: Handle<ColorMaterial>,
	pub piece_b_alpha: Handle<ColorMaterial>,
	pub piece_w: Handle<ColorMaterial>,
	pub piece_w_alpha: Handle<ColorMaterial>,
	pub red: Handle<ColorMaterial>,
	pub transparent: Handle<ColorMaterial>,
}

impl MaterialHandles {
	pub fn piece_mat(
		&self,
		color: PieceColor,
		alpha: bool,
	) -> Handle<ColorMaterial> {
		match color {
			PieceColor::Black => {
				if alpha {
					self.piece_b_alpha
				} else {
					self.piece_b
				}
			}
			PieceColor::White => {
				if alpha {
					self.piece_w_alpha
				} else {
					self.piece_w
				}
			}
		}
	}
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
	asset_server: Res<AssetServer>,
	mat_handles: Res<MaterialHandles>,
	global_entities: Res<GlobalEntities>,
) {
	let font: Handle<Font> =
		asset_server.load("assets/OpenSans-Regular.ttf").unwrap();

	commands
		.spawn(UiCameraComponents::default())
		.spawn_as_entity(
			global_entities.board,
			NodeComponents {
				style: Style {
					display: Display::Flex,
					size: Size::new(Val::Px(441.0), Val::Px(441.0)),
					position: Rect {
						bottom: Val::Px(0.0),
						left: Val::Px(0.0),
						..Default::default()
					},
					position_type: PositionType::Absolute,
					..Default::default()
				},
				draw: Draw {
					is_visible: true,
					..Default::default()
				},
				material: mat_handles.board,
				..Default::default()
			},
		)
		.with(GlobalBoard)
		.with_children(|parent| {
			parent
				.spawn_as_entity(
					global_entities.hover,
					crate::ui_piece::ui_piece(mat_handles.piece_b, (0, 0), true),
				)
				.with(GlobalHover);
		})
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
						.with(PosValue((i as u8, j as u8)));
				}
			}
		})
		.spawn_as_entity(
			global_entities.text,
			TextComponents {
				style: Style {
					size: Size::new(Val::Percent(100.0), Val::Px(20.0)),
					position_type: PositionType::Absolute,
					position: Rect {
						top: Val::Px(10.0),
						..Default::default()
					},
					..Default::default()
				},
				text: Text {
					value: "Hello, world!".to_string(),
					font,
					style: TextStyle {
						font_size: 30.0,
						color: Color::WHITE,
					},
					..Default::default()
				},
				..Default::default()
			},
		);

	// .spawn_as_entity(global_entities.hover, NodeComponents {});
}

fn main() {
	App::build()
		.add_resource(WindowDescriptor {
			title: "Go in Bevy!".to_string(),
			width: 441,
			height: 471,
			resizable: false,
			..Default::default()
		})
		.add_default_plugins()
		.add_event::<BoardEvent>()
		.init_resource::<MaterialHandles>()
		.add_resource(Game::new(0))
		.add_resource(GlobalEntities::default())
		.add_resource(BoardEventResource(EventReader::default()))
		.add_resource(MostRecentButtonResource::default())
		.add_startup_system(setup.system())
		.add_system(mouse_system.system())
		.add_system(board_events_system.system())
		.add_system(keyboard_events_system.system())
		.run();
}
