#![warn(rust_2018_idioms)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]

pub mod di;

use std::{
    ffi::c_void,
    fmt::{Debug, Display},
    time::Duration,
};
use mdi::inject;

pub trait Serialize {}

impl Serialize for String {}

pub trait Cache<S: Serialize> {
    fn get(&self, key: &str) -> &S;
    fn set(&mut self, key: &str, value: S, time: Duration);
}

pub struct MyCache;

impl<S: Serialize> Cache<S> for MyCache {
    fn get(&self, key: &str) -> &S {
        todo!()
    }
    fn set(&mut self, key: &str, value: S, time: Duration) {
        todo!()
    }
}

#[inject]
fn test_mdi(cache: impl Cache<String>) {
    cache.get("some_key");
}

fn test_debug() -> Box<dyn Debug> {
    Box::new("hello")
}
