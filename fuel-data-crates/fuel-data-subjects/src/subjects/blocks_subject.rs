use fuel_data_types::*;

use crate::{Filter, Subject, SubjectFilter};
use fuel_data_subjects_macros::*;

#[derive(Subject)]
pub struct BlocksSubject {
    pub producer: Address,
    pub block_height: u32,
}

#[derive(SubjectFilter)]
pub struct BlocksSubjectFilter {
    pub producer: Filter<Address>,
    pub block_height: Filter<u32>,
}
