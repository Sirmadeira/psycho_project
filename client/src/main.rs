use crate::client::ExampleClientPlugin;
use crate::server::ExampleServerPlugin;
use crate::shared::SharedPlugin;
use common::app::{Apps, Cli};
use common::settings::{read_settings, Settings};

mod client;
mod server;
mod shared;

fn main() {
    let cli = Cli::default();
    // Commong config settings being disdposed
    let settings_str = include_str!("../assets/settings.ron");
    let settings = read_settings::<Settings>(settings_str);
    let mut apps = Apps::new(settings, cli);
    // Adding multipler lightyear plugins
    apps.add_lightyear_plugins()
        // add our plugins
        .add_user_plugins(ExampleClientPlugin, ExampleServerPlugin, SharedPlugin);

    // run the app
    apps.run();
}
