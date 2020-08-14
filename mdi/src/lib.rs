#![warn(rust_2018_idioms)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]

pub use mdi_macros::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    trait Serialize {}

    impl Serialize for String {}

    trait Cache<S: Serialize> {
        fn get(key: &str) -> &S;
        fn set(key: &str, value: S, time: Duration);
    }

    struct MyCache;

    impl<S: Serialize> Cache<S> for MyCache {
        fn get(key: &str) -> &S {
            todo!()
        }
        fn set(key: &str, value: S, time: Duration) {
            todo!()
        }
    }

    #[resolve]
    fn cache() -> impl Cache<String> {
        MyCache
    }

    #[inject]
    fn test_mdi(cache: impl Cache<String>) {}
}
