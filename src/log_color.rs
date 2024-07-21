#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        eprintln!("{}", format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        eprintln!("{}", format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        eprintln!("{}", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! interaction {
    ($($arg:tt)*) => {
        eprintln!("{}", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! bool_state {
    ($t:expr, $f:expr, $status:expr) => {
        match $status {
            true => format!("{}", $t),
            false => format!("{}", $f),
        }
    };
}
