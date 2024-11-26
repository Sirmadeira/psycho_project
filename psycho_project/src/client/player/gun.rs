use crate::shared::protocol::weapon_structs::BulletMarker;
use crate::shared::shared_gun::shared_spawn_bullet;
use crate::shared::shared_physics::BulletPhysics;
use crate::shared::shared_physics::InputPhysicsSet;
use avian3d::prelude::Collider;
use bevy::prelude::*;
use lightyear::client::prediction::plugin::is_in_rollback;
use lightyear::client::prediction::Predicted;
/// Responsible for gun related mechanics
pub struct PlayerGunPlugin;

impl Plugin for PlayerGunPlugin {
    fn build(&self, app: &mut App) {
        // Done so it avoid double bullet spawn else server spawns it
        app.add_systems(
            FixedUpdate,
            shared_spawn_bullet
                .run_if(not(is_in_rollback))
                .in_set(InputPhysicsSet::Input),
        );
        app.add_systems(Update, add_bullet_physics);
    }
}

fn add_bullet_physics(
    mut commands: Commands,
    mut bullet_query: Query<Entity, (With<BulletMarker>, Added<Predicted>, Without<Collider>)>,
) {
    for entity in bullet_query.iter_mut() {
        info!("Adding physics to a replicated bullet:  {entity:?}");
        commands.entity(entity).insert(BulletPhysics::default());
    }
}
