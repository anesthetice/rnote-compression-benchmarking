use std::io::Write;

pub fn gzip(level: u32) -> Box<dyn Fn(&[u8]) -> Vec<u8>> {
    Box::new(move |data: &[u8]| {
        let mut encoder =
            flate2::write::GzEncoder::new(Vec::<u8>::new(), flate2::Compression::new(level));
        encoder.write_all(data).unwrap();
        encoder.finish().unwrap()
    })
}

pub fn brotli(level: u32, buffer_size: usize, window_size: u32) -> Box<dyn Fn(&[u8]) -> Vec<u8>> {
    Box::new(move |data: &[u8]| {
        let mut compressed: Vec<u8> = Vec::new();
        let mut encoder =
            brotli::CompressorWriter::new(&mut compressed, buffer_size, level, window_size);
        encoder.write_all(data).unwrap();
        drop(encoder);
        compressed
    })
}

pub fn zstd(level: i32) -> Box<dyn Fn(&[u8]) -> Vec<u8>> {
    Box::new(move |data: &[u8]| {
        let mut encoder = zstd::Encoder::new(Vec::<u8>::new(), level).unwrap();
        encoder.write_all(data).unwrap();
        encoder.finish().unwrap()
    })
}
