use crate::{Cache, MyCache};
use mdi::resolve;

#[resolve]
pub fn name() -> &'static str {
    "cache"
}

#[resolve]
pub fn cache() -> impl Cache<String> {
    MyCache::default()
}
