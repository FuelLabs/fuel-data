use std::time::Duration;

use async_nats::{
    jetstream::{self, stream::LastRawMessageErrorKind},
    ConnectOptions,
};
use prost::Message;

use fuel_data_nats::NatsClient;

use fuel_node_publishing::{packets::Packet, subjects::SubjectQuery};

pub struct ArchiveNodeNatsClient {
    client: async_nats::Client,
    jetstream: jetstream::stream::Stream,
}

impl ArchiveNodeNatsClient {
    pub async fn connect() -> anyhow::Result<Self> {
        let nats_url = fuel_data_cluster::where_are::archive_nats().await;
        let nats_url = nats_url.first().unwrap();

        let user = "admin".to_owned();
        let password = dotenvy::var("ARCHIVE_NATS_ADMIN_PASSWORD")
            .expect("ARCHIVE_NATS_ADMIN_PASSWORD must be set for admin role");

        let client = ConnectOptions::with_user_and_password(user, password)
            .connection_timeout(Duration::from_secs(5))
            .max_reconnects(1)
            .connect(nats_url)
            .await?;

        let jetstream = jetstream::new(client.clone());
        let jetstream = jetstream
            .get_or_create_stream(jetstream::stream::Config {
                name: "blocks_stream".to_string(),
                subjects: vec!["blocks.>".to_string()],
                ..Default::default()
            })
            .await?;

        Ok(Self { client, jetstream })
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

    pub async fn get_last_published<Query: SubjectQuery>(
        &self,
        subject_query: &Query,
    ) -> anyhow::Result<Option<Query::DataType>> {
        tracing::info!(
            "Getting last published for {:?}",
            subject_query.to_nats_subject()
        );
        Ok(self
            .get_last_published_proto(subject_query)
            .await?
            .map(Query::DataType::from))
    }

    pub async fn get_last_published_proto<Query: SubjectQuery>(
        &self,
        subject_query: &Query,
    ) -> anyhow::Result<Option<Query::DataTypeProto>> {
        tracing::info!(
            "Getting last published proto for {:?}",
            subject_query.to_nats_subject()
        );

        let last_published = self
            .jetstream
            .get_last_raw_message_by_subject(&subject_query.to_nats_subject())
            .await;

        match last_published {
            Ok(message) => {
                let last_published_proto = Query::DataTypeProto::decode(message.payload)
                    .expect("DataTypeProto should always decode");

                Ok(Some(last_published_proto))
            }
            Err(error) => match &error.kind() {
                LastRawMessageErrorKind::NoMessageFound => Ok(None),
                _ => Err(error.into()),
            },
        }
    }
}

impl NatsClient for ArchiveNodeNatsClient {
    fn nats_client(&self) -> async_nats::Client {
        self.client.clone()
    }
}
