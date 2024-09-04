use bevy::utils::Duration;
use lightyear::prelude::Mode;
use lightyear::shared::config::SharedConfig;
use lightyear::shared::tick_manager::TickConfig;

// This is extremely important is basically the configuration for our fixedupdate
pub const FIXED_TIMESTEP_HZ: f64 = 60.0;

// Utilized to tell me how often I send packets
pub const SERVER_REPLICATION_INTERVAL: Duration = Duration::from_millis(100);

// Config that need to be the same
pub fn create_shared_config() -> SharedConfig {
    SharedConfig {
        server_replication_send_interval: SERVER_REPLICATION_INTERVAL,
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
        },
        // Meaning server and client will be separated
        mode: Mode::Separate,
    }
}
