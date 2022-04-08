use bevy::prelude::*;
use bevy_mod_picking::*;
use board::*;
use pieces::*;

mod board;
mod pieces;

fn main()
{
    App::new()
        .insert_resource(Msaa {samples: 4})
        .insert_resource(WindowDescriptor
        {
            title: "Chess".to_string(),
            width: 1600.,
            height: 1000.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(BoardPlugin)
        .add_plugin(PiecesPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup
(
    mut commands: Commands,
)
{
    commands
        .spawn_bundle(PointLightBundle
        {
            point_light: PointLight
            {
                intensity: 1500.0,
                shadows_enabled: true,
                ..Default::default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..Default::default()
        });
    commands
        .spawn_bundle(PerspectiveCameraBundle
        {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(0., 8., 4., 0.).normalize(),
                Vec3::new(3.5, 8., -3.),
            )),
            ..Default::default()
        })
        .insert_bundle(PickingCameraBundle::default());
}