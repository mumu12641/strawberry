#[macro_export]
macro_rules! c_str {
    ($s:expr) => {
        // concat!($s, "\0").as_ptr() as *const i8
        format!("{}{}", $s, "\0").as_ptr() as *const i8
    };
}

