use crate::pieces::*;
use bevy::prelude::*;
use bevy_mod_picking::*;

pub struct Square {
    pub x: u8,
    pub y: u8,
}
impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}
#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}
#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}

fn create_board(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));

    for i in 0..8 {
        for j in 0..8 {
            commands
                .spawn(PbrBundle {
                    mesh: mesh.clone(),
                    material: if (i + j + 1) % 2 == 0 {
                        materials.add(Color::rgb(1., 0.9, 0.9).into())
                    } else {
                        materials.add(Color::rgb(0., 0.1, 0.1).into())
                    },
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                    ..Default::default()
                })
                .with(PickableMesh::default())
                .with(Square { x: i, y: j });
        }
    }
}

fn color_squares(
    pick_state: Res<PickState>,
    selected_square: Res<SelectedSquare>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Square, &Handle<StandardMaterial>)>,
) {
    let top_entity = if let Some((entity, _intersection)) = pick_state.top(Group::default()) {
        Some(*entity)
    } else {
        None
    };

    for (enity, square, material_handle) in query.iter() {
        // actual material
        let material = materials.get_mut(material_handle).unwrap();

        material.albedo = if Some(enity) == top_entity {
            Color::rgb(0.8, 0.3, 0.3)
        } else if Some(enity) == selected_square.entity {
            Color::rgb(0.9, 0.1, 0.1)
        } else if square.is_white() {
            Color::rgb(1., 0.9, 0.9)
        } else {
            Color::rgb(0., 0.1, 0.1)
        };
    }
}

fn select_square(
    pick_state: Res<PickState>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    squares_query: Query<&Square>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some((square_entity, _intersection)) = pick_state.top(Group::default()) {
        if let Ok(square) = squares_query.get(*square_entity) {
            selected_square.entity = Some(*square_entity);

            if let Some(selected_piece_entity) = selected_piece.entity {
                if let Ok((_piece_entity, mut piece)) = pieces_query.get_mut(selected_piece_entity)
                {
                    piece.x = square.x;
                    piece.y = square.y;
                }
                selected_square.entity = None;
                selected_piece.entity = None;
            } else {
                for (piece_entity, piece) in pieces_query.iter_mut() {
                    if piece.x == square.x && piece.y == square.y {
                        selected_piece.entity = Some(piece_entity);
                        break;
                    }
                }
            }
        }
    } else {
        selected_square.entity = None;
        selected_piece.entity = None;
    };
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .add_startup_system(create_board.system())
            .add_system(color_squares.system())
            .add_system(select_square.system());
    }
}
