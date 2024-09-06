use client::create_app;
use lightyear_examples_common::app::{Apps, Cli};
use lightyear_examples_common::settings::read_settings;
use lightyear_examples_common::settings::Settings;

pub fn main() {
    let cli = Cli::default();
    let settings_str = include_str!("../assets/settings.ron");
    let settings = read_settings::<Settings>(settings_str);
    let mut app = create_app();
    app.run();
}
