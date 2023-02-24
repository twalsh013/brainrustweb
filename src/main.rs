#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use std::future::Future;
use rfd::AsyncFileDialog;
use web_sys::console;
use wasm_bindgen::JsValue;
// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(brainrustweb::TemplateApp::new(cc))),
    )
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    // let task = rfd::AsyncFileDialog::new().pick_file();
    // wasm_bindgen_futures::spawn_local(async {
    //     let file = task.await;

    //     if let Some(file) = file {
    //         // If you are on native platform you can just get the path
    //         #[cfg(not(target_arch = "wasm32"))]
    //         println!("{:?}", file.path());

    //         // If you care about wasm support you just read() the file
    //         let mystring = String::from_utf8(file.read().await).unwrap();
    //         let js: JsValue = mystring.into();
    //         console::log_1(&js);      
    //     }
    // });


    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(brainrustweb::TemplateApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
