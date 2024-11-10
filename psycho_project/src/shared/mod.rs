use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_screen_diagnostics::*;
use lightyear::client::prediction::diagnostics::PredictionDiagnosticsPlugin;
use lightyear::transport::io::IoDiagnosticsPlugin;
use shared_physics::SharedPhysicsPlugin;
pub mod protocol;
pub mod shared_physics;

use self::protocol::ProtocolPlugin;

/// In this plugin you should add all systems/plugins that need to exist both in server and in client
/// Worth noting that most input logic should be here, as you move something in client you should also move in server. When doing client side prediction
#[derive(Clone)]
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        // Imported plugins
        // Self made plugins
        app.add_plugins(ProtocolPlugin);
        app.add_plugins(ScreenDiagnosticsPlugin::default());
        app.add_systems(Startup, setup_diagnostic);
        app.add_plugins(SharedPhysicsPlugin);
    }
}

fn setup_diagnostic(mut onscreen: ResMut<ScreenDiagnostics>) {
    onscreen
        .add(
            "Rollbacks".to_string(),
            PredictionDiagnosticsPlugin::ROLLBACKS,
        )
        .aggregate(Aggregate::Value)
        .format(|v| format!("{v:.0}"));
    onscreen
        .add(
            "Rollback tick".to_string(),
            PredictionDiagnosticsPlugin::ROLLBACK_TICKS,
        )
        .aggregate(Aggregate::Value)
        .format(|v| format!("{v:.0}"));
    onscreen
        .add(
            "Rollback depth".to_string(),
            PredictionDiagnosticsPlugin::ROLLBACK_DEPTH,
        )
        .aggregate(Aggregate::Value)
        .format(|v| format!("{v:.1}"));
    // screen diagnostics twitches due to layout change when a metric adds or removes
    // a digit, so pad these metrics to 3 digits.
    onscreen
        .add("KB_in".to_string(), IoDiagnosticsPlugin::BYTES_IN)
        .aggregate(Aggregate::Average)
        .format(|v| format!("{v:0>3.0}"));
    onscreen
        .add("KB_out".to_string(), IoDiagnosticsPlugin::BYTES_OUT)
        .aggregate(Aggregate::Average)
        .format(|v| format!("{v:0>3.0}"));

    onscreen
        .add("FPS".to_string(), FrameTimeDiagnosticsPlugin::FPS)
        .aggregate(Aggregate::Value)
        .format(|v| format!("{v:.0}"));

    onscreen
        .add(
            "ms/frame".to_string(),
            FrameTimeDiagnosticsPlugin::FRAME_TIME,
        )
        .aggregate(Aggregate::MovingAverage(5))
        .format(|v| format!("{v:.2}"));

    onscreen
        .add(
            "Entity amount".to_string(),
            EntityCountDiagnosticsPlugin::ENTITY_COUNT,
        )
        .aggregate(Aggregate::Value)
        .format(|v| format!("{v:.0}"));
}
