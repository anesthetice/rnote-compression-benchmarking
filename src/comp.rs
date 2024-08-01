use std::io::Write;

pub fn gzip(level: u32) -> impl Fn(&[u8]) -> Vec<u8> {
    move |data: &[u8]| {
        let mut encoder =
            flate2::write::GzEncoder::new(Vec::<u8>::new(), flate2::Compression::new(level));
        encoder.write_all(data).unwrap();
        encoder.finish().unwrap()
    }
}
