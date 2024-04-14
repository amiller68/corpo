pub mod app;
#[cfg(feature = "ssr")]
mod config;
#[cfg(feature = "ssr")]
mod database;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
#[cfg(feature = "ssr")]
mod ipfs_gateway;
#[cfg(feature = "ssr")]
mod state;
#[cfg(feature = "ssr")]
mod version;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
