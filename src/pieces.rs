use super::*;

const ORDER_LAYER: f32 = 5.0;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum PieceColor {
    #[default]
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Component, Clone, Copy)]
pub struct Piece {
    pub my_type: PieceType,
    pub color: PieceColor,
    // NOTE: Position
    pub x: u8,
    pub y: u8,
}

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        // app.add_state::<Game>()
        app.add_systems(Startup, spawn_pieces)
            .add_systems(PostStartup, load_sprites)
            .add_systems(Update, move_pieces);
    }
}

fn spawn_pieces(mut commands: Commands) {
    let chess_pieces: [(PieceType, Vec<(u8, u8)>); 6] = [
        (PieceType::King, vec![(4, 0)]),
        (PieceType::Queen, vec![(3, 0)]),
        (PieceType::Rook, vec![(0, 0), (7, 0)]),
        (PieceType::Bishop, vec![(1, 0), (6, 0)]),
        (PieceType::Knight, vec![(2, 0), (5, 0)]),
        (PieceType::Pawn, (0..MAX).map(|x| (x, 1)).collect()),
    ];

    for (my_type, pos_arr) in chess_pieces.into_iter() {
        for i in 0..pos_arr.len() {
            commands.spawn(Piece {
                my_type: my_type,
                color: PieceColor::White,
                x: pos_arr[i].0,
                y: pos_arr[i].1,
            });
            commands.spawn(Piece {
                my_type: my_type,
                color: PieceColor::Black,
                x: pos_arr[i].0,
                y: MAX - 1 - pos_arr[i].1,
            });
        }
    }
}

fn load_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Piece)>,
) {
    for (id, piece) in query.iter() {
        commands.entity(id).insert(SpriteBundle {
            texture: {
                let image_path = format!(
                    "ARABIAN CHESS/sprites/pieces/{:?}_{:?}.png",
                    piece.color, piece.my_type
                )
                .to_lowercase();
                asset_server.load(image_path)
            },
            transform: {
                let pos = Vec3::new(piece.x as f32 * SIZE, piece.y as f32 * SIZE, ORDER_LAYER);
                let scale = Vec3::new(SIZE / 16., SIZE / 16., 1.);

                Transform::from_translation(pos).with_scale(scale)
            },
            ..default()
        });
    }
}

fn move_pieces(
    mut commands: Commands,
    mut selection: ResMut<Selection>,
    mut query: Query<(Entity, &mut Piece, &mut Transform)>,
    mut manager: ResMut<TurnManager>,
) {
    if selection.to == Vec2::NEG_ONE {
        return;
    }

    // let mut piece_vector = Vec::new();
    let mut attacker: Option<(Mut<'_, Piece>, Mut<'_, Transform>)> = None;
    let mut defender: Option<(Mut<'_, Piece>, Mut<'_, Transform>, Entity)> = None;

    for (id, piece, transform) in query.iter_mut() {
        let pos = Vec2::new(piece.x as f32, piece.y as f32);
        if pos == selection.from {
            attacker = Some((piece, transform));
        } else if pos == selection.to {
            defender = Some((piece, transform, id));
        }

        // piece_vector.push(piece);
    }

    let Some(attack) = attacker else {
        return;
    };

    // TODO: fill arguments
    // if attack.0.my_type != PieceType::Knight
    //     && !path_empty(
    //         (
    //             (selection.old.x / SIZE) as u8,
    //             (selection.old.y / SIZE) as u8,
    //         ),
    //         (
    //             (selection.new.x / SIZE) as u8,
    //             (selection.new.y / SIZE) as u8,
    //         ),
    //         piece_vector,
    //     )
    // {
    //     return;
    // }

    if let Some(defend) = defender {
        if defend.0.color == attack.0.color {
            let king_rook = (attack.0.my_type == PieceType::King
                && defend.0.my_type == PieceType::Rook)
                || (attack.0.my_type == PieceType::Rook && defend.0.my_type == PieceType::King);
            if king_rook {
                // FIX: Check the following conditions rules
                // NOTE: 1. Neither the king nor the rook involved in the switch must have moved previously.
                // NOTE: 2. There should be no pieces between the king and the rook.
                // NOTE: 3. The king should not be in check.
                // NOTE: 4. The squares that the king crosses and ends up on should not be under attack.

                move_piece(defend.0, defend.1, selection.from);
                move_piece(attack.0, attack.1, selection.to);
                selection.from = Vec2::NEG_ONE;
                selection.to = Vec2::NEG_ONE;
                manager.next_turn();
                return;
            }

            selection.to = Vec2::NEG_ONE;
            return;
        }

        commands.entity(defend.2).despawn();
    }

    move_piece(attack.0, attack.1, selection.to);
    selection.from = Vec2::NEG_ONE;
    selection.to = Vec2::NEG_ONE;
    manager.next_turn();

    fn move_piece(mut piece: Mut<'_, Piece>, mut transform: Mut<'_, Transform>, pos: Vec2) {
        piece.x = pos.x as u8;
        piece.y = pos.y as u8;

        transform.translation = Vec3::new(pos.x * SIZE, pos.y * SIZE, ORDER_LAYER);
    }
}

fn path_empty(from: (u8, u8), to: (u8, u8), pieces: Vec<&Piece>) -> bool {
    // TODO: Check that there are no pieces in between the two points
    return true;

    // NOTE: Stolen code
    //     if begin.0 == end.0 {
    //         for piece in pieces {
    //             if piece.x == begin.0
    //                 && ((piece.y > begin.1 && piece.y < end.1)
    //                     || (piece.y > end.1 && piece.y < begin.1))
    //             {
    //                 return false;
    //             }
    //         }
    //     }

    //     if begin.1 == end.1 {
    //         for piece in pieces {
    //             if piece.y == begin.1
    //                 && ((piece.x > begin.0 && piece.x < end.0)
    //                     || (piece.x > end.0 && piece.x < begin.0))
    //             {
    //                 return false;
    //             }
    //         }
    //     }

    //     let x_diff = (begin.0 - end.0).abs();
    //     let y_diff = (begin.1 - end.1).abs();
    //     if x_diff == y_diff {
    //         for i in 1..x_diff {
    //             let pos = if begin.0 < end.0 && begin.1 < end.1 {
    //                 // left bottom - right top
    //                 (begin.0 + i, begin.1 + i)
    //             } else if begin.0 < end.0 && begin.1 > end.1 {
    //                 // left top - right bottom
    //                 (begin.0 + i, begin.1 - i)
    //             } else if begin.0 > end.0 && begin.1 < end.1 {
    //                 // right bottom - left top
    //                 (begin.0 - i, begin.1 + i)
    //             } else {
    //                 // begin.0 > end.0 && begin.1 > end.1
    //                 // right top - left bottom
    //                 (begin.0 - i, begin.1 - i)
    //             };

    //             if piece_color(pos, pieces).is_some() {
    //                 return false;
    //             }
    //         }
    //     }

    //     true
}
