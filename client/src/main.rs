use crate::client::ExampleClientPlugin;
use crate::server::ExampleServerPlugin;
use crate::shared::SharedPlugin;
use common::app::{Apps, Cli};
use common::settings::{read_settings, Settings};

mod client;
mod protocol;
mod server;
mod shared;

fn main() {
    let cli = Cli::default();
    let settings_str = include_str!("../assets/settings.ron");
    let settings = read_settings::<Settings>(settings_str);
    // build the bevy app (this adds common plugin such as the DefaultPlugins)
    // and returns the `ClientConfig` and `ServerConfig` so that we can modify them if needed
    let mut apps = Apps::new(settings, cli);
    // add the `ClientPlugins` and `ServerPlugins` plugin groups
    apps.add_lightyear_plugins()
        // add our plugins
        .add_user_plugins(ExampleClientPlugin, ExampleServerPlugin, SharedPlugin);
    // run the app
    apps.run();
}
