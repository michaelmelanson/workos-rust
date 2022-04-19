mod operations;
mod types;

pub use operations::*;
pub use types::*;

use crate::WorkOs;

pub struct Organizations<'a> {
    workos: &'a WorkOs,
}

impl<'a> Organizations<'a> {
    pub fn new(workos: &'a WorkOs) -> Self {
        Self { workos }
    }
}
