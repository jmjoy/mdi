#![warn(rust_2018_idioms)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]

pub mod di;

use cache::Cache;
use mdi::{call, inject};

#[inject]
fn get_cache(cache: impl Cache<String>) {
    let c = cache.get("foo");
}

fn main() {
    call!(get_cache);
}
