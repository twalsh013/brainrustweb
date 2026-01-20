#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Ok(Box::new(brainrustweb::TemplateApp::new(cc)))),
    )
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let window = web_sys::window().expect("No window");
        let document = window.document().expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        // Resize canvas to fill the browser viewport
        fn resize_canvas(window: &web_sys::Window, canvas: &web_sys::HtmlCanvasElement) {
            let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
            let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
            canvas.set_width(width);
            canvas.set_height(height);
        }

        resize_canvas(&window, &canvas);

        // Set up resize event listener
        let canvas_clone = canvas.clone();
        let window_clone = window.clone();
        let resize_closure = Closure::<dyn Fn()>::new(move || {
            resize_canvas(&window_clone, &canvas_clone);
        });
        window
            .add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref())
            .expect("Failed to add resize listener");
        resize_closure.forget(); // Keep the closure alive

        let runner = eframe::WebRunner::new();
        runner
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(brainrustweb::TemplateApp::new(cc)))),
            )
            .await
            .expect("failed to start eframe");
    });
}
