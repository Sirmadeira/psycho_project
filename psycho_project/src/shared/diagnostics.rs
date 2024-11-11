use bevy::prelude::*;
use bevy_screen_diagnostics::*;
use lightyear::client::prediction::diagnostics::PredictionDiagnosticsPlugin;
use lightyear::transport::io::IoDiagnosticsPlugin;

/// Simple plugin utilized to show me some important info
pub struct CentralDiagnosticsPlugin;

impl Plugin for CentralDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ScreenDiagnosticsPlugin::default());
        app.add_plugins(ScreenEntityDiagnosticsPlugin);
        app.add_plugins(ScreenFrameDiagnosticsPlugin);
        app.add_systems(Startup, setup_diagnostic);
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
}
