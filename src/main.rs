use clap::Parser;

use nats_tui::{
    Cli,
    nats::client::Client,
    ui::app::App,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let client = Client::new(args).await?;

    App::new(4.0, 60.0)
        .run(client)
        .await
        .unwrap();

    Ok(())
}
