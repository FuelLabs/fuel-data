mod blocks_subject;

pub use blocks_subject::*;

pub trait Subject {
    type DataTypeProto: prost::Message;

    fn to_nats_subject(&self) -> String;
}

pub enum Filter<T> {
    All,
    Only(T),
}

impl<T> From<Option<T>> for Filter<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Filter::Only(value),
            None => Filter::All,
        }
    }
}

impl<T> Filter<T> {
    pub fn as_ref(&self) -> Option<&T> {
        match self {
            Filter::All => None,
            Filter::Only(value) => Some(value),
        }
    }
}

pub trait SubjectFilter {
    type Subject: Subject;
    type DataTypeProto: prost::Message + Default;
    type DataType: From<Self::DataTypeProto>;

    fn to_nats_subject_filter(&self) -> String;
}
