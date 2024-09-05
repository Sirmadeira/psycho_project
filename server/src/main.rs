use clap::Parser;
use server::create_app;
use server::Cli;

fn main() {
    let cli = Cli::parse();
    let mut app = create_app(cli);
    app.run();
}
