use std::io::Read;

pub fn decompress_default(input: &[u8]) -> Vec<u8> {
    let mut bytes: Vec<u8> = {
        let mut decompressed_size: [u8; 4] = [0; 4];
        decompressed_size.copy_from_slice(&input[input.len() - 4..]);
        Vec::with_capacity(u32::from_le_bytes(decompressed_size) as usize)
    };
    let mut decoder = flate2::read::MultiGzDecoder::new(input);
    decoder.read_to_end(&mut bytes).unwrap();
    bytes
}
