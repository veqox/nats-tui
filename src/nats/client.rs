use std::collections::HashMap;

use crate::Cli;
use async_nats::{Client as NatsClient, Subscriber};

pub struct Client {
    nats_clients: HashMap<String, NatsClient>,
    subscriptions: HashMap<String, Subscriber>,
}

impl Client {
    pub async fn new(args: Cli) -> Self {
        let client = async_nats::ConnectOptions::new()
            .user_and_password(
                args.username,
                args.password,
            )
            .connect(&args.server)
            .await
            .unwrap();

        Self {
            nats_clients: HashMap::from([(args.server, client)]),
            subscriptions: HashMap::new(),
        }
    }
}
