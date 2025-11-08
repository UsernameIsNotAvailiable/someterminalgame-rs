extern crate rustc_version_runtime;
extern crate core;

mod shared;
mod util;

use core::panic::PanicInfo;
use chrono::Local;
use shared::constants::versioning;
use util::logger::log;
use std::{panic, thread};
use std::fmt::format;
use std::time::SystemTime;

fn init_panic_hook() {
    log!(info, "setup panic hook...");

    panic::set_hook(Box::new(|panic_info| {
        let timestamp = SystemTime::now();
        let time = chrono::Local::now();

        let thread = thread::current();
        let thread_name = thread.name().unwrap_or("UNNAMED");

        let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            *s
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.as_str()
        } else {
            "Unknown panic payload"
        };

        let location = panic_info.location()
            .map(|loc| format!("{}:{}:{}",loc.file(),loc.line(),loc.column()))
            .unwrap_or_else(|| "unknown location".into());

        let backtrace = std::backtrace::Backtrace::force_capture();

        eprintln!(
            "\n\x1b[1;31m========== RUST PANIC ==========\x1b[0m
            Time:       {time}
            Thread:     {thread_name}
            Message:    \x1b[1;33m{msg}\x1b[0m
            Location:   {location}
            ---------------------------------
            Backtrace:
            {backtrace}
            \x1b[1;31m===============================\x1b[0m\n"
        );

    }));
    log!(info, "done");
}

fn main() {
    init_panic_hook();
    log!(info, "----- LOG START : {:?} -----", Local::now());
    log!(
        info,
        "Starting {} {}",
        versioning::PKG_NAME,
        versioning::PKG_VERSION
    );
    log!(
        info,
        "rustc version is {}",
        rustc_version_runtime::version()
    );
    
    panic!("oh no!");
    println!("Hello, world!");
}
