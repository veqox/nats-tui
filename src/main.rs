use clap::Parser;

use nats_tui::{
    Cli,
    nats::{
        client::Client,
    },
    ui::{
        renderer::render_loop,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let client = Client::new(args).await?;

    render_loop(client).await?;

    Ok(())
}
