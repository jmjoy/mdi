use mdi::{extern_di, resolve};

extern_di!(cache);
extern_di!(mailer);

#[resolve]
pub fn name() -> &'static str {
    "app"
}
