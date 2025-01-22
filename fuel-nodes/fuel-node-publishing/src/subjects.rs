use fuel_data_types::*;

pub struct BlocksSubject {
    pub producer: Address,
    pub block_height: u32,
}

pub enum Subject {
    Blocks(BlocksSubject),
}

impl std::fmt::Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Subject::Blocks(BlocksSubject {
                producer,
                block_height,
                ..
            }) => write!(f, "blocks.{}.{}", producer, block_height),
        }
    }
}

pub enum Query<T> {
    All,
    Only(T),
}

impl<T> Query<T> {
    pub fn as_ref(&self) -> Option<&T> {
        match self {
            Query::All => None,
            Query::Only(value) => Some(value),
        }
    }
}

pub struct BlocksSubjectQuery {
    pub producer: Query<Address>,
    pub block_height: Query<u32>,
}

pub trait SubjectQuery {
    type DataTypeProto: prost::Message + Default;
    type DataType: From<Self::DataTypeProto>;

    fn to_nats_subject(&self) -> String;
}

macro_rules! query_field_to_string {
    ($self:ident, $field:ident) => {
        $self
            .$field
            .as_ref()
            .map(|v| v.to_string())
            .unwrap_or_else(|| "*".to_string())
    };
}

impl SubjectQuery for BlocksSubjectQuery {
    type DataType = Block;
    type DataTypeProto = BlockProto;

    fn to_nats_subject(&self) -> String {
        let producer = query_field_to_string!(self, producer);
        let block_height = query_field_to_string!(self, block_height);

        format!("blocks.{producer}.{block_height}")
    }
}
