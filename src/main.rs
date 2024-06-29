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
    let args = Cli::try_parse()?;
    let _client = Client::new(args);

    render_loop()?;

    Ok(())
}
