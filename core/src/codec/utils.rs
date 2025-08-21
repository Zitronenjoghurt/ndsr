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
