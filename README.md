## Usage
* clone this repository
* cd into the repo root directory
* download files.7z from the releases
* extract and place the .rnote files into `./files`
* run `cargo run --release`

The program will then run through two sets of benchmarks:
* general benchmarks (gzip, brotli, zstd)
* zstd benchmarks (zstd-3 to zstd-16)

This will take around 25 minutes (sorry)

## Zstd
* zstd seems to be the best option, good speed on compression and decompression, good ratio, multi-threading supported and easy to use
* compression of 9 (out of 21) seems to be ideal
* note that multithreading seems to stop working with very high compression levels

## Brotli
* decent single-threaded speed with a good ratio (better than zstd)
* very poor documentation in general, no documentation on how to use it with multi-threading, esoteric.

## Gzip
* good performance, especially when multi-threaded, not the best compression ratios though
* backwards comp. as rnote already uses gzip
