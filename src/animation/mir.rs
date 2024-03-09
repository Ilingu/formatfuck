use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    ops::RangeInclusive,
    path::Path,
    thread,
    time::{Duration, SystemTime},
};

use crate::utils::{clear_file, AppError, PathToStr};

use super::Animation;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum XDirection {
    Left = 0,
    Right,
}
pub struct MakeItRain {
    /// should not be mutated, mutate mut_fil
    original_file: String,
    /// file current frame
    mut_file: String,
    filepath: String,
    initial_num_chars: usize,
    /// range of lines, line should begin at 0
    wsize: RangeInclusive<usize>,
    /// current raw frame
    animation_lines: Vec<Vec<(char, usize, XDirection)>>, // could make a type 'struct Cell(char, XDirection)' for convieniency and clean code, but I fear the perf
    /// len of all lines
    width: usize,
}

impl Animation for MakeItRain {
    fn new(filepath: &Path, window_size: RangeInclusive<usize>) -> Result<Self, AppError> {
        let file_content = fs::read_to_string(filepath)
            .map_err(|_| AppError::FileReading(filepath.to_string()))?;

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
                let mut anim_line = s
                    .chars()
                    .enumerate()
                    .map(|(i, c)| (c, i, XDirection::Left))
                    .collect::<Vec<_>>();
                anim_line.resize(max_width, (' ', 0, XDirection::Left));
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
            original_file: file_content,
            initial_num_chars: mut_file.len(),
            mut_file, // it clones once and not everytime a new frame render
            // const_file: lines,
            animation_lines,
            width: max_width,
            wsize: window_size,
            filepath: filepath.to_str().expect("Uuuh?").to_string(),
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
                    self.animation_lines[y][x] = (' ', 0, XDirection::Left);
                    number_of_move += 1;
                    x += 1;
                    continue;
                }
                // check horizontally
                match cell.2 {
                    XDirection::Left => {
                        if x != 0 && self.animation_lines[y][x - 1].0 == ' ' {
                            self.animation_lines[y][x - 1] = cell;
                            self.animation_lines[y][x] = (' ', 0, XDirection::Left);
                            number_of_move += 1;
                        } else {
                            self.animation_lines[y][x] = (cell.0, cell.1, XDirection::Right);
                        }
                    }
                    XDirection::Right => {
                        if x != self.width - 1 && self.animation_lines[y][x + 1].0 == ' ' {
                            self.animation_lines[y][x + 1] = cell;
                            self.animation_lines[y][x] = (' ', 0, XDirection::Left);
                            number_of_move += 1;
                            x += 1; // to prevent from beeing re-moved at the next iteration
                        } else {
                            self.animation_lines[y][x] = (cell.0, cell.1, XDirection::Left);
                        }
                    }
                    _ => {}
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
            let chars = rawline.iter().map(|(c, _, _)| c);
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
            .map_err(|_| AppError::FileWriting(self.filepath.to_owned()))
    }

    fn animation_loop(&mut self) -> Result<(), AppError> {
        const DELTA_FRAME: usize = 50; // in ms -> 20fps

        let mut file = OpenOptions::new()
            .write(true)
            .open(&self.filepath)
            .map_err(|_| AppError::FileReading(self.filepath.to_owned()))?;

        // possible perf improvement: spawn a thread where all the frame are computed in a loop without a sleep
        // and via channels send the computed frame to a 'to render' queue which is where the render then sleep happens
        // but not sure if I will really improve the perf in all kind of scenario because we automatically lose the "in place",
        // and need to clone...
        loop {
            let now = SystemTime::now();

            // compute next frame chars position
            let number_of_move = self.compute_next_frame();
            println!(
                "{}",
                100.0 * (number_of_move as f64 / self.initial_num_chars as f64)
            );
            // check if animation settled
            if 100.0 * (number_of_move as f64 / self.initial_num_chars as f64) <= 0.5 {
                clear_file(&mut file)?;
                file.write_all(self.original_file.as_bytes())
                    .map_err(|_| AppError::FileWriting(self.filepath.to_owned()))?;
                break;
            }
            // update view (file)
            self.render_frame(&mut file)?;

            // if no duration, don't crash the app, it's does not really matter
            if let Ok(dur) = now.elapsed() {
                let elapsed = dur.as_millis() as usize;
                // println!("compute_time = {elapsed}ms");

                if elapsed >= DELTA_FRAME {
                    continue;
                }
                // sleep by the amount of time remaining before next frame
                thread::sleep(Duration::from_millis((DELTA_FRAME - elapsed) as u64));
            }
        }
        Ok(())
    }
}
