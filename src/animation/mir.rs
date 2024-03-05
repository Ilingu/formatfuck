use std::{fs, ops::RangeInclusive, path::Path};

use crate::utils::AppError;

use super::Animation;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum XDirection {
    Left = 0,
    Right,
}
pub struct MakeItRain {
    const_file: Vec<String>,
    animation_lines: Vec<Vec<(char, XDirection)>>, // could make a type 'struct Cell(char, XDirection)' for convieniency and clean code, but I fear the perf
    max_width: usize,
}

impl Animation for MakeItRain {
    fn new(filepath: &Path, window_size: RangeInclusive<usize>) -> Result<Self, AppError> {
        let file_content = fs::read_to_string(filepath).map_err(|_| AppError::FileReading)?;
        let lines = file_content
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<_>>();

        let mut animation_lines = lines[window_size.clone()]
            .iter()
            .map(|s| s.chars().map(|c| (c, XDirection::Left)).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let max_width = animation_lines
            .iter()
            .map(|l| l.len())
            .max()
            .ok_or(AppError::UnwrapOption)?;
        animation_lines
            .iter_mut()
            .for_each(|l| l.resize(max_width, (' ', XDirection::Left)));

        Ok(Self {
            const_file: lines,
            animation_lines,
            max_width,
        })
    }

    fn compute_next_frame(&mut self) {
        // from bottom to top so that it can be done in place!
        let lines_height = self.animation_lines.len();
        for y in (0..lines_height).rev() {
            for x in 0..self.max_width {
                let cell = self.animation_lines[y][x];
                // check vertically
                if y != lines_height - 1 && self.animation_lines[y + 1][x].0 == ' ' {
                    self.animation_lines[y + 1][x] = cell;
                    self.animation_lines[y][x] = (' ', XDirection::Left);
                    continue;
                }
                // check horizontally
                match cell.1 {
                    XDirection::Left if x != 0 => {
                        if self.animation_lines[y][x - 1].0 == ' ' {
                            self.animation_lines[y][x - 1] = cell;
                            self.animation_lines[y][x] = (' ', XDirection::Left);
                        } else {
                            self.animation_lines[y][x] = (cell.0, XDirection::Right);
                        }
                    }
                    XDirection::Right if x != self.max_width - 1 => {
                        if self.animation_lines[y][x + 1].0 == ' ' {
                            self.animation_lines[y][x + 1] = cell;
                            self.animation_lines[y][x] = (' ', XDirection::Left);
                        } else {
                            self.animation_lines[y][x] = (cell.0, XDirection::Left);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
