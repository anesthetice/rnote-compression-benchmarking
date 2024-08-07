use std::io::{Read, Write};

use bencher::Bencher;
use bfunc::Bfunc;
use utils::decompress_default;

mod bencher;
mod bfunc;
mod comp;
mod decomp;
mod graph;
mod utils;
mod zstd_dict;
mod zstd_impl;

const COMP_1: &[u8] = include_bytes!("../files/1.rnote");
const COMP_2: &[u8] = include_bytes!("../files/2.rnote");
const COMP_3: &[u8] = include_bytes!("../files/3.rnote");
const COMP_4: &[u8] = include_bytes!("../files/4.rnote");
const COMP_5: &[u8] = include_bytes!("../files/5.rnote");
const COMP_6: &[u8] = include_bytes!("../files/6.rnote");
const COMP_7: &[u8] = include_bytes!("../files/7.rnote");
const COMP_8: &[u8] = include_bytes!("../files/8.rnote");
const COMP_9: &[u8] = include_bytes!("../files/9.rnote");
const COMP_10: &[u8] = include_bytes!("../files/10.rnote");
const COMP_11: &[u8] = include_bytes!("../files/11.rnote");

fn main() {
    bench_stuff();
}

fn bench_stuff() {
    #[cfg(debug_assertions)]
    panic!("for more accurate results, please run this in release mode.");

    let decomp_1 = decompress_default(COMP_1);
    let decomp_2 = decompress_default(COMP_2);
    let decomp_3 = decompress_default(COMP_3);
    let decomp_4 = decompress_default(COMP_4);
    let decomp_5 = decompress_default(COMP_5);
    let decomp_6 = decompress_default(COMP_6);
    let decomp_7 = decompress_default(COMP_7);
    let decomp_8 = decompress_default(COMP_8);
    let decomp_9 = decompress_default(COMP_9);
    let decomp_10 = decompress_default(COMP_10);
    let decomp_11 = decompress_default(COMP_11);
    println!("{:?}", decomp_11.len() as f64 / 1e6);

    let bencher = Bencher::new(
        vec![
            Bfunc::new("gzip-5", comp::gzip(5), decomp::gzip()),
            Bfunc::new(
                "brotli-2-4096-24",
                comp::brotli(2, 4096, 24),
                decomp::brotli(),
            ),
            Bfunc::new(
                "brotli-4-4096-24",
                comp::brotli(4, 4096, 24),
                decomp::brotli(),
            ),
            Bfunc::new("par-gzip-9", comp::par_gzip(9), decomp::gzip()),
            Bfunc::new("par-zstd-9", comp::par_zstd(9, 12), decomp::zstd()),
            Bfunc::new("par-zstd-9-opt", comp::par_zstd_opt(), decomp::zstd_opt()),
        ],
        vec![
            &decomp_1, &decomp_2, &decomp_3, &decomp_4, &decomp_5, &decomp_6, &decomp_7, &decomp_8,
            &decomp_9, &decomp_10, &decomp_11,
        ],
    );
    bencher.run(16);
}

/*
Bfunc::new("gzip-5", comp::gzip(5), decomp::gzip()),
Bfunc::new(
    "brotli-4-4096-24",
    comp::brotli(4, 4096, 24),
    decomp::brotli(),
),
Bfunc::new("zstd-6", comp::zstd(6), decomp::zstd()),
Bfunc::new("par-gzip-9", comp::par_gzip(9), decomp::gzip()),
Bfunc::new("par-zstd-9", comp::par_zstd(9, num_workers), decomp::zstd()),
*/

fn create_dict() {
    let dict = crate::zstd_dict::train(None, "./files/dict/".as_ref());
    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("rnote.dict")
        .unwrap()
        .write_all(&dict)
        .unwrap();
}
