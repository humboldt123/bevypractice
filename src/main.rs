use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian2d::*;

const BACKGROUND: Color = Color::srgb(0.298, 0.298, 0.298);

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Platformer Demo".into(),
                    resolution: (800., 600.).into(),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian2dPlugin::new(FixedUpdate),
        ))
        .insert_resource(Gravity(Vec2::NEG_Y * 600.0))
        .add_systems(Startup, (setup_camera, setup_level, setup_player))
        .add_systems(FixedUpdate, (player_controls,))
        .run();
}

#[derive(Component)]
struct Player;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_level(mut commands: Commands) {
    // Ground
    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.7),
            custom_size: Some(Vec2::new(800.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -250.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(800.0, 50.0),
        Visibility::default(),
        ViewVisibility::default(),
        InheritedVisibility::default(),
    ));

    // Platform
    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.7),
            custom_size: Some(Vec2::new(200.0, 30.0)),
            ..default()
        },
        Transform::from_xyz(-200.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(200.0, 30.0),
        Visibility::default(),
        ViewVisibility::default(),
        InheritedVisibility::default(),
    ));
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.8, 0.3),
            custom_size: Some(Vec2::new(30.0, 60.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Player,
        RigidBody::Dynamic,
        Collider::rectangle(30.0, 60.0),
        TnuaController::default(),
        TnuaAvian2dSensorShape(Collider::rectangle(29.0, 0.0)),
        LockedAxes::ROTATION_LOCKED,
        Visibility::default(),
        ViewVisibility::default(),
        InheritedVisibility::default(),
    ));
}

fn player_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut TnuaController, With<Player>>,
) {
    let Ok(mut controller) = query.get_single_mut() else { return };

    let mut direction = Vec2::ZERO;
    
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 30.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        direction.x += 30.0;
    }

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: Vec3::new(direction.x * 500.0, 0.0, 0.0),
        spring_strengh: 50.0,
        acceleration: 500.0,
        float_height: 31.0,
        ..default()
    });

    if keyboard.pressed(KeyCode::Space) {
        controller.action(TnuaBuiltinJump {
            height: 100.0,
            ..default()
        });
    }
}