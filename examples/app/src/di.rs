use cache::Cache;
use mdi::resolve;

#[resolve]
pub fn cache() -> impl Cache<String> {
    cache::di::cache()
}
