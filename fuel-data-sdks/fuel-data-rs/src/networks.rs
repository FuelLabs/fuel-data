use tonic::transport::Endpoint;

use crate::errors::GrpcConnectionError;

pub enum FuelDataGrpcEdgeNetwork {
    Local,
    Remote,
}

impl TryInto<Endpoint> for FuelDataGrpcEdgeNetwork {
    type Error = GrpcConnectionError;

    fn try_into(self) -> Result<Endpoint, Self::Error> {
        match self {
            FuelDataGrpcEdgeNetwork::Local => "http://[::1]:50051"
                .parse()
                .map_err(GrpcConnectionError::from),
            FuelDataGrpcEdgeNetwork::Remote => "http://fuel-data-grpc-edge.fuel.network:50051"
                .parse()
                .map_err(GrpcConnectionError::from),
        }
    }
}
