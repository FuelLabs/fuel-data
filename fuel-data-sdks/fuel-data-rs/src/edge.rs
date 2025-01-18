use crate::errors::{FuelDataError, GrpcConnectionError};

use tonic::transport::Endpoint;

#[derive(Debug, Clone)]
pub struct FuelDataEdge {
    pub(crate) grpc_endpoint: Endpoint,
}

impl FuelDataEdge {
    pub async fn connect<E>(grpc_endpoint: E) -> Result<Self, FuelDataError>
    where
        E: TryInto<Endpoint> + Send,
        <E as TryInto<Endpoint>>::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        let grpc_endpoint = grpc_endpoint
            .try_into()
            .map_err(|e| GrpcConnectionError::InvalidGrpcUri(e.into()))?;
        Ok(Self { grpc_endpoint })
    }
}
