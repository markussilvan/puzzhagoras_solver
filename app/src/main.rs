// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::all, rust_2018_idioms)]

mod gui;
mod piece;
mod puzzle;
mod solver;

use eframe::egui;

// Main for the native application
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(
                    &include_bytes!("../assets/puzzhagoras_icon.png")[..],
                )
                .unwrap(),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Puzzhagoras Solver",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(crate::gui::PuzzhagorasApp::new(cc)))
        }),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(crate::gui::PuzzhagorasApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
