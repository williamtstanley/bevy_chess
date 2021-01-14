use bevy::prelude::*;
use bevy_mod_picking::*;

mod pieces;
use pieces::*;

mod board;
use board::*;

fn setup(commands: &mut Commands) {
    commands
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
            )),
            ..Default::default()
        })
        .with(PickSource::default())
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            title: "Chess!".to_string(),
            width: 1600.,
            height: 1600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(PiecesPlugin)
        // .add_plugin(DebugPickingPlugin)
        .add_startup_system(setup.system())
        .run();
}
