use mdi::resolve;
use crate::{MyCache, Cache};

#[resolve]
pub fn cache() -> impl Cache<String> {
    MyCache
}
