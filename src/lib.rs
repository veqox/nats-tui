pub mod nats;
pub mod ui;

use clap::{Parser, ValueHint};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(
        short, 
        long, 
        value_hint = ValueHint::Other, 
        value_name = "STRING",
    )]
    pub server: String,

    #[arg(
        short, 
        long, 
        value_hint = ValueHint::Username, 
        value_name = "STRING",
        default_value = "",
    )]
    pub username: String,

    #[arg(
        short, 
        long, 
        value_hint = ValueHint::Other, 
        value_name = "STRING",
        default_value = "",
    )]
    pub password: String,
}
