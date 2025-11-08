use once_cell::sync::Lazy;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    sync::Mutex,
};

static LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("some_terminal_game.log")
        .expect("Failed to open log file!");
    Mutex::new(file)
});

//because macros are stupid sometimes
//
#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        name.strip_suffix("::f").unwrap()
    }};
}
#[allow(unused)]
pub(crate) use function_name;

#[track_caller]
pub fn log_internal(func: &str, level: &str, msg: &str) {
    let line1 = format!("[{func}] [{level}] ", func = func, level = level,);

    let mut line2 = line1.clone();
    line2.push_str(msg);
    line2.push('\n');

    let color = match level {
        "INF" => "\x1b[32m",
        "WRN" => "\x1b[33m",
        "ERR" => "\x1b[31m",
        _ => "\x1b[0m",
    };
    println!("{color}{line1}\x1b[0m{msg}");

    if let Ok(mut f) = LOG_FILE.lock() {
        let _ = f.write_all(line2.as_bytes());
    }
}

#[macro_export]
macro_rules! __real_log {
    // INFO ARM: Matches the syntax `log(info, ...)`
    (info, $($arg:tt)*) => {{
        $crate::util::logger::log_internal(function_name!(),"INF", &format!($($arg)*));
    }};

    // WARN ARM: Matches the syntax `log(warn, ...)`
    (warn, $($arg:tt)*) => {{
        $crate::util::logger::log_internal(function_name!(),"WRN", &format!($($arg)*));
    }};

    // ERROR ARM: Matches the syntax `log(error, ...)`
    (error, $($arg:tt)*) => {{
        $crate::util::logger::log_internal(function_name!(),"ERR", &format!($($arg)*));
    }};

    // Optional: A fallback for unrecognized levels
    ($level:ident, $($arg:tt)*) => {{
        $crate::util::logger::log_internal(
            &stringify!($level).to_uppercase(),
            &format!($($arg)*)
        );
    }};
}
pub use __real_log as log;
