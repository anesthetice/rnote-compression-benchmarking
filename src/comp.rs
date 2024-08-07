use gzp::{
    deflate::Gzip,
    par::compress::{ParCompress, ParCompressBuilder},
    Compression, ZWriter,
};
use std::io::Write;

pub type CompFunc = Box<dyn Fn(&[u8]) -> Vec<u8>>;

pub fn gzip(level: u32) -> CompFunc {
    Box::new(move |data: &[u8]| {
        let mut encoder =
            flate2::write::GzEncoder::new(Vec::<u8>::new(), flate2::Compression::new(level));
        encoder.write_all(data).unwrap();
        encoder.finish().unwrap()
    })
}

pub fn brotli(level: u32, buffer_size: usize, window_size: u32) -> CompFunc {
    Box::new(move |data: &[u8]| {
        let mut compressed: Vec<u8> = Vec::new();
        let mut encoder =
            brotli::CompressorWriter::new(&mut compressed, buffer_size, level, window_size);
        encoder.write_all(data).unwrap();
        drop(encoder);
        compressed
    })
}

pub fn zstd(level: i32) -> CompFunc {
    Box::new(move |data: &[u8]| {
        let mut encoder = zstd::Encoder::new(Vec::<u8>::new(), level).unwrap();
        encoder.write_all(data).unwrap();
        encoder.finish().unwrap()
    })
}

pub fn par_zstd(level: i32, num_workers: u32) -> CompFunc {
    Box::new(move |data: &[u8]| {
        let mut encoder = zstd::Encoder::new(Vec::<u8>::new(), level).unwrap();
        encoder.multithread(num_workers).unwrap();
        encoder.write_all(data).unwrap();
        encoder.finish().unwrap()
    })
}

pub fn par_gzip(level: u32) -> CompFunc {
    Box::new(move |data: &[u8]| {
        let compressed: Goofy = Goofy::new();
        let mut encoder: ParCompress<Gzip> = ParCompressBuilder::new()
            .compression_level(Compression::new(level))
            .from_writer(compressed.clone());
        encoder.write_all(data).unwrap();
        encoder.finish().unwrap();
        compressed.extract_clone()
    })
}

pub fn par_zstd_opt() -> CompFunc {
    Box::new(move |data: &[u8]| crate::zstd_impl::compress_to_zstd(data).unwrap())
}

use std::sync::{Arc, RwLock};

struct Goofy {
    inner: Arc<RwLock<Vec<u8>>>,
}

impl core::clone::Clone for Goofy {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl Goofy {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(Vec::new())),
        }
    }
    pub fn extract_clone(&self) -> Vec<u8> {
        self.inner.read().unwrap().clone()
    }
}

impl std::io::Write for Goofy {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write().unwrap().write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.write().unwrap().flush()
    }
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.inner.write().unwrap().write_all(buf)
    }
}
