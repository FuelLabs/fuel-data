// lib.rs
pub mod errors;

use async_nats::ConnectOptions;
use errors::{EdgeNatsError, SubscriptionError};
use fuel_data_subjects::SubjectFilter;
use futures::stream::{self, BoxStream, StreamExt, TryStreamExt};
use std::time::Duration;
use tracing::{debug, error, info, warn};

const CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_RECONNECTS: usize = 3;
const RELAY_USER: &str = "default_user";
const ARCHIVE_USER: &str = "admin";

pub struct EdgeNatsClient;

impl EdgeNatsClient {
    /// Subscribes to the latest messages from the relay NATS server.
    pub async fn subscribe_to_latest(
        subject_filter: impl SubjectFilter,
    ) -> Result<async_nats::Subscriber, SubscriptionError> {
        let subject = subject_filter.to_nats_subject_filter();
        debug!("Subscribing to latest messages for subject: {}", &subject);

        Self::relay_nats_client()
            .await?
            .subscribe(subject.to_string())
            .await
            .map_err(|error| SubscriptionError::SubscribeError {
                subject,
                error: Box::new(error),
            })
    }

    pub async fn subscribe(
        subject_filter: impl SubjectFilter,
    ) -> Result<BoxStream<'static, Result<async_nats::Message, SubscriptionError>>, SubscriptionError>
    {
        let subject = subject_filter.to_nats_subject_filter();
        info!("Creating subscription for subject: {}", subject);

        let mut archive_streams = Self::create_archive_streams(&subject).await?;
        let relay_stream = Self::create_relay_stream(&subject).await?;

        Ok(Box::pin(
            archive_streams
                .drain(..)
                .reduce(|acc, stream| Box::pin(acc.chain(stream)))
                .unwrap_or_else(|| stream::empty().boxed())
                .chain(relay_stream),
        ))
    }

    async fn relay_nats_client() -> Result<async_nats::Client, SubscriptionError> {
        let nats_url = fuel_data_cluster::where_is::relay_nats();
        debug!("Connecting to relay NATS server at: {}", nats_url);

        Self::create_client(&nats_url, RELAY_USER.to_owned(), String::new())
            .await
            .map_err(|error| {
                error!("Failed to connect to relay NATS server: {}", error);
                SubscriptionError::ConnectionFailure {
                    msg: "Relay NATS connection failed".into(),
                    source: Some(error),
                }
            })
    }

    async fn archive_nats_clients() -> Result<Vec<async_nats::Client>, SubscriptionError> {
        let archive_urls = fuel_data_cluster::where_are::archive_nats().await;
        info!("Connecting to {} archive NATS servers", archive_urls.len());

        let password = dotenvy::var("ARCHIVE_NATS_ADMIN_PASSWORD").map_err(|_| {
            SubscriptionError::MissingCredentials("ARCHIVE_NATS_ADMIN_PASSWORD".into())
        })?;

        let connect_futures = archive_urls
            .iter()
            .map(|url| Self::create_client(url, ARCHIVE_USER.to_owned(), password.clone()));

        let results = futures::future::join_all(connect_futures).await;
        let total = results.len();
        let clients: Vec<_> = results.into_iter().filter_map(Result::ok).collect();
        let connected = clients.len();

        if connected < total {
            warn!(
                "Only connected to {}/{} archive NATS servers",
                connected, total
            );
            Err(SubscriptionError::PartialConnectionFailure { connected, total })
        } else {
            Ok(clients)
        }
    }

    async fn create_client(
        url: &str,
        user: String,
        password: String,
    ) -> Result<async_nats::Client, EdgeNatsError> {
        Ok(ConnectOptions::with_user_and_password(user, password)
            .connection_timeout(CONNECTION_TIMEOUT)
            .max_reconnects(MAX_RECONNECTS)
            .connect(url)
            .await?)
    }

    async fn create_archive_streams(
        subject: &str,
    ) -> Result<
        Vec<BoxStream<'static, Result<async_nats::Message, SubscriptionError>>>,
        SubscriptionError,
    > {
        let clients = Self::archive_nats_clients().await?;

        let streams = futures::future::join_all(clients.into_iter().map(|client| async move {
            let subscription = client
                .subscribe(subject.to_string())
                .await
                .map_err(|error| SubscriptionError::SubscribeError {
                    subject: subject.to_string(),
                    error: Box::new(error),
                })?;

            let stream: BoxStream<'static, Result<async_nats::Message, SubscriptionError>> =
                subscription
                    .map(Ok)
                    .map_err(SubscriptionError::StreamError)
                    .boxed();

            Ok::<
                BoxStream<'static, Result<async_nats::Message, SubscriptionError>>,
                SubscriptionError,
            >(stream)
        }))
        .await
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

        Ok(streams)
    }

    async fn create_relay_stream(
        subject: &str,
    ) -> Result<BoxStream<'static, Result<async_nats::Message, SubscriptionError>>, SubscriptionError>
    {
        let client = Self::relay_nats_client().await?;

        let subscription = client
            .subscribe(subject.to_string())
            .await
            .map_err(|error| SubscriptionError::SubscribeError {
                subject: subject.to_string(),
                error: Box::new(error),
            })?;

        Ok(subscription
            .map(Ok)
            .map_err(SubscriptionError::StreamError)
            .boxed())
    }
}
