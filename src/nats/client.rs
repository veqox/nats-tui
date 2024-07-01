use crate::Cli;
use async_nats::{Client as NatsClient, Message, Subscriber};

use futures::StreamExt;

pub struct Client {
    client: NatsClient,
    subscription: Option<Subscriber>,
}

impl Client {
    pub async fn new(args: Cli) -> Result<Self, Box<dyn std::error::Error>> {
        let client = async_nats::ConnectOptions::new()
            .user_and_password(args.username, args.password)
            .connect(&args.server)
            .await?;

        Ok(Self {
            client,
            subscription: None,
        })
    }

    pub async fn subscribe(&mut self, subject: String) -> Result<(), Box<dyn std::error::Error>> {
        let subscriber = self.client.subscribe(subject.clone()).await?;

        self.subscription = Some(subscriber);

        Ok(())
    }

    pub async fn next_msg(&mut self) -> Result<Message, Box<dyn std::error::Error>> {
        let Some(subscriber) = self.subscription.as_mut() else {
            // TODO: custom error
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No subscription",
            )));
        };

        let Some(answer) = subscriber.next().await else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No message",
            )));
        };

        Ok(answer)
    }
}
