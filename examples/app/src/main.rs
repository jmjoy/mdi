#![warn(rust_2018_idioms)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]

pub mod di;

use cache::Cache;
use mdi::{call, inject};
use std::time::Duration;

#[inject]
fn get_cache(mut cache: impl Cache<String>) -> Option<String> {
    cache.set("foo", "bar".to_owned(), Duration::from_secs(5));
    cache.get("foo").map(|s| s.clone())
}

fn main() {
    dbg!(call!(get_cache));
}
