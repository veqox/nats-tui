use clap::Parser;
use std::io::Result;

use nats_tui::{nats::client::Client, ui::app::App, Cli};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let client = Client::new(args).await.unwrap();

    App::new(4.0, 60.0).run(client).await
}
