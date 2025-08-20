use ndsr_core::codec::rom::NDSRom;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("./roms/hgss.nds");
    let bytes = std::fs::read(path).unwrap();
    //let raw = RawNDSRom::from_bytes(&bytes).unwrap();
    let rom = NDSRom::from_bytes(&bytes).unwrap();
    println!("{:#?}", rom);
}
