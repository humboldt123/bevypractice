use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian2d::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

const BACKGROUND: Color = Color::srgb(0.298, 0.298, 0.298);

// Character attributes resource
#[derive(Resource)]
struct CharacterAttributes {
    // Movement attributes
    walk_speed: f32,
    float_height: f32,
    cling_distance: f32,
    spring_strength: f32,
    spring_dampening: f32,
    acceleration: f32,
    air_acceleration: f32,
    coyote_time: f32,
    free_fall_extra_gravity: f32,
    tilt_offset_angvel: f32,
    tilt_offset_angacl: f32,
    turning_angvel: f32,
    
    // Jump attributes
    jump_height: f32,
    jump_in_air: bool,
    upslope_extra_gravity: f32,
    takeoff_extra_gravity: f32,
    takeoff_above_velocity: f32,
    fall_extra_gravity: f32,
    shorten_extra_gravity: f32,
    peak_prevention_at_upward_velocity: f32,
    peak_prevention_extra_gravity: f32,
    input_buffer_time: f32,
}

impl Default for CharacterAttributes {
    fn default() -> Self {
        Self {
            walk_speed: 500.0,
            float_height: 31.0,
            cling_distance: 1.0,
            spring_strength: 400.0,
            spring_dampening: 1.2,
            acceleration: 1550.0,
            air_acceleration: 1550.0,
            coyote_time: 0.15,
            free_fall_extra_gravity: 60.0,
            tilt_offset_angvel: 5.0,
            tilt_offset_angacl: 500.0,
            turning_angvel: 10.0,
            
            jump_height: 150.0,
            jump_in_air: false,
            upslope_extra_gravity: 30.0,
            takeoff_extra_gravity: 500.0,
            takeoff_above_velocity: 200.0,
            fall_extra_gravity: 550.0,
            shorten_extra_gravity: 2000.0,
            peak_prevention_at_upward_velocity: 100.0,
            peak_prevention_extra_gravity: 400.0,
            input_buffer_time: 0.2,
        }
    }
}

fn ui_character_editor(
    mut contexts: EguiContexts,
    mut character_attributes: ResMut<CharacterAttributes>,
) {
    egui::Window::new("Character Editor").show(contexts.ctx_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Movement Settings");
            ui.add(egui::Slider::new(&mut character_attributes.walk_speed, 0.0..=1000.0).text("Walk Speed"));
            ui.add(egui::Slider::new(&mut character_attributes.float_height, 0.0..=400.0).text("Float Height"));
            ui.add(egui::Slider::new(&mut character_attributes.cling_distance, 0.0..=10.0).text("Cling Distance"));
            ui.add(egui::Slider::new(&mut character_attributes.spring_strength, 0.0..=1000.0).text("Spring Strength"));
            ui.add(egui::Slider::new(&mut character_attributes.spring_dampening, 0.0..=5.0).text("Spring Dampening"));
            ui.add(egui::Slider::new(&mut character_attributes.acceleration, 0.0..=3000.0).text("Acceleration"));
            ui.add(egui::Slider::new(&mut character_attributes.air_acceleration, 0.0..=3000.0).text("Air Acceleration"));
            ui.add(egui::Slider::new(&mut character_attributes.coyote_time, 0.0..=0.5).text("Coyote Time"));
            ui.add(egui::Slider::new(&mut character_attributes.free_fall_extra_gravity, 0.0..=200.0).text("Free Fall Gravity"));
            ui.add(egui::Slider::new(&mut character_attributes.tilt_offset_angvel, 0.0..=20.0).text("Tilt Offset Angular Velocity"));
            ui.add(egui::Slider::new(&mut character_attributes.tilt_offset_angacl, 0.0..=1000.0).text("Tilt Offset Angular Acceleration"));
            ui.add(egui::Slider::new(&mut character_attributes.turning_angvel, 0.0..=20.0).text("Turning Angular Velocity"));
            ui.separator();
            ui.heading("Jump Settings");
            ui.add(egui::Slider::new(&mut character_attributes.jump_height, 0.0..=300.0).text("Jump Height"));
            ui.checkbox(&mut character_attributes.jump_in_air, "Allow Jump in Air");
            ui.add(egui::Slider::new(&mut character_attributes.upslope_extra_gravity, 0.0..=4000.0).text("Upslope Gravity"));
            ui.add(egui::Slider::new(&mut character_attributes.takeoff_extra_gravity, 0.0..=4000.0).text("Takeoff Gravity"));
            ui.add(egui::Slider::new(&mut character_attributes.takeoff_above_velocity, 0.0..=4000.0).text("Takeoff Above Velocity"));
            ui.add(egui::Slider::new(&mut character_attributes.fall_extra_gravity, 0.0..=4000.0).text("Fall Gravity"));
            ui.add(egui::Slider::new(&mut character_attributes.shorten_extra_gravity, 0.0..=4000.0).text("Shorten Jump Gravity"));
            ui.add(egui::Slider::new(&mut character_attributes.peak_prevention_at_upward_velocity, 0.0..=4000.0).text("Peak Prevention Velocity"));
            ui.add(egui::Slider::new(&mut character_attributes.peak_prevention_extra_gravity, 0.0..=4000.0).text("Peak Prevention Gravity"));
            ui.add(egui::Slider::new(&mut character_attributes.input_buffer_time, 0.0..=0.5).text("Input Buffer Time"));
        });
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND))
        .insert_resource(CharacterAttributes::default())
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Platformer Demo".into(),
                    resolution: (900., 700.).into(),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian2dPlugin::new(FixedUpdate),
        ))
        .add_plugins(EguiPlugin)
        .insert_resource(Gravity(Vec2::NEG_Y * 600.0))
        .add_systems(Startup, (setup_camera, setup_level, setup_player))
        .add_systems(FixedUpdate, (player_controls,))
        .add_systems(Update, ui_character_editor)
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
        Transform::from_xyz(-200.0, -50.0, 0.0),
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
    character_attributes: Res<CharacterAttributes>,
) {
    let Ok(mut controller) = query.get_single_mut() else { return };

    let mut direction = Vec2::ZERO;
    
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: Vec3::new(direction.x * character_attributes.walk_speed, 0.0, 0.0),
        float_height: character_attributes.float_height,
        cling_distance: character_attributes.cling_distance,
        spring_strengh: character_attributes.spring_strength,
        spring_dampening: character_attributes.spring_dampening,
        acceleration: character_attributes.acceleration,
        air_acceleration: character_attributes.air_acceleration,
        coyote_time: character_attributes.coyote_time,
        free_fall_extra_gravity: character_attributes.free_fall_extra_gravity,
        tilt_offset_angvel: character_attributes.tilt_offset_angvel,
        tilt_offset_angacl: character_attributes.tilt_offset_angacl,
        turning_angvel: character_attributes.turning_angvel,
        ..default()
    });

    if keyboard.pressed(KeyCode::Space) || keyboard.pressed(KeyCode::ArrowUp) {
        controller.action(TnuaBuiltinJump {
            height: character_attributes.jump_height,
            allow_in_air: character_attributes.jump_in_air,
            upslope_extra_gravity: character_attributes.upslope_extra_gravity,
            takeoff_extra_gravity: character_attributes.takeoff_extra_gravity,
            takeoff_above_velocity: character_attributes.takeoff_above_velocity,
            fall_extra_gravity: character_attributes.fall_extra_gravity,
            shorten_extra_gravity: character_attributes.shorten_extra_gravity,
            peak_prevention_at_upward_velocity: character_attributes.peak_prevention_at_upward_velocity,
            peak_prevention_extra_gravity: character_attributes.peak_prevention_extra_gravity,
            reschedule_cooldown: None,
            input_buffer_time: character_attributes.input_buffer_time,
            ..default()
        });
    }
}