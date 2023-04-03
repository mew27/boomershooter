mod configs;

use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI,
};

use bevy::{
    input::{
        keyboard::KeyboardInput,
        mouse::{self, MouseMotion},
    },
    prelude::*,
};
use configs::{ButtonAction, RawInputContainer};

#[derive(Component)]
struct Hitbox;

#[derive(Component)]
struct GroundReference;

#[derive(Resource)]
struct Configs(HashMap<RawInputContainer, ButtonAction>);

#[derive(Resource)]
struct ActionSet(HashSet<ButtonAction>);

fn setup_window(mut window_query: Query<&mut Window>) {
    let mut window = window_query.single_mut();
    
    window.cursor.visible = false;
    window.title = String::from("Boomer Shooter");
}

fn spawn_target(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let square_pos = Transform::from_xyz(0., 5., 5.);

    // square
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube { size: 5. }.into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: square_pos,
            ..default()
        },
        Hitbox,
    ));

    //plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(500.).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        //transform: Transform::from_xyz(-10., 0., -5.),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    let camera_initial_pos = Transform::from_xyz(21., 5., 6.).with_rotation(Quat::from_xyzw(0., 0.7, 0., 0.7));
    
    // camera
    commands.spawn(Camera3dBundle {
        transform: camera_initial_pos.clone(),
        ..default()
    });

    // ground reference
    commands.spawn((camera_initial_pos.clone(), GroundReference));
}

fn handle_movement(
    mut action_set: ResMut<ActionSet>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let mut camera_transform = camera_query.single_mut();

    let initial_pos = camera_transform.translation;

    let mut final_pos = initial_pos;

    match action_set.0.take(&ButtonAction::MoveForward) {
        Some(_) => {
            final_pos += camera_transform.forward() / 2.;
        }
        _ => (),
    }

    match action_set.0.take(&ButtonAction::MoveBackward) {
        Some(_) => {
            final_pos += -camera_transform.forward() / 2.;
        }
        _ => (),
    }

    match action_set.0.take(&ButtonAction::MoveLeft) {
        Some(_) => {
            final_pos += camera_transform.left() / 2.;
        }
        _ => (),
    }

    match action_set.0.take(&ButtonAction::MoveRight) {
        Some(_) => {
            final_pos += -camera_transform.left() / 2.;
        }
        _ => (),
    }

    final_pos.y = 5.;
    camera_transform.translation = final_pos;
}

fn handle_raw_input(
    configs: Res<Configs>,
    mut action_set: ResMut<ActionSet>,
    keys_query: Res<Input<KeyCode>>,
    mouse_query: Res<Input<MouseButton>>,
    //camera_query: Query<&Transform, With<Camera>>,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<StandardMaterial>>,
    //mut commands: Commands,
) {
    for input_container in configs.0.keys() {
        match input_container {
            RawInputContainer::KeyCode(key) => {
                if keys_query.pressed(*key) {
                    action_set
                        .0
                        .insert(*configs.0.get(input_container).unwrap());
                }
            }
            RawInputContainer::MouseButton(button) => {
                if mouse_query.pressed(*button) {
                    action_set
                        .0
                        .insert(*configs.0.get(input_container).unwrap());
                }
            }
        }
    }

    //let camera_transform = camera_query.single();

    //const HITSCAN_SIZE: f32 = 50.;

    // if mouse_query.pressed(MouseButton::Left) {
    //     commands.spawn(PbrBundle {
    //         mesh: meshes.add(
    //             shape::Cylinder {
    //                 radius: 0.05,
    //                 height: HITSCAN_SIZE,
    //                 ..default()
    //             }
    //             .into(),
    //         ),
    //         material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //         transform: Transform {
    //             translation: camera_transform.translation
    //                 + (camera_transform.forward() * HITSCAN_SIZE / 2.),
    //             ..default()
    //         }
    //         .looking_to(camera_transform.down(), camera_transform.forward()),
    //         //visibility: Visibility::Hidden,
    //         ..default()
    //     });
    // }
}

fn handle_camera_mov(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut mouse_mov: EventReader<MouseMotion>,
) {
    let mut camera_transform = camera_query.single_mut();

    for mouse_evt in mouse_mov.iter() {
        camera_transform.rotate_y(-mouse_evt.delta[0] / 1000.);
        camera_transform.rotate_local_x(-mouse_evt.delta[1] / 1000.);
    }
}

fn main() {
    let configs = configs::load_input_map();

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Configs(configs))
        .insert_resource(ActionSet(HashSet::new()))
        .add_startup_systems((setup_window, spawn_target))
        .add_systems((handle_raw_input, handle_movement, handle_camera_mov).chain())
        .run();
}
