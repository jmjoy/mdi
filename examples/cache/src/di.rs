use crate::{Cache, MyCache};
use mdi::resolve;

#[resolve]
pub fn cache() -> impl Cache<String> {
    MyCache::default()
}
