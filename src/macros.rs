#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ($crate::error::Error::Custom(format!("{}", format_args!($($arg)*))));
}
#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
        (return Err($crate::error::Error::Custom(format!("{}", format_args!($($arg)*)))));
    }
}
