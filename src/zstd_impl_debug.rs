use std::io::{Read, Write};

/// Decompress bytes with zstd
pub fn decompress_from_zstd(compressed: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    // Optimization for the zstd format
    let mut bytes: Vec<u8> = {
        let frame_header_descriptor = compressed.get(4).ok_or(
            anyhow::anyhow!("Not a valid zstd-compressed file")
                .context("Failed to get the frame header descriptor of the file"),
        )?;

        for (idx, &elem) in compressed[0..15].iter().enumerate() {
            println!("{:0>2} - {:0>8b} | {:0>2x}", idx, elem, elem)
        }
        let frame_content_size_flag = frame_header_descriptor >> 6;
        let single_segment_flag = (frame_header_descriptor >> 5) & 1;
        let dictionary_id_flag = frame_header_descriptor & 11;

        let fcs_sidx = (6 + dictionary_id_flag - single_segment_flag) as usize;

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
    println!("{}", bytes.capacity());
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

pub fn test() {
    let decomp_4 = crate::utils::decompress_default(crate::COMP_4);
    println!(
        "Size = {}, L.E. = {:0>2x?}",
        decomp_4.len(),
        decomp_4.len().to_le_bytes()
    );
    let zstd_comp = compress_to_zstd(&decomp_4).unwrap();
    decompress_from_zstd(&zstd_comp).unwrap();

    println!("\n\n");

    let decomp_10 = crate::utils::decompress_default(crate::COMP_10);
    println!(
        "Size = {}, L.E. = {:0>2x?}",
        decomp_10.len(),
        decomp_10.len().to_le_bytes()
    );
    let zstd_comp = compress_to_zstd(&decomp_10).unwrap();
    decompress_from_zstd(&zstd_comp).unwrap();

    println!("\n\n");

    let decomp_s = vec![0_u8; 233];
    println!(
        "Size = {}, L.E. = {:0>2x?}",
        decomp_s.len(),
        decomp_s.len().to_le_bytes()
    );
    let zstd_comp = compress_to_zstd(&decomp_s).unwrap();
    decompress_from_zstd(&zstd_comp).unwrap();

    println!("\n\n");

    let decomp_m = vec![0_u8; 256];
    println!(
        "Size = {}, L.E. = {:0>2x?}",
        decomp_m.len(),
        decomp_m.len().to_le_bytes()
    );
    let zstd_comp = compress_to_zstd(&decomp_m).unwrap();
    decompress_from_zstd(&zstd_comp).unwrap();
}
