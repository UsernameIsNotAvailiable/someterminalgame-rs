extern crate rustc_version_runtime;
extern crate core;

mod shared;
mod util;

use chrono::Local;
use shared::constants::versioning;
use util::logger::log;

fn main() {
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
}
