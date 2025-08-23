pub fn trim_zeros_u8(bytes: &[u8]) -> Vec<u8> {
    bytes.iter().take_while(|&b| *b != 0).copied().collect()
}

pub fn trim_zeros_u16(bytes: &[u16]) -> Vec<u16> {
    bytes.iter().take_while(|&b| *b != 0).copied().collect()
}

pub fn decode_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

pub fn encode_string(string: &str) -> Vec<u8> {
    string.as_bytes().to_vec()
}

pub fn read_to_end<R: binrw::io::Read + binrw::io::Seek>(
    reader: &mut R,
    _: binrw::Endian,
    _: (),
) -> binrw::BinResult<Vec<u8>> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}
