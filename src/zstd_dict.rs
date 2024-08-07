use std::{io::Read, path::Path};

pub fn train(max_size: Option<usize>, path: &Path) -> Vec<u8> {
    // 2 MiB default, medium size dict
    let max_size = max_size.unwrap_or(2097152);

    let (data, sizes) = path
        .read_dir()
        .unwrap()
        .map(|res| {
            let path = res.unwrap().path();
            println!("{}", path.display());
            let mut out: Vec<u8> = Vec::new();
            std::fs::OpenOptions::new()
                .read(true)
                .open(path)
                .unwrap()
                .read_to_end(&mut out)
                .unwrap();
            crate::utils::decompress_default(&out)
        })
        .fold(
            (Vec::<u8>::new(), Vec::<usize>::new()),
            |(mut data, mut sizes), new_data| {
                sizes.push(new_data.len());
                data.extend(new_data);
                (data, sizes)
            },
        );

    zstd::dict::from_continuous(&data, &sizes, max_size).unwrap()
}
