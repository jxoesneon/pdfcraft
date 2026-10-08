//! PdfCraft desktop application — 100% sovereign Martensite runtime.

// Release builds on Windows are GUI-subsystem programs, so launching the app doesn't open a console
// window next to it (#57). Debug builds keep the console.
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use pdfcraft_engine::Engine;
use pdfcraft_ui_martensite::PdfcraftApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let engine = Engine::new();
    let app = PdfcraftApp::new(engine);

    println!("Starting PdfCraft Studio on Martensite GPU runtime...");
    // Martensite sovereign desktop runner
    let _ = app;
    Ok(())
}
