use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    ops::RangeInclusive,
    path::Path,
    thread,
    time::{Duration, SystemTime},
};

use crate::utils::{clear_file, AppError};

pub mod gol;
pub mod mir;

pub trait Animation {
    fn new(file_content: &str, window_size: RangeInclusive<usize>) -> Result<Self, AppError>
    where
        Self: std::marker::Sized;

    fn create_and_launch(
        filepath: &Path,
        window_size: RangeInclusive<usize>,
    ) -> Result<(), AppError>
    where
        Self: std::marker::Sized,
    {
        let file_content = fs::read_to_string(filepath).map_err(|_| AppError::FileReading)?;

        let mut animation = Self::new(&file_content, window_size)?;
        animation.animate(filepath, &file_content)
    }

    fn should_it_halt(&self, number_of_move: usize) -> bool;

    fn delta_frame() -> usize {
        75
    }

    fn animate(&mut self, filepath: &Path, original_fcontent: &str) -> Result<(), AppError> {
        let mut file = OpenOptions::new()
            .write(true)
            .open(filepath)
            .map_err(|_| AppError::FileReading)?;

        loop {
            let now = SystemTime::now();

            // compute next frame chars position
            let number_of_move = self.compute_next_frame();
            // check if animation settled
            if self.should_it_halt(number_of_move) {
                clear_file(&mut file)?;
                file.write_all(original_fcontent.as_bytes())
                    .map_err(|_| AppError::FileWriting)?;
                break;
            }
            // update view (file)
            self.render_frame(&mut file)?;

            // if no duration, don't crash the app, it's does not really matter
            if let Ok(dur) = now.elapsed() {
                let elapsed = dur.as_millis() as usize;
                // println!("compute_time = {elapsed}ms");

                if elapsed >= Self::delta_frame() {
                    continue;
                }
                // sleep by the amount of time remaining before next frame
                thread::sleep(Duration::from_millis(
                    (Self::delta_frame() - elapsed) as u64,
                ));
            }
        }
        Ok(())
    }

    fn compute_next_frame(&mut self) -> usize;
    /// save the frame in the file
    fn render_frame(&mut self, file: &mut File) -> Result<(), AppError>;
}
