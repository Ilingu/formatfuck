#[macro_export]
macro_rules! errorlog {
    ($msg:expr) => {
        eprintln!("{}", $msg.red())
    };
}

pub enum AppError {
    FileReading,
    UnwrapOption,
}
