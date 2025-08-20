use ndsr_core::codec::rom::NDSRom;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("./../roms/malu_bis.nds");
    let bytes = std::fs::read(path).unwrap();
    let rom = NDSRom::from_bytes(&bytes).unwrap();
    println!("{}", serde_json::to_string_pretty(&rom).unwrap());
}
