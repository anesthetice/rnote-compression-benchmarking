use std::io::{Read, Write};

/// Decompress bytes with zstd
pub fn decompress_from_zstd(compressed: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    // Optimization for the zstd format, less pretty than for gzip but this does shave off a bit of time
    // https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#frame_header
    let mut bytes: Vec<u8> = {
        let frame_header_descriptor = compressed.get(4).ok_or(
            anyhow::anyhow!("Not a valid zstd-compressed file")
                .context("Failed to get the frame header descriptor of the file"),
        )?;

        let frame_content_size_flag = frame_header_descriptor >> 6;
        let single_segment_flag = (frame_header_descriptor >> 5) & 1;
        let did_field_size = {
            let dictionary_id_flag = frame_header_descriptor & 11;
            if dictionary_id_flag == 3 {
                4
            } else {
                dictionary_id_flag
            }
        };
        // frame header size start index
        let fcs_sidx = (6 + did_field_size - single_segment_flag) as usize;
        // magic number: 4 bytes + window descriptor: 1 byte if single segment flag is not set + frame header descriptor: 1 byte + dict. field size: 0-4 bytes
        // testing suggests that dicts. don't improve the compression ratio and worsen writing/reading speeds, therefore they won't be used
        // thus this part could be simplified, but wouldn't strictly adhere to zstd standards

        match frame_content_size_flag {
            // not worth it to potentially pre-allocate a maximum of 255 bytes
            0 => Vec::new(),
            1 => {
                let mut decompressed_size: [u8; 2] = [0; 2];
                decompressed_size.copy_from_slice(
                    compressed.get(fcs_sidx..fcs_sidx + 2).ok_or(
                        anyhow::anyhow!("Not a valid zstd-compressed file").context(
                            "Failed to get the uncompressed size of the data from two bytes",
                        ),
                    )?,
                );
                Vec::with_capacity(usize::from(256 + u16::from_le_bytes(decompressed_size)))
            }
            2 => {
                let mut decompressed_size: [u8; 4] = [0; 4];
                decompressed_size.copy_from_slice(
                    compressed.get(fcs_sidx..fcs_sidx + 4).ok_or(
                        anyhow::anyhow!("Not a valid zstd-compressed file").context(
                            "Failed to get the uncompressed size of the data from four bytes",
                        ),
                    )?,
                );
                Vec::with_capacity(
                    u32::from_le_bytes(decompressed_size)
                        .try_into()
                        .unwrap_or(usize::MAX),
                )
            }
            // in practice this should not happen, as a rnote file being larger than 4 GiB is very unlikely
            3 => {
                let mut decompressed_size: [u8; 8] = [0; 8];
                decompressed_size.copy_from_slice(compressed.get(fcs_sidx..fcs_sidx + 8).ok_or(
                    anyhow::anyhow!("Not a valid zstd-compressed file").context(
                        "Failed to get the uncompressed size of the data from eight bytes",
                    ),
                )?);
                Vec::with_capacity(
                    u64::from_le_bytes(decompressed_size)
                        .try_into()
                        .unwrap_or(usize::MAX),
                )
            }
            // unreachable since our u8 is formed by only 2 bits
            4.. => unreachable!(),
        }
    };
    let mut decoder = zstd::Decoder::new(compressed)?;
    decoder.read_to_end(&mut bytes)?;
    Ok(bytes)
}

/// Compress bytes with zstd
pub fn compress_to_zstd(to_compress: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let mut encoder = zstd::Encoder::new(Vec::<u8>::new(), 9)?;
    encoder.set_pledged_src_size(Some(to_compress.len() as u64))?;
    encoder.include_contentsize(true)?;
    if let Ok(num_workers) = std::thread::available_parallelism() {
        encoder.multithread(num_workers.get() as u32)?;
    }
    encoder.write_all(to_compress)?;
    Ok(encoder.finish()?)
}
