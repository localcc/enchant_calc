pub mod app;
pub mod calculator;
pub mod images;
pub mod step_ext;

pub use app::App;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn main(supports_worker_type: bool) {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async move {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(move |cc| Box::new(App::new(cc, supports_worker_type))),
        )
        .await
        .expect("failed to start eframe");
    });
}
