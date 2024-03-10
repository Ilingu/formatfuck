use std::{fs::File, io::Write, ops::RangeInclusive};

use crate::utils::{clear_file, AppError};

use super::Animation;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum XDirection {
    Left = 0,
    Right,
}
pub struct MakeItRain {
    /// file current frame
    mut_file: String,
    initial_num_chars: usize,
    /// range of lines, line should begin at 0
    wsize: RangeInclusive<usize>,
    /// current raw frame
    animation_lines: Vec<Vec<(char, XDirection)>>, // could make a type 'struct Cell(char, XDirection)' for convieniency and clean code, but I fear the perf
    /// len of all lines
    width: usize,
}

impl Animation for MakeItRain {
    fn new(file_content: &str, window_size: RangeInclusive<usize>) -> Result<Self, AppError> {
        let lines = file_content
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<_>>();
        if lines.len() <= *window_size.end() {
            return Err(AppError::InvalidLines);
        }

        let max_width = lines
            .iter()
            .map(|l| l.len())
            .max()
            .ok_or(AppError::UnwrapOption)?;

        let animation_lines = lines[window_size.clone()]
            .iter()
            .map(|s| {
                let mut anim_line = s.chars().map(|c| (c, XDirection::Left)).collect::<Vec<_>>();
                anim_line.resize(max_width, (' ', XDirection::Left));
                anim_line
            })
            .collect::<Vec<_>>();

        let mut_file = lines
            .into_iter()
            .map(|l| {
                let mut new_line = l.chars().collect::<Vec<_>>();
                new_line.resize(max_width, ' ');
                new_line.push('\n');

                String::from_iter(new_line)
            })
            .collect::<String>();

        Ok(Self {
            initial_num_chars: mut_file.len(),
            mut_file, // it clones once and not everytime a new frame render
            // const_file: lines,
            animation_lines,
            width: max_width,
            wsize: window_size,
        })
    }

    fn compute_next_frame(&mut self) -> usize {
        let mut number_of_move: usize = 0;

        // from bottom to top so that it can be done in place!
        let lines_height = self.animation_lines.len();
        let (mut y, mut x) = (lines_height - 1, 0);
        loop {
            while x < self.width {
                // logic buisness
                let cell = self.animation_lines[y][x];
                if cell.0 == ' ' {
                    x += 1;
                    continue;
                }
                // check vertically
                if y != lines_height - 1 && self.animation_lines[y + 1][x].0 == ' ' {
                    self.animation_lines[y + 1][x] = cell;
                    self.animation_lines[y][x] = (' ', XDirection::Left);
                    number_of_move += 1;
                    x += 1;
                    continue;
                }
                // check horizontally
                match cell.1 {
                    XDirection::Left => {
                        if x != 0 && self.animation_lines[y][x - 1].0 == ' ' {
                            self.animation_lines[y][x - 1] = cell;
                            self.animation_lines[y][x] = (' ', XDirection::Left);
                            number_of_move += 1;
                        } else {
                            self.animation_lines[y][x] = (cell.0, XDirection::Right);
                        }
                    }
                    XDirection::Right => {
                        if x != self.width - 1 && self.animation_lines[y][x + 1].0 == ' ' {
                            self.animation_lines[y][x + 1] = cell;
                            self.animation_lines[y][x] = (' ', XDirection::Left);
                            number_of_move += 1;
                            x += 1; // to prevent from being re-moved at the next iteration
                        } else {
                            self.animation_lines[y][x] = (cell.0, XDirection::Left);
                        }
                    }
                }

                // index managment
                x += 1;
            }

            // index managment
            x = 0;
            if y == 0 {
                break;
            }
            y -= 1;
        }
        number_of_move
    }

    fn render_frame(&mut self, file: &mut File) -> Result<(), AppError> {
        // compute entire file frame
        let lstart = *self.wsize.start();
        for (i, rawline) in self.animation_lines.iter().enumerate() {
            let chars = rawline.iter().map(|(c, _)| c);
            // more optimal 'Vec<char> to string' than .collect
            let mut anim_line = String::with_capacity(chars.len());
            anim_line.extend(chars);
            anim_line.push('\n');

            let lid = lstart + i;
            let offs = lid * (self.width + 1);
            let offe = offs + (self.width + 1);

            self.mut_file.replace_range(offs..offe, &anim_line);
        }
        // assert_eq!(self.mut_file.len(), self.initial_num_chars);

        // apply frame
        clear_file(file)?;
        file.write_all(self.mut_file.as_bytes())
            .map_err(|_| AppError::FileWriting)
    }

    fn should_it_halt(&self, number_of_move: usize) -> bool {
        100.0 * (number_of_move as f64 / self.initial_num_chars as f64) <= 0.5
    }
}
