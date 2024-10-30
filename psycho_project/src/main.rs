use crate::client::ExampleClientPlugin;
use crate::server::ExampleServerPlugin;
use crate::shared::SharedPlugin;
use common::app::{Apps, Cli};
use common::settings::{read_settings, Settings};
use serde::{Deserialize, Serialize};

mod client;
mod server;
mod shared;

fn main() {
    let cli = Cli::default();

    // Commong config settings for our server and client basically a shit ton of constants
    let settings_str = include_str!("../assets/settings.ron");

    let settings = read_settings::<MySettings>(settings_str);
    let mut apps = Apps::new(settings.common, cli);

    apps.update_lightyear_client_config(|config| {
        config.prediction.minimum_input_delay_ticks = settings.input_delay_ticks;
        config.prediction.correction_ticks_factor = settings.correction_ticks_factor;
    });
    // Adding multipler lightyear plugins
    apps.add_lightyear_plugins()
        // add our plugins
        .add_user_plugins(ExampleClientPlugin, ExampleServerPlugin, SharedPlugin);

    // run the app
    apps.run();
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MySettings {
    pub common: Settings,

    /// By how many ticks an input press will be delayed?
    /// This can be useful as a tradeoff between input delay and prediction accuracy.
    /// If the input delay is greater than the RTT, then there won't ever be any mispredictions/rollbacks.
    /// See [this article](https://www.snapnet.dev/docs/core-concepts/input-delay-vs-rollback/) for more information.
    pub input_delay_ticks: u16,

    /// If visual correction is enabled, we don't instantly snapback to the corrected position
    /// when we need to rollback. Instead we interpolated between the current position and the
    /// corrected position.
    /// This controls the duration of the interpolation; the higher it is, the longer the interpolation
    /// will take
    pub correction_ticks_factor: f32,
}
