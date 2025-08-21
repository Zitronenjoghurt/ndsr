use ndsr_core::codec::rom::NDSRom;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("./roms/hgss.nds");
    let bytes = std::fs::read(path).unwrap();
    let rom = NDSRom::from_bytes(&bytes).unwrap();

    let icon_path = PathBuf::from("./hgss.png");
    let mut buffer = Vec::new();
    rom.icon.render_icon_png_512x(&mut buffer);
    std::fs::write(icon_path, buffer).unwrap();
}
