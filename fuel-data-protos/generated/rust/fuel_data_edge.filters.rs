// This file is @generated by prost-build.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlocksFilterProto {
    #[prost(uint32, optional, tag = "1")]
    pub from: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub to: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub producer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "4")]
    pub take: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "5")]
    pub chunk_size: ::core::option::Option<u32>,
}
