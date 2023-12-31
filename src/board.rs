use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

use crate::input::Selection;

pub const SIZE: f32 = 80.0; // NOTE: can we make it relative to the screen height? SIZE = window.height / 13.5
pub const HALF_SIZE: f32 = SIZE * 0.5;
pub const MAX: u8 = 8;
const ORDER_LAYER: f32 = 0.0;

#[derive(Bundle)]
struct BoardBundle {
    model: SceneBundle,
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb_u8(57, 31, 33)))
            .add_systems(Startup, load_sprites)
            .add_systems(Update, draw_selected);
    }
}

fn load_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut choice = true;
    let white_image_path = "ARABIAN CHESS/sprites/board/board_square_white.png";
    let black_image_path = "ARABIAN CHESS/sprites/board/board_square_black.png";

    for i in 0..MAX {
        for j in 0..MAX {
            commands.spawn(SpriteBundle {
                texture: if choice {
                    asset_server.load(white_image_path)
                } else {
                    asset_server.load(black_image_path)
                },
                transform: Transform::from_xyz(SIZE * (i as f32), SIZE * (j as f32), ORDER_LAYER)
                    .with_scale(Vec3::new(SIZE / 16., SIZE / 16., 1.)),
                ..default()
            });
            choice = !choice;
        }
        choice = !choice;
    }
}

pub fn inside_board(x: f32, y: f32) -> bool {
    let half = SIZE * 0.5;
    let min = -half; // NOTE: Same as -> (0.0 * SIZE) - half
    let max = ((MAX as f32 - 1.0) * SIZE) + half;
    return x > min && x < max && y > min && y < max;
}

pub fn square_center(x: f32, y: f32) -> Vec2 {
    return Vec2::new(nearest_center(x), nearest_center(y));

    fn nearest_center(axis: f32) -> f32 {
        let mut nearest: u8 = 0;
        let mut smallest_diff: f32 = axis; // NOTE: Same as -> ((0 as f32 * SIZE) - axis_value).abs()

        for i in 1..MAX {
            let diff = ((i as f32 * SIZE) - axis).abs();
            if diff < smallest_diff {
                smallest_diff = diff;
                nearest = i;
            }
        }

        return nearest as f32 * SIZE;
    }
}

fn draw_selected(mut painter: ShapePainter, selection: Res<Selection>) {
    if selection.from == Vec2::NEG_ONE {
        return;
    }

    let pos = Vec3::new(
        selection.from.x * SIZE,
        selection.from.y * SIZE,
        ORDER_LAYER + 1.,
    );
    painter.set_translation(pos);

    painter.color = Color::WHITE;
    painter.circle(SIZE * 0.5);
}

//
//
//

// use crate::pieces::*;
// use bevy::{app::AppExit, prelude::*};
// use bevy_mod_picking::*;

// pub struct Square {
//     pub x: u8,
//     pub y: u8,
// }
// impl Square {
//     fn is_white(&self) -> bool {
//         (self.x + self.y + 1) % 2 == 0
//     }
// }

// fn create_board(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     materials: Res<SquareMaterials>,
// ) {
//     // Add meshes
//     let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));

//     // Spawn 64 squares
//     for i in 0..8 {
//         for j in 0..8 {
//             commands
//                 .spawn_bundle(PbrBundle {
//                     mesh: mesh.clone(),
//                     // Change material according to position to get alternating pattern
//                     material: if (i + j + 1) % 2 == 0 {
//                         materials.white_color.clone()
//                     } else {
//                         materials.black_color.clone()
//                     },
//                     transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
//                     ..Default::default()
//                 })
//                 .insert_bundle(PickableBundle::default())
//                 .insert(Square { x: i, y: j });
//         }
//     }
// }

// fn color_squares(
//     selected_square: Res<SelectedSquare>,
//     materials: Res<SquareMaterials>,
//     mut query: Query<(Entity, &Square, &mut Handle<StandardMaterial>)>,
//     picking_camera_query: Query<&PickingCamera>,
// ) {
//     // Get entity under the cursor, if there is one
//     let top_entity = match picking_camera_query.iter().last() {
//         Some(picking_camera) => match picking_camera.intersect_top() {
//             Some((entity, _intersection)) => Some(entity),
//             None => None,
//         },
//         None => None,
//     };

//     for (entity, square, mut material) in query.iter_mut() {
//         // Change the material
//         *material = if Some(entity) == top_entity {
//             materials.highlight_color.clone()
//         } else if Some(entity) == selected_square.entity {
//             materials.selected_color.clone()
//         } else if square.is_white() {
//             materials.white_color.clone()
//         } else {
//             materials.black_color.clone()
//         };
//     }
// }

// struct SquareMaterials {
//     highlight_color: Handle<StandardMaterial>,
//     selected_color: Handle<StandardMaterial>,
//     black_color: Handle<StandardMaterial>,
//     white_color: Handle<StandardMaterial>,
// }

// impl FromWorld for SquareMaterials {
//     fn from_world(world: &mut World) -> Self {
//         let world = world.cell();
//         let mut materials = world
//             .get_resource_mut::<Assets<StandardMaterial>>()
//             .unwrap();
//         SquareMaterials {
//             highlight_color: materials.add(Color::rgb(0.8, 0.3, 0.3).into()),
//             selected_color: materials.add(Color::rgb(0.9, 0.1, 0.1).into()),
//             black_color: materials.add(Color::rgb(0., 0.1, 0.1).into()),
//             white_color: materials.add(Color::rgb(1., 0.9, 0.9).into()),
//         }
//     }
// }

// #[derive(Default)]
// struct SelectedSquare {
//     entity: Option<Entity>,
// }
// #[derive(Default)]
// struct SelectedPiece {
//     entity: Option<Entity>,
// }
// pub struct PlayerTurn(pub PieceColor);
// impl Default for PlayerTurn {
//     fn default() -> Self {
//         Self(PieceColor::White)
//     }
// }
// impl PlayerTurn {
//     fn change(&mut self) {
//         self.0 = match self.0 {
//             PieceColor::White => PieceColor::Black,
//             PieceColor::Black => PieceColor::White,
//         }
//     }
// }

// fn select_square(
//     mouse_button_inputs: Res<Input<MouseButton>>,
//     mut selected_square: ResMut<SelectedSquare>,
//     mut selected_piece: ResMut<SelectedPiece>,
//     squares_query: Query<&Square>,
//     picking_camera_query: Query<&PickingCamera>,
// ) {
//     // Only run if the left button is pressed
//     if !mouse_button_inputs.just_pressed(MouseButton::Left) {
//         return;
//     }

//     // Get the square under the cursor and set it as the selected
//     if let Some(picking_camera) = picking_camera_query.iter().last() {
//         if let Some((square_entity, _intersection)) = picking_camera.intersect_top() {
//             if let Ok(_square) = squares_query.get(square_entity) {
//                 // Mark it as selected
//                 selected_square.entity = Some(square_entity);
//             }
//         } else {
//             // Player clicked outside the board, deselect everything
//             selected_square.entity = None;
//             selected_piece.entity = None;
//         }
//     }
// }

// fn select_piece(
//     selected_square: Res<SelectedSquare>,
//     mut selected_piece: ResMut<SelectedPiece>,
//     turn: Res<PlayerTurn>,
//     squares_query: Query<&Square>,
//     pieces_query: Query<(Entity, &Piece)>,
// ) {
//     if !selected_square.is_changed() {
//         return;
//     }

//     let square_entity = if let Some(entity) = selected_square.entity {
//         entity
//     } else {
//         return;
//     };

//     let square = if let Ok(square) = squares_query.get(square_entity) {
//         square
//     } else {
//         return;
//     };

//     if selected_piece.entity.is_none() {
//         // Select the piece in the currently selected square
//         for (piece_entity, piece) in pieces_query.iter() {
//             if piece.x == square.x && piece.y == square.y && piece.color == turn.0 {
//                 // piece_entity is now the entity in the same square
//                 selected_piece.entity = Some(piece_entity);
//                 break;
//             }
//         }
//     }
// }

// fn move_piece(
//     mut commands: Commands,
//     selected_square: Res<SelectedSquare>,
//     selected_piece: Res<SelectedPiece>,
//     mut turn: ResMut<PlayerTurn>,
//     squares_query: Query<&Square>,
//     mut pieces_query: Query<(Entity, &mut Piece)>,
//     mut reset_selected_event: EventWriter<ResetSelectedEvent>,
// ) {
//     if !selected_square.is_changed() {
//         return;
//     }

//     let square_entity = if let Some(entity) = selected_square.entity {
//         entity
//     } else {
//         return;
//     };

//     let square = if let Ok(square) = squares_query.get(square_entity) {
//         square
//     } else {
//         return;
//     };

//     if let Some(selected_piece_entity) = selected_piece.entity {
//         let pieces_vec = pieces_query.iter_mut().map(|(_, piece)| *piece).collect();
//         let pieces_entity_vec = pieces_query
//             .iter_mut()
//             .map(|(entity, piece)| (entity, *piece))
//             .collect::<Vec<(Entity, Piece)>>();
//         // Move the selected piece to the selected square
//         let mut piece =
//             if let Ok((_piece_entity, piece)) = pieces_query.get_mut(selected_piece_entity) {
//                 piece
//             } else {
//                 return;
//             };

//         if piece.is_move_valid((square.x, square.y), pieces_vec) {
//             // Check if a piece of the opposite color exists in this square and despawn it
//             for (other_entity, other_piece) in pieces_entity_vec {
//                 if other_piece.x == square.x
//                     && other_piece.y == square.y
//                     && other_piece.color != piece.color
//                 {
//                     // Mark the piece as taken
//                     commands.entity(other_entity).insert(Taken);
//                 }
//             }

//             // Move piece
//             piece.x = square.x;
//             piece.y = square.y;

//             // Change turn
//             turn.change();
//         }

//         reset_selected_event.send(ResetSelectedEvent);
//     }
// }

// struct ResetSelectedEvent;

// fn reset_selected(
//     mut event_reader: EventReader<ResetSelectedEvent>,
//     mut selected_square: ResMut<SelectedSquare>,
//     mut selected_piece: ResMut<SelectedPiece>,
// ) {
//     for _event in event_reader.iter() {
//         selected_square.entity = None;
//         selected_piece.entity = None;
//     }
// }

// struct Taken;
// fn despawn_taken_pieces(
//     mut commands: Commands,
//     mut app_exit_events: EventWriter<AppExit>,
//     query: Query<(Entity, &Piece, &Taken)>,
// ) {
//     for (entity, piece, _taken) in query.iter() {
//         // If the king is taken, we should exit
//         if piece.piece_type == PieceType::King {
//             println!(
//                 "{} won! Thanks for playing!",
//                 match piece.color {
//                     PieceColor::White => "Black",
//                     PieceColor::Black => "White",
//                 }
//             );
//             app_exit_events.send(AppExit);
//         }

//         // Despawn piece and children
//         commands.entity(entity).despawn_recursive();
//     }
// }

// pub struct BoardPlugin;
// impl Plugin for BoardPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app.init_resource::<SelectedSquare>()
//             .init_resource::<SelectedPiece>()
//             .init_resource::<SquareMaterials>()
//             .init_resource::<PlayerTurn>()
//             .add_event::<ResetSelectedEvent>()
//             .add_startup_system(create_board.system())
//             .add_system(color_squares.system())
//             .add_system(select_square.system().label("select_square"))
//             .add_system(
//                 // move_piece needs to run before select_piece
//                 move_piece
//                     .system()
//                     .after("select_square")
//                     .before("select_piece"),
//             )
//             .add_system(
//                 select_piece
//                     .system()
//                     .after("select_square")
//                     .label("select_piece"),
//             )
//             .add_system(despawn_taken_pieces.system())
//             .add_system(reset_selected.system().after("select_square"));
//     }
// }
