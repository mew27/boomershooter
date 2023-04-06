mod configs;
mod physics;
mod health;

use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI,
};

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    input::{
        keyboard::KeyboardInput,
        mouse::{self, MouseMotion},
    },
    prelude::*,
    render::{primitives::Aabb, view::RenderLayers},
    sprite::MaterialMesh2dBundle,
};
use configs::{ButtonAction, RawInputContainer};
use physics::{AA_Hitbox, Hitray, check_collision};
use health::Health;

#[derive(Component)]
struct ThreeDCamera;

#[derive(Component)]
struct TwoDCamera;

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
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let square_pos = Transform::from_xyz(0., 5., 5.);
    let square_size = Vec3 {
        x: 5.,
        y: 5.,
        z: 5.,
    };

    // square 1
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube { size: 5. }.into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: square_pos,
            ..default()
        },
        AA_Hitbox {
            origin: square_pos.translation - square_size / 2.,
            extent: square_size,
        },
        Health(50.),
    ));

    // square 2
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube { size: 5. }.into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: square_pos.with_translation(Vec3 {
                x: -10.,
                y: 5.,
                z: 5.,
            }),
            ..default()
        },
        AA_Hitbox {
            origin: square_pos.translation - square_size / 2.,
            extent: square_size,
        },
        Health(50.),
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

    let camera_initial_pos =
        Transform::from_xyz(21., 5., 6.).with_rotation(Quat::from_xyzw(0., 0.7, 0., 0.7));

    // 3D camera
    commands.spawn((
        Camera3dBundle {
            transform: camera_initial_pos.clone(),
            ..default()
        },
        ThreeDCamera,
    ));

    //2d overlay camera
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            camera: Camera {
                order: 1,
                ..default()
            },
            ..default()
        },
        RenderLayers::from_layers(&[1]),
        TwoDCamera,
    ));

    //Spawn vertical crossair
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    shape::Box::from_corners(
                        Vec3 {
                            x: -2.,
                            y: -20.,
                            z: 0.,
                        },
                        Vec3 {
                            x: 2.,
                            y: 20.,
                            z: 0.,
                        },
                    )
                    .into(),
                )
                .into(),
            material: color_materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        },
        RenderLayers::layer(1),
    ));

    //Spawn horizontal crossair
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    shape::Box::from_corners(
                        Vec3 {
                            x: -20.,
                            y: -2.,
                            z: 0.,
                        },
                        Vec3 {
                            x: 20.,
                            y: 2.,
                            z: 0.,
                        },
                    )
                    .into(),
                )
                .into(),
            material: color_materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        },
        RenderLayers::layer(1),
    ));

    // ground reference
    commands.spawn((camera_initial_pos.clone(), GroundReference));
}

fn handle_movement(
    mut action_set: ResMut<ActionSet>,
    mut camera_query: Query<&mut Transform, With<ThreeDCamera>>,
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
}

fn handle_camera_mov(
    mut camera_query: Query<&mut Transform, With<ThreeDCamera>>,
    mut mouse_mov: EventReader<MouseMotion>,
) {
    let mut camera_transform = camera_query.single_mut();

    for mouse_evt in mouse_mov.iter() {
        camera_transform.rotate_y(-mouse_evt.delta[0] / 1000.);
        camera_transform.rotate_local_x(-mouse_evt.delta[1] / 1000.);
    }
}

fn handle_fire(
    camera_query: Query<&Transform, With<ThreeDCamera>>,
    mut action_set: ResMut<ActionSet>,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let look_direction = camera_query.single().forward();
    let camera_origin = camera_query.single().translation;

    match action_set.0.take(&ButtonAction::Fire) {
        Some(_) => {
            commands.spawn(Hitray(Ray {
                origin: camera_origin,
                direction: look_direction,
            }));
        }
        _ => (),
    }
}

fn should_handle_fire(action_set: Res<ActionSet>) -> bool {
    return action_set.0.contains(&ButtonAction::Fire);
}

fn hitline_system(
    mut hitrays: Query<(Entity, &Hitray)>,
    mut hitbox_query: Query<(Entity, &AA_Hitbox, &mut Health)>,
    mut commands: Commands,
) {
    for (hitray_entity, hitray) in &hitrays {
        let mut hits = Vec::<(f32, Mut<Health>, Entity)>::new();

        for (entity, hitbox, health) in &mut hitbox_query {
            if let Some((entry_dist, _)) = check_collision(&hitbox, &hitray) {
                //println!("Enter point = {:?}", hitray.get_point(tmin));
                hits.push((entry_dist, health, entity));
                //println!("COLLISION!");
            }
        }

        hits.sort_by(|(a, _, _), (b, _, _)| a.partial_cmp(b).unwrap());

        if let Some((_, health, entity)) = hits.get_mut(0) {
            commands.entity(hitray_entity).despawn();

            health.0 -= 1.;
            if health.0 <= 0. {
                commands.entity(*entity).despawn();
            }
        }
    }
}

fn main() {
    let configs = configs::load_input_map();

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Configs(configs))
        .insert_resource(ActionSet(HashSet::new()))
        .add_startup_systems((setup_window, spawn_target))
        .add_systems(
            (
                handle_raw_input,
                handle_movement,
                handle_camera_mov,
                handle_fire.run_if(should_handle_fire),
                hitline_system
            )
                .chain(),
        )
        .run();
}
