pub use async_nats::Error as EdgeNatsError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SubscriptionError {
    #[error("Failed to connect to NATS server: {0}")]
    ConnectionError(EdgeNatsError),

    #[error("Failed to subscribe to subject '{subject}': {error}")]
    SubscribeError {
        subject: String,
        error: EdgeNatsError,
    },

    #[error("Error in message stream: {0}")]
    StreamError(EdgeNatsError),

    #[error("Missing required environment variable: {0}")]
    MissingCredentials(String),

    #[error("Connection failed: {msg}")]
    ConnectionFailure {
        msg: String,
        #[source]
        source: Option<EdgeNatsError>,
    },

    #[error("Failed to connect to {connected}/{total} archive NATS servers")]
    PartialConnectionFailure { connected: usize, total: usize },
}

// Implement manual conversions instead of using #[from]
impl From<EdgeNatsError> for SubscriptionError {
    fn from(error: EdgeNatsError) -> Self {
        SubscriptionError::ConnectionError(error)
    }
}

// Helper methods for creating specific error variants
impl SubscriptionError {
    pub fn subscribe_error(subject: &str, error: EdgeNatsError) -> Self {
        SubscriptionError::SubscribeError {
            subject: subject.to_string(),
            error,
        }
    }

    pub fn stream_error(error: EdgeNatsError) -> Self {
        SubscriptionError::StreamError(error)
    }

    pub fn connection_failure(msg: &str, source: Option<EdgeNatsError>) -> Self {
        SubscriptionError::ConnectionFailure {
            msg: msg.to_string(),
            source,
        }
    }

    pub fn partial_connection_failure(connected: usize, total: usize) -> Self {
        SubscriptionError::PartialConnectionFailure { connected, total }
    }
}
