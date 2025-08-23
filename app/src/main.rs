use crate::app::NDSRApp;
use ndsr_core::codec::rom::NDSRom;
use std::path::PathBuf;

mod app;
mod components;
mod state;
mod systems;
mod views;
mod windows;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_title("NDSR"),
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "NDSR",
        native_options,
        Box::new(|cc| Ok(Box::new(NDSRApp::new(cc)))),
    )
    .expect("Failed to run egui application.");
}

fn load_rom() -> NDSRom {
    let path = PathBuf::from("./roms/hg.nds");
    let bytes = std::fs::read(path).unwrap();
    NDSRom::from_bytes(&bytes).unwrap()
}

fn render_rom_icon(rom: &NDSRom) {
    let icon_path = PathBuf::from("./hg.png");
    let mut buffer = Vec::new();
    rom.icon.render_icon_png_512x(&mut buffer);
    std::fs::write(icon_path, buffer).unwrap();
}
