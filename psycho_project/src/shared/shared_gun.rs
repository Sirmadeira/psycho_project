use super::protocol::lobby_structs::Lobbies;
use crate::shared::protocol::player_structs::*;
use crate::shared::protocol::weapon_structs::*;
use crate::shared::shared_physics::InputPhysicsSet;
use crate::shared::shared_physics::PhysicsBundle;
use crate::shared::shared_physics::REPLICATION_GROUP;
use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use lightyear::client::prediction::prespawn::PreSpawnedPlayerObject;
use lightyear::connection::id::ClientId;
use lightyear::prelude::client::Predicted;
use lightyear::prelude::server::Replicate;
use lightyear::prelude::server::SyncTarget;
use lightyear::shared::plugin::NetworkIdentity;
use lightyear::shared::replication::components::ReplicationTarget;
use lightyear::shared::replication::network_target::NetworkTarget;
use lightyear::shared::tick_manager::TickManager;

pub struct SharedGunPlugin;

impl Plugin for SharedGunPlugin {
    fn build(&self, app: &mut App) {
        // Registerign bullet hit event
        app.add_event::<BulletHitEvent>();
        // Fixed update because physics related also needs to occur in input
        app.add_systems(
            FixedUpdate,
            (process_collisions, lifetime_despawner).in_set(InputPhysicsSet::Input),
        );
    }
}

/// A shared system generates these events on server and client.
/// On the server, we use them to manipulate player scores;
/// On the clients, we just use them for visual effects.
#[derive(Event, Debug)]
pub struct BulletHitEvent {
    pub bullet_owner: ClientId,
    /// if it struck a player, this is their clientid:
    pub victim_client_id: Option<ClientId>,
    pub position: Vec3,
}

/// Responsible for spawning predicted bullets both in client and in server
pub fn shared_spawn_bullet(
    mut query: Query<
        (
            &Position,
            &Rotation,
            &LinearVelocity,
            &PlayerId,
            &ActionState<CharacterAction>,
            &mut Weapon,
        ),
        Or<(With<Predicted>, With<ReplicationTarget>)>,
    >,
    tick_manager: Res<TickManager>,
    lobbies: Res<Lobbies>,
    mut commands: Commands,
    identity: NetworkIdentity,
) {
    // If there is no entity no need for this system to be enabled
    if query.is_empty() {
        return;
    }
    // Current tick
    let current_tick = tick_manager.tick();

    for (player_position, player_rotation, player_velocity, player_id, action_state, mut weapon) in
        query.iter_mut()
    {
        if !action_state.just_pressed(&CharacterAction::Shoot) {
            continue;
        }
        // Tick difference between weapon and current tick
        let tick_diff = weapon.last_fire_tick - current_tick;

        // Checking if weapon
        if tick_diff.abs() <= weapon.cooldown as i16 {
            // Here he cant technically fire for now as he is in cooldown
            if weapon.last_fire_tick == current_tick {
                info!("Player cant fire for now, as he is firing in same tick")
            }
            continue;
        }

        let prev_last_fire_tick = weapon.last_fire_tick;
        weapon.last_fire_tick = current_tick;

        let bullet_spawn_offset = Vec3::new(0.0, 0.5, 2.0);
        let bullet_origin = player_position.0 + bullet_spawn_offset;
        let bullet_linvel = player_rotation * (Vec3::Z * weapon.bullet_speed) + player_velocity.0;

        // We do this to avo
        let prespawned = PreSpawnedPlayerObject::default_with_salt(player_id.0.to_bits());

        let bullet_entity = commands
            .spawn((
                BulletBundle::new(player_id.0, bullet_origin, bullet_linvel, current_tick),
                PhysicsBundle::bullet(),
                prespawned,
            ))
            .id();
        // info!(
        //     "Spawned bullet for ActionState, bullet={bullet_entity:?} ({}, {}). prev last_fire tick: {prev_last_fire_tick:?}",
        //     weapon.last_fire_tick.0, player_id.0
        // );
        if identity.is_server() {
            // info!("Replicating bullet for others in lobbies");
            let replicate = Replicate {
                sync: SyncTarget {
                    prediction: NetworkTarget::Only(lobbies.lobbies[0].players.clone()),
                    ..Default::default()
                },
                // make sure that all entities that are predicted are part of the same replication group
                group: REPLICATION_GROUP,
                ..default()
            };
            commands.entity(bullet_entity).insert(replicate);
        }
    }
}

/// THE EXTERMINATOR OF BULLETS
pub fn lifetime_despawner(
    q: Query<(Entity, &Lifetime)>,
    tick_manager: Res<TickManager>,
    identity: NetworkIdentity,
    mut commands: Commands,
) {
    for (entity, lifetime) in q.iter() {
        // Evaluates if tick is way further that lifetime available
        if (tick_manager.tick() - lifetime.origin_tick) > lifetime.lifetime {
            if identity.is_server() {
                info!("Despawning entity {} in server", entity);
                // Stop replicating and despawn it if server
                commands.entity(entity).remove::<Replicate>().despawn();
            } else {
                info!("Despawning entity {} in client", entity);
                // Despanw every child entity in client
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

/// Process all type of collisions warning this is gonna be a biggie
pub fn process_collisions(
    mut collision_event_reader: EventReader<Collision>,
    bullet_q: Query<(&BulletMarker, &Position)>,
    player_q: Query<&PlayerId>,
    identity: NetworkIdentity,
    mut commands: Commands,
    mut hit_ev_writer: EventWriter<BulletHitEvent>,
) {
    for collision in collision_event_reader.read() {
        let contact = &collision.0;
        if let Ok((bullet, bullet_pos)) = bullet_q.get(contact.entity1) {
            // despawn the bullet
            if identity.is_server() {
                commands
                    .entity(contact.entity1)
                    .remove::<Replicate>()
                    .despawn();
            } else {
                commands.entity(contact.entity1).despawn_recursive();
            }

            if let Ok(victim_client_id) = player_q.get(contact.entity2) {
                info!("There was a victime {}", victim_client_id.0);
                let ev = BulletHitEvent {
                    bullet_owner: bullet.owner,
                    victim_client_id: Some(victim_client_id.0),
                    position: bullet_pos.0,
                };
                hit_ev_writer.send(ev);
            } else {
                info!("No victim");

                let ev = BulletHitEvent {
                    bullet_owner: bullet.owner,
                    victim_client_id: None,
                    position: bullet_pos.0,
                };
                hit_ev_writer.send(ev);
            }
        }
        // Twice because collisions are vice versa sometimes
        if let Ok((bullet, bullet_pos)) = bullet_q.get(contact.entity2) {
            // despawn the bullet
            if identity.is_server() {
                commands
                    .entity(contact.entity2)
                    .remove::<Replicate>()
                    .despawn();
            } else {
                commands.entity(contact.entity2).despawn_recursive();
            }

            if let Ok(victim_client_id) = player_q.get(contact.entity1) {
                info!("There was a victim {}", victim_client_id.0);
                let ev = BulletHitEvent {
                    bullet_owner: bullet.owner,
                    victim_client_id: Some(victim_client_id.0),
                    position: bullet_pos.0,
                };
                hit_ev_writer.send(ev);
            } else {
                info!("No victim");
                let ev = BulletHitEvent {
                    bullet_owner: bullet.owner,
                    victim_client_id: None,
                    position: bullet_pos.0,
                };
                hit_ev_writer.send(ev);
            };
        }
    }
}
