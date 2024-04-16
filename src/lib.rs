pub mod app;
#[cfg(feature = "ssr")]
pub mod config;
#[cfg(feature = "ssr")]
mod database;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod file_serve;
#[cfg(feature = "ssr")]
pub mod ipfs;
#[cfg(feature = "ssr")]
pub mod state;
#[cfg(feature = "ssr")]
mod version;

mod health;
mod server;
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(WebApp);
}
