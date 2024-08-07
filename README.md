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
