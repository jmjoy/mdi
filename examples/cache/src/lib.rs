#![warn(rust_2018_idioms)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]

pub mod di;

use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

pub trait Serialize {}

impl Serialize for String {}

pub trait Cache<S: Serialize> {
    fn get(&self, key: &str) -> Option<&S>;
    fn set(&mut self, key: &str, value: S, time: Duration);
}

#[derive(Default)]
pub struct MyCache {
    inner: HashMap<String, (String, SystemTime)>,
}

impl Cache<String> for MyCache {
    fn get(&self, key: &str) -> Option<&String> {
        self.inner.get(key).and_then(|(value, time)| {
            if &SystemTime::now() < time {
                Some(value)
            } else {
                None
            }
        })
    }

    fn set(&mut self, key: &str, value: String, ttl: Duration) {
        self.inner
            .insert(key.to_owned(), (value, SystemTime::now() + ttl));
    }
}
