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
            }) => write!(f, "blocks.{}.{}", producer, block_height),
        }
    }
}
