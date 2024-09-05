use clap::Parser;
use client::create_app;
use client::Cli;

pub fn main() {
    let cli = Cli::parse();
    let mut app = create_app(cli);
    app.run();
}
