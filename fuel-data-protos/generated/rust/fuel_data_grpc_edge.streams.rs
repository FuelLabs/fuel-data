// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlocksStreamRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub ip_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub filter: ::core::option::Option<
        crate::fuel_data_edge::filters::BlocksFilter,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlocksStreamResponse {
    #[prost(message, repeated, tag = "1")]
    pub blocks: ::prost::alloc::vec::Vec<
        crate::fuel_data_types::block::BlockProto,
    >,
}
