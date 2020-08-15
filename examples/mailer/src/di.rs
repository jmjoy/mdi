use mdi::resolve;

#[resolve]
pub fn name() -> &'static str {
    "mailer"
}
