mod operations;
mod types;

pub use operations::*;
pub use types::*;

use crate::WorkOs;

pub struct Sso<'a> {
    workos: &'a WorkOs,
}

impl<'a> Sso<'a> {
    pub fn new(workos: &'a WorkOs) -> Self {
        Self { workos }
    }
}
