use tonic::transport::Endpoint;

use crate::errors::GrpcConnectionError;

pub enum FuelDataStreamNetwork {
    Local,
    Remote,
}

impl TryInto<Endpoint> for FuelDataStreamNetwork {
    type Error = GrpcConnectionError;

    fn try_into(self) -> Result<Endpoint, Self::Error> {
        match self {
            FuelDataStreamNetwork::Local => "http://[::1]:50051"
                .parse()
                .map_err(GrpcConnectionError::from),
            FuelDataStreamNetwork::Remote => "http://fuel-data-grpc-edge.fuel.network:50051"
                .parse()
                .map_err(GrpcConnectionError::from),
        }
    }
}
