use std::io::Read;

pub fn gzip(compressed: &[u8]) {
    let mut bytes: Vec<u8> = {
        let mut decompressed_size: [u8; 4] = [0; 4];
        decompressed_size.copy_from_slice(&compressed[compressed.len() - 4..]);
        Vec::with_capacity(u32::from_le_bytes(decompressed_size) as usize)
    };
    let mut decoder = flate2::read::MultiGzDecoder::new(compressed);
    decoder.read_to_end(&mut bytes).unwrap();
}
