#![deny(warnings)]
#![deny(unused_extern_crates)]
pub use app::{App, Msg};
use sauron::prelude::*;
pub use sauron;

mod app;
pub mod util;

/// The serialized_state is supplied by the generated page from the webserver.
/// The generated page in index function has a main function which is supplied by a json text
/// serialized state. This json text is deserialized and used here as our `App` value which
/// will then be injected into the view
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub async fn main(serialized_state: String) {
    #[cfg(feature = "wasm-bindgen")]
    {
        console_log::init_with_level(log::Level::Trace).ok();
        console_error_panic_hook::set_once();
    }

    let app = match serde_json::from_str::<App>(&serialized_state) {
        Ok(app_state) => app_state,
        Err(e) => {
            log::warn!("error: {}", e);
            App::default()
        }
    };
    Program::replace_body(app);
}
