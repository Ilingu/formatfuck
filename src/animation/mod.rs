use std::{fs::File, ops::RangeInclusive, path::Path};

use crate::utils::AppError;

pub mod gof;
pub mod mir;

pub trait Animation {
    fn new(filepath: &Path, window_size: RangeInclusive<usize>) -> Result<Self, AppError>
    where
        Self: std::marker::Sized;

    // for each frame count the number of characters that moved
    // by divising by the total number of characters we have the percentage of characters that moved
    // if this percentage is inferior to a certain threshold, automatically stop the simulation
    fn animation_loop(&mut self) -> Result<(), AppError>;

    fn compute_next_frame(&mut self) -> usize;
    /// save the frame in the file
    fn render_frame(&mut self, file: &mut File) -> Result<(), AppError>;
}
