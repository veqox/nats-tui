use std::collections::HashMap;

use crate::Cli;
use async_nats::{Client as NatsClient, Subscriber};

use futures::StreamExt;

pub struct Client {
    active_client: String,
    active_sub: Option<String>,

    nats_clients: HashMap<String, NatsClient>,
    subscriptions: HashMap<String, Subscriber>,
}

impl Client {
    pub async fn new(args: Cli) -> Result<Self, Box<dyn std::error::Error>> {
        let client = async_nats::ConnectOptions::new()
            .user_and_password(
                args.username,
                args.password,
            )
            .connect(&args.server)
            .await?;

        Ok(Self {
            active_client: args.server.clone(),
            active_sub: None,

            nats_clients: HashMap::from([(args.server, client)]),
            subscriptions: HashMap::new(),
        })
    }

    pub async fn subscribe(&mut self, subject: String) -> Result<(), Box<dyn std::error::Error>> {
        let Some(client) = self.nats_clients.get(&self.active_client) else {
            // TODO: custom error
            return Ok(());
        };

        let subscriber = client.subscribe(subject.clone()).await?;
        self.subscriptions.insert(subject.clone(), subscriber);
        self.active_sub = Some(subject);

        Ok(())
    }

    pub async fn next_msg(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let Some(ref sub) = self.active_sub else {
            // TODO: custom error
            return Ok("".to_string());
        };

        let Some(subscriber) = self.subscriptions.get_mut(sub) else {
            // TODO: custom error
            return Ok("".to_string());
        };

        let Some(answer) = subscriber.next().await else {
            return Ok("ERROR".to_string());
        };

        Ok(answer.subject.to_string())
    }
}
