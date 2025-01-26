use std::time::Duration;

use async_nats::ConnectOptions;

use fuel_node_publishing::packets::Packet;

pub struct RelayNodeNatsClient {
    pub client: async_nats::Client,
}

impl RelayNodeNatsClient {
    pub async fn connect() -> anyhow::Result<Self> {
        let nats_url = fuel_data_cluster::where_is::relay_nats();
        let user = "admin".to_owned();
        let password = dotenvy::var("RELAY_NATS_ADMIN_PASSWORD")
            .expect("RELAY_NATS_ADMIN_PASSWORD must be set for admin role");

        let client = ConnectOptions::with_user_and_password(user, password)
            .connection_timeout(Duration::from_secs(5))
            .max_reconnects(1)
            .connect(nats_url)
            .await?;

        Ok(Self { client })
    }

    pub async fn publish<T>(&self, Packet { subject, payload }: Packet<T>) -> anyhow::Result<()>
    where
        T: prost::Message,
    {
        let mut buf = Vec::with_capacity(payload.encoded_len());
        payload.encode(&mut buf)?;

        self.client.publish(subject.to_string(), buf.into()).await?;

        Ok(())
    }
}
