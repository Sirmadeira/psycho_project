// Creating the player entity, and entity that will have the ability to move itself that is controlled via key inputs

use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy_rapier3d::prelude::*;

use crate::form_modular_char::lib::*;
use crate::treat_animations::lib::AnimatedEntity;
use crate::form_modular_char::helpers::find_child_with_name_containing;


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

#[derive(Component, Reflect, Debug)]
pub struct Health(pub i8);

// Kind of a simple pid
#[derive(Reflect, Component, Debug)]
pub struct PdInfo {
    pub kp: f32,
}

// Times the dash for each key
#[derive(Reflect, Component, Debug)]
pub struct Timers {
    pub up: Stopwatch,
    pub down: Stopwatch,
    pub left: Stopwatch,
    pub right: Stopwatch,
}

// Adding physical body that will move our modular character dynamically
// Also adding usefull attachments
pub fn spawn_main_rigidbody(
    mut commands: Commands,
    mod_characters: Query<(Entity, &Name), With<Skeleton>>,
    children_entities: Query<&Children>,
    names: Query<&Name>,
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
            PdInfo { kp: 500.0 },
            Name::new(format!("Player_{}", player_count)),
            GravityScale(1.0),
            AdditionalMassProperties::Mass(10.0),
            LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        );

        let second_rigidbody = (
            RigidBody::Fixed,
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
            PdInfo { kp: 500.0 },
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

        let health = Health(10);

        let timers = (Timers {
            up: Stopwatch::new(),
            down: Stopwatch::new(),
            left: Stopwatch::new(),
            right: Stopwatch::new(),
        },);

        let limit = (Limit {
            ..Default::default()
        },);

        if scene_name.to_string() == "skeleton_1" {
            // Main rigidbody + it is collider
            let player_details = commands
                .spawn(main_rigidbody)
                .insert(Player)
                .insert(timers)
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
                .insert(timers)
                .insert(limit)
                .insert(health)
                .with_children(|children: &mut ChildBuilder| {
                    children.spawn(main_collider);
                })
                .id();

            commands.entity(player_character).set_parent(other_details);
        }
    }
}
