use crate::errors::StreamFilterError;

pub trait StreamFilter {
    fn take(&mut self, count: u16) -> Self;
    fn chunk(&mut self, count: u16) -> Self;
    fn build(&self) -> Result<Self, StreamFilterError>
    where
        Self: Sized;
}

pub struct BlocksStream {}

impl BlocksStream {
    pub fn new() -> Self {
        Self {}
    }
    pub fn from() -> Self {
        Self {}
    }
    pub fn to() -> Self {
        Self {}
    }
    pub fn producer() -> Self {
        Self {}
    }
}

impl StreamFilter for BlocksStream {
    fn take(&mut self, count: u16) -> Self {
        Self {}
    }

    fn chunk(&mut self, count: u16) -> Self {
        Self {}
    }

    fn build(&self) -> Result<Self, StreamFilterError> {
        Ok(Self {})
    }
}
