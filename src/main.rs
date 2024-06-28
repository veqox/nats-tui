use std::collections::HashSet;

use clap::Parser;
use futures::stream::StreamExt;

#[derive(clap::Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    server: String,
    #[arg(short, long)]
    user: Option<String>,
    #[arg(short, long)]
    password: Option<String>,
    #[arg(short, long)]
    subject: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let client = async_nats::ConnectOptions::new()
        .user_and_password(
            args.user.unwrap_or("".to_string()),
            args.password.unwrap_or("".to_string()),
        )
        .connect(args.server)
        .await
        .unwrap();

    let mut subscriber = client
        .subscribe(args.subject.unwrap_or(">".to_string()))
        .await
        .unwrap();

    let mut subjects = HashSet::new();

    while let Some(msg) = subscriber.next().await {
        if subjects.insert(msg.subject.to_string()) {
            println!("Subject: {}", msg.subject);
        }
    }
}
