use std::{fs::File, io::Seek, path::Path, process::exit};

use colored::Colorize;

#[macro_export]
macro_rules! errlog {
    ($($arg:tt)*) => {
        eprintln!("{}", format_args!("{}", format!($($arg)*).red()))
    };
}

pub trait PathToStr {
    fn to_string(&self) -> String;
}
impl PathToStr for Path {
    fn to_string(&self) -> String {
        self.to_str().expect("Uuh?").to_string()
    }
}

pub enum AppError {
    FileReading(String),
    FileWriting(String),
    FileRewind,
    InvalidLines,
    UnwrapOption,
}

pub trait ErrorMsg<T> {
    fn unwrap_app(self) -> T;
}
impl<T> ErrorMsg<T> for Result<T, AppError> {
    fn unwrap_app(self) -> T {
        match self {
            Ok(data) => data,
            Err(e) => {
                match e {
                    AppError::FileReading(fpath) => {
                        errlog!("[FATAL]: There was an error while reading '{fpath}'")
                    }
                    AppError::FileWriting(fpath) => {
                        errlog!("[FATAL]: There was an error while writing '{fpath}'")
                    }
                    AppError::InvalidLines => errlog!("[ERROR] Lines range out of bounds"),
                    AppError::UnwrapOption => errlog!("Unwrap on None"),
                    AppError::FileRewind => errlog!("[FATAL] Failed to clear view for new frame"),
                }
                exit(0);
            }
        }
    }
}

pub fn clear_file(file: &mut File) -> Result<(), AppError> {
    file.rewind().map_err(|_| AppError::FileRewind)?;
    file.set_len(0).map_err(|_| AppError::FileRewind)
}
