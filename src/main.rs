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
    prelude::*, render::{primitives::Aabb, view::RenderLayers}, sprite::MaterialMesh2dBundle, core_pipeline::clear_color::ClearColorConfig,
};
use configs::{ButtonAction, RawInputContainer};

#[derive(Component)]
struct Hitbox;

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
    mut color_materials : ResMut<Assets<ColorMaterial>>
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
    
    // 3D camera
    commands.spawn((Camera3dBundle {
            transform: camera_initial_pos.clone(),
            ..default()
        },
        ThreeDCamera
    ));

    //2d overlay camera
    commands.spawn((
        Camera2dBundle {
            camera_2d : Camera2d {
                clear_color : ClearColorConfig::None
            },
            camera : Camera {
                order : 1,
                ..default()
            },
            ..default()
        },
        RenderLayers::from_layers(&[1]),
        TwoDCamera
    ));

    //Spawn vertical crossair
    commands.spawn((
    MaterialMesh2dBundle {
        mesh : meshes.add(shape::Box::from_corners(Vec3 { x : -2., y : -20., z : 0.}, Vec3 { x: 2., y: 20., z: 0. }).into()).into(),
        material : color_materials.add(ColorMaterial::from(Color::WHITE)),
        ..default()
    },
    RenderLayers::layer(1)
    ));

    //Spawn horizontal crossair
    commands.spawn((
    MaterialMesh2dBundle {
        mesh : meshes.add(shape::Box::from_corners(Vec3 { x : -20., y : -2., z : 0.}, Vec3 { x: 20., y: 2., z: 0. }).into()).into(),
        material : color_materials.add(ColorMaterial::from(Color::WHITE)),
        ..default()
    },
    RenderLayers::layer(1)
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
    hitbox_query : Query<(Entity, &Transform), With<Hitbox>>,
    camera_query : Query<&Transform, With<ThreeDCamera>>,
    mut action_set   : ResMut<ActionSet>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let look_direction = camera_query.single().forward();
    let camera_origin  = camera_query.single().translation;

    let hitray = Ray {origin : camera_origin, direction : look_direction};

    match action_set.0.take(&ButtonAction::Fire) {
        Some(_) => {
            //Hitbox checking
            
            for (entity, hitbox) in hitbox_query.iter() {
                let hitbox_size   = Vec3 {x: 2.5, y: 2.5, z: 2.5};
                let hitbox_pos    = hitbox.translation;
                let hitbox_minmax = (hitbox_pos - hitbox_size, hitbox_pos + hitbox_size);
                
                println!("hitbox_minmax = {:?}", hitbox_minmax);
                println!("hitbox_pos    = {:?}", hitbox_pos);
                println!("hitray        = {:?}", (hitray));

                let (mut tmin, mut tmax) = (- f32::INFINITY, f32::INFINITY);

                // x slab collision
                if hitray.direction.x != 0. {
                    let tx1 = (hitbox_minmax.0.x - hitray.origin.x) / hitray.direction.x;
                    let tx2 = (hitbox_minmax.1.x - hitray.origin.x) / hitray.direction.x;

                    tmin = tmin.max(tx1.min(tx2));
                    tmax = tmax.min(tx1.max(tx2));
                }

                //y slab collision
                if hitray.direction.y != 0. {
                    let ty1 = (hitbox_minmax.0.y - hitray.origin.y) / hitray.direction.y;
                    let ty2 = (hitbox_minmax.1.y - hitray.origin.y) / hitray.direction.y;

                    tmin = tmin.max(ty1.min(ty2)); 
                    tmax = tmax.min(ty1.max(ty2));
                }

                //z slab collision
                if hitray.direction.z != 0. {
                    let tz1 = (hitbox_minmax.0.z - hitray.origin.z) / hitray.direction.z;
                    let tz2 = (hitbox_minmax.1.z - hitray.origin.z) / hitray.direction.z;

                    tmin = tmin.max(tz1.min(tz2));
                    tmax = tmax.min(tz1.max(tz2));
                }

                if tmax >= tmin {
                    println!("Enter point = {:?}", hitray.get_point(tmin));
                }
            }
        },
        _ => ()
    }
    //const HITSCAN_SIZE: f32 = 50.;

    //commands.spawn(PbrBundle {
        //mesh: meshes.add(
             //shape::Cylinder {
                 //radius: 0.05,
                 //height: HITSCAN_SIZE,
                 //..default()
             //}
             //.into(),
         //),
        //material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        //transform: Transform {
             //translation: camera_origin
                 //+ (look_direction * HITSCAN_SIZE / 2.),
             //..default()
        //}
        //.looking_to(camera_query.single().down(), camera_query.single().forward()),
         ////visibility: Visibility::Hidden,
        //..default()
    //});    
}

fn should_check_collision (
    action_set : Res<ActionSet>
) -> bool {
    return action_set.0.contains(&ButtonAction::Fire);
}

fn main() {
    let configs = configs::load_input_map();

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Configs(configs))
        .insert_resource(ActionSet(HashSet::new()))
        .add_startup_systems((setup_window, spawn_target))
        .add_systems((handle_raw_input, handle_movement, handle_camera_mov, handle_fire.run_if(should_check_collision)).chain())
        .run();
}
