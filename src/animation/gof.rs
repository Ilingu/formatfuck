use std::{ops::RangeInclusive, path::Path};

use crate::utils::AppError;

use super::Animation;

pub struct GameOfLife {}

impl Animation for GameOfLife {
    fn new(filepath: &Path, window_size: RangeInclusive<usize>) -> Result<Self, AppError> {
        todo!()
    }

    fn compute_next_frame(&mut self) {
        todo!()
    }
}
