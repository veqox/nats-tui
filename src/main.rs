use std::io::Result;
use clap::Parser;

use nats_tui::{
    Cli,
    nats::client::Client,
    ui::app::App,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let client = Client::new(args).await.unwrap();

    App::new(4.0, 60.0)
        .run(client)
        .await
}
