// Creating the player entity, and entity that will have the ability to move itself that is controlled via key inputs

use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy_rapier3d::prelude::*;

use crate::form_modular_char::helpers::find_child_with_name_containing;
use crate::form_modular_char::lib::Skeleton;
use crate::treat_animations::lib::AnimatedEntity;
use crate::MyAppState;

//Player
// Marker component - Basically the rigid body that will move the player
#[derive(Component)]
pub struct Player;

// Marker just to easily check other players
#[derive(Component)]
pub struct SidePlayer;

// Amount of jumps you can have
#[derive(Reflect, Component, Debug)]
pub struct Limit {
    pub jump_limit: u8,
}

impl Default for Limit {
    fn default() -> Self {
        Self { jump_limit: 2 }
    }
}

// Kind of a simple pid
#[derive(Reflect, Component, Debug)]
pub struct PlayerVel {
    pub ang_vel: f32,
    pub linvel: f32,
    pub jump_vel: f32,
    pub dash_vel: f32,
}

impl Default for PlayerVel {
    fn default() -> Self {
        Self {
            ang_vel: 600.0,
            linvel: 20.0,
            jump_vel: 20.0,
            dash_vel: 200.0,
        }
    }
}

#[derive(Component, Reflect, Debug)]
pub struct Health(pub i8);

impl Default for Health {
    fn default() -> Self {
        Self(10)
    }
}

// Times the dash for each key
#[derive(Reflect, Component, Debug)]
pub struct DashTimers {
    pub up: Stopwatch,
    pub down: Stopwatch,
    pub left: Stopwatch,
    pub right: Stopwatch,
}

impl Default for DashTimers {
    fn default() -> Self {
        Self {
            up: Stopwatch::new(),
            down: Stopwatch::new(),
            left: Stopwatch::new(),
            right: Stopwatch::new(),
        }
    }
}

// Marker component - Tells me which is the collider to check for ground
#[derive(Component)]
pub struct PlayerGroundCollider;

// Adding physical body that will move our modular character dynamically
// Also adding usefull attachments
pub fn spawn_main_rigidbody(
    mut commands: Commands,
    mod_characters: Query<(Entity, &Name), With<Skeleton>>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
    mut my_app_state: ResMut<NextState<MyAppState>>,
) {
    for ((player_character, scene_name), player_count) in mod_characters.iter().zip(1..) {
        // Spawning main physical body
        let main_rigidbody = (
            RigidBody::Dynamic,
            SpatialBundle {
                transform: Transform::from_xyz(-4.0, 0.25, -1.0),
                ..Default::default()
            },
            Velocity::zero(),
            Damping {
                linear_damping: 0.9,
                angular_damping: 0.0,
            },
            ExternalImpulse {
                impulse: Vec3::ZERO,
                torque_impulse: Vec3::ZERO,
            },
            PlayerVel::default(),
            Name::new(format!("Player_{}", player_count)),
            GravityScale(1.0),
            AdditionalMassProperties::Mass(10.0),
            LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        );

        let second_rigidbody = (
            RigidBody::Dynamic,
            SpatialBundle {
                transform: Transform::from_xyz(4.0, 0.25, 1.0),
                ..Default::default()
            },
            Velocity::zero(),
            Damping {
                linear_damping: 0.9,
                angular_damping: 0.0,
            },
            ExternalImpulse {
                impulse: Vec3::ZERO,
                torque_impulse: Vec3::ZERO,
            },
            PlayerVel::default(),
            Name::new(format!("Player_{}", player_count)),
            GravityScale(1.0),
            AdditionalMassProperties::Mass(10.0),
            LockedAxes::ROTATION_LOCKED,
        );

        // Spawning the collider that moves and tells things
        let main_collider = (
            Collider::cuboid(0.65, 1.0, 0.25),
            CollisionGroups::new(Group::GROUP_11, Group::GROUP_10),
            ActiveEvents::COLLISION_EVENTS,
            TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)),
            PlayerGroundCollider,
        );

        let side_collider = (
            Collider::cuboid(0.65, 1.0, 0.25),
            CollisionGroups::new(Group::GROUP_11, Group::GROUP_10),
            ActiveEvents::COLLISION_EVENTS,
            TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)),
        );


        // Character health
        let health = Health::default();

        // The amount of time the player has if quickly taps to dash
        let dash_timers = DashTimers::default();

        // A few of the player limits
        let limit = Limit::default();

        if scene_name.to_string() == "skeleton_1" {
            // Main rigidbody + it is collider
            let player_details = commands
                .spawn(main_rigidbody)
                .insert(Player)
                .insert(dash_timers)
                .insert(limit)
                .insert(health)
                .with_children(|children: &mut ChildBuilder| {
                    children.spawn(main_collider);
                })
                .id();

            // Final result - Basically the skeleton created in mod char + the rigidbody that is gonna move it
            let player = commands
                .entity(player_character)
                .set_parent(player_details)
                .id();

            // Usefull marker component for treat_anim - TODO MADE IT SO I CAN EASILY SELECT THE MAIN PLAYER LATER SEPARATE THIS
            let animated_entity =
                find_child_with_name_containing(&children_entities, &names, &player, "Armature")
                    .expect("To have entity with animation");
            commands.entity(animated_entity).insert(AnimatedEntity);
        } else {
            // Main rigidbody + it is collider + a simple marker component saying is not main player
            let other_details = commands
                .spawn(second_rigidbody)
                .insert(SidePlayer)
                .insert(dash_timers)
                .insert(limit)
                .insert(health)
                .with_children(|children: &mut ChildBuilder| {
                    children.spawn(side_collider);
                })
                .id();

            commands.entity(player_character).set_parent(other_details);
        }
    }
    my_app_state.set(MyAppState::PlayerCreated);
    my_app_state.set(MyAppState::InGame);
}
