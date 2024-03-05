#[macro_export]
macro_rules! errorlog {
    ($msg:expr) => {
        eprintln!("{}", $msg.red())
    };
}
