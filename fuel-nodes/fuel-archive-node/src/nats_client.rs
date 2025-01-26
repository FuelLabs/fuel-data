use std::time::Duration;

use async_nats::{
    jetstream::{self, stream::LastRawMessageErrorKind},
    ConnectOptions,
};
use prost::Message;

use fuel_node_publishing::{packets::Packet, subjects::SubjectQuery};

// TODO: Cache what we can
// TODO: Add Pooling for NATS connections
pub struct ArchiveNodeNatsClient;

impl ArchiveNodeNatsClient {
    pub async fn publish<T>(Packet { subject, payload }: Packet<T>) -> anyhow::Result<()>
    where
        T: prost::Message,
    {
        let mut buf = Vec::with_capacity(payload.encoded_len());
        payload.encode(&mut buf)?;

        Self::client()
            .await?
            .publish(subject.to_string(), buf.into())
            .await?;

        Ok(())
    }

    pub async fn get_last_published<Query: SubjectQuery>(
        subject_query: &Query,
    ) -> anyhow::Result<Option<Query::DataType>> {
        tracing::info!(
            "Getting last published for {:?}",
            subject_query.to_nats_subject()
        );
        Ok(Self::get_last_published_proto(subject_query)
            .await?
            .map(Query::DataType::from))
    }

    pub async fn get_last_published_proto<Query: SubjectQuery>(
        subject_query: &Query,
    ) -> anyhow::Result<Option<Query::DataTypeProto>> {
        tracing::info!(
            "Getting last published proto for {:?}",
            subject_query.to_nats_subject()
        );

        let last_published = Self::jetstream(&Self::client().await?)
            .await?
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

    async fn client() -> anyhow::Result<async_nats::Client> {
        let archive_nats_url = fuel_data_cluster::where_are::latest_archive_nats().await;
        let user = "admin".to_owned();
        let password = dotenvy::var("ARCHIVE_NATS_ADMIN_PASSWORD")
            .expect("ARCHIVE_NATS_ADMIN_PASSWORD must be set for admin role");

        Ok(ConnectOptions::with_user_and_password(user, password)
            .connection_timeout(Duration::from_secs(5))
            .max_reconnects(1)
            .connect(archive_nats_url)
            .await?)
    }

    async fn jetstream(client: &async_nats::Client) -> anyhow::Result<jetstream::stream::Stream> {
        let jetstream = jetstream::new(client.clone());
        let jetstream = jetstream
            .get_or_create_stream(jetstream::stream::Config {
                name: "blocks_stream".to_string(),
                subjects: vec!["blocks.>".to_string()],
                ..Default::default()
            })
            .await?;

        Ok(jetstream)
    }
}
