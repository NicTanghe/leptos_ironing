pub mod app;
pub mod comms;
pub mod error_template;
pub mod tasks;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod contacts;
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
