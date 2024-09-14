#![recursion_limit = "192"]

pub mod SearchComponent;
pub mod api_doc_page;
pub mod api_doc_request_and_response;
pub mod app;
pub mod transition_blank;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
