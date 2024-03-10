use std::{fs::File, io::Write, ops::RangeInclusive};

use crate::utils::{clear_file, AppError};

use super::Animation;

pub struct GameOfLife {
    /// should not be mutated, mutate mut_fil
    initial_num_chars: usize,
    /// file current frame
    mut_file: Vec<String>,
    /// range of lines, line should begin at 0
    wsize: RangeInclusive<usize>,
    /// current raw frame
    animation_lines: Vec<Vec<char>>,
    width: usize,
}

impl Animation for GameOfLife {
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
                let mut anim_line = s.chars().collect::<Vec<_>>();
                anim_line.resize(max_width, ' ');
                anim_line
            })
            .collect::<Vec<_>>();

        Ok(Self {
            initial_num_chars: max_width * animation_lines.len(),
            mut_file: lines,
            animation_lines,
            width: max_width,
            wsize: window_size,
        })
    }

    fn compute_next_frame(&mut self) -> usize {
        let mut changes_count = 0;

        let (w, h) = (self.width, self.animation_lines.len());
        let mut new_frame = vec![vec![' '; w]; h];

        #[allow(clippy::needless_range_loop)]
        for y in 0..h {
            for x in 0..w {
                let (top_raw, bottom_raw) = (
                    if y == 0 { h - 1 } else { y - 1 },
                    if y == h - 1 { 0 } else { y + 1 },
                );
                let (left_col, right_col) = (
                    if x == 0 { w - 1 } else { x - 1 },
                    if x == w - 1 { 0 } else { x + 1 },
                );

                let neighbours = [
                    self.animation_lines[top_raw][left_col],     // top left
                    self.animation_lines[top_raw][x],            // top mid
                    self.animation_lines[top_raw][right_col],    // top right
                    self.animation_lines[y][left_col],           // mid left
                    self.animation_lines[y][right_col],          // mid right
                    self.animation_lines[bottom_raw][left_col],  // bottom left
                    self.animation_lines[bottom_raw][x],         // bottom mid
                    self.animation_lines[bottom_raw][right_col], // bottom right
                ];

                let is_alive = self.animation_lines[y][x] != ' ';
                let alive_cells = neighbours.iter().filter(|c| c != &&' ').collect::<Vec<_>>();
                let alive_cells_count = alive_cells.len();

                new_frame[y][x] = match (is_alive, alive_cells_count) {
                    (true, count) if !(2..=3).contains(&count) => ' ',
                    (false, 3) => *alive_cells[(x + y) % 3],
                    _ => self.animation_lines[y][x],
                };
                if new_frame[y][x] != self.animation_lines[y][x] {
                    changes_count += 1;
                }
            }
        }
        self.animation_lines = new_frame;
        changes_count
    }

    fn render_frame(&mut self, file: &mut File) -> Result<(), AppError> {
        let (offs, offe) = (*self.wsize.start(), *self.wsize.end());
        for alid in 0..(offe - offs + 1) {
            let line = &self.animation_lines[alid];

            let mut anim_line = String::with_capacity(line.len());
            anim_line.extend(line);
            self.mut_file[offs + alid] = anim_line
        }
        let file_frame = self.mut_file.join("\n");
        // apply frame
        clear_file(file)?;
        file.write_all(file_frame.as_bytes())
            .map_err(|_| AppError::FileWriting)
    }

    fn should_it_halt(&self, _: usize) -> bool {
        false
    }

    fn delta_frame() -> usize {
        100
    }
}
