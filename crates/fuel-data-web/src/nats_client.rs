use std::time::Duration;

use async_nats::ConnectOptions;
use fuel_data_nats::NatsClient;

pub struct ProxyNatsClient {
    pub client: async_nats::Client,
    pub jetstream: async_nats::jetstream::Context,
}

impl ProxyNatsClient {
    pub async fn connect() -> anyhow::Result<Self> {
        let nats_url = dotenvy::var("NATS_URL").expect("NATS_URL must be set for admin role");

        let user = "default_user".to_owned();
        let password = "".to_owned();

        let client = ConnectOptions::with_user_and_password(user, password)
            .connection_timeout(Duration::from_secs(5))
            .max_reconnects(1)
            .connect(nats_url)
            .await?;

        let jetstream = async_nats::jetstream::new(client.to_owned());

        Ok(Self { client, jetstream })
    }
}

impl NatsClient for ProxyNatsClient {
    fn nats_client(&self) -> async_nats::Client {
        self.client.clone()
    }
}
