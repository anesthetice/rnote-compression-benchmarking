use std::time::{Duration, Instant};

// bench function
pub struct Bfunc<F1, F2>
where
    F1: Fn(&[u8]) -> Vec<u8>,
    F2: Fn(&[u8]),
{
    pub title: &'static str,
    pub compressor: F1,
    pub decompressor: F2,
}

impl<F1, F2> Bfunc<F1, F2>
where
    F1: Fn(&[u8]) -> Vec<u8>,
    F2: Fn(&[u8]),
{
    pub fn new(title: &'static str, compressor: F1, decompressor: F2) -> Self {
        Self {
            title,
            compressor,
            decompressor,
        }
    }
    pub fn bench(&self, uncompressed_input_data: &[u8], num_of_samples: u8) -> BenchResult {
        println!("--  Benchmarking '{}'  --", self.title);

        println!("## warming up...");
        let _start = Instant::now();
        let compressed_input_data = (self.compressor)(uncompressed_input_data);
        while _start.elapsed().as_secs() < 3 {
            (self.compressor)(uncompressed_input_data);
        }

        println!("## compressing...");
        let mut durations: Vec<Duration> = Vec::new();
        for _ in 0..num_of_samples {
            let inst = Instant::now();
            let _ = (self.compressor)(uncompressed_input_data);
            durations.push(inst.elapsed())
        }
        let median_comp_time = Self::median(durations).as_secs_f64();
        println!("median compression time: {:.8}", median_comp_time);

        println!("## decompressing...");
        let mut durations: Vec<Duration> = Vec::new();
        for _ in 0..num_of_samples {
            let inst = Instant::now();
            (self.decompressor)(&compressed_input_data);
            durations.push(inst.elapsed())
        }
        let median_decomp_time = Self::median(durations).as_secs_f64();
        println!("median decompression time: {:.8}", median_decomp_time);

        let decomp_size_mb = uncompressed_input_data.len() as f64 / 1e6;
        let comp_size_mb = compressed_input_data.len() as f64 / 1e6;

        BenchResult {
            decomp_size_comp_size: (decomp_size_mb, comp_size_mb),
            decomp_size_comp_time: (decomp_size_mb, median_comp_time),
            comp_size_decomp_time: (comp_size_mb, median_decomp_time),
        }
    }

    fn median(mut input: Vec<Duration>) -> Duration {
        input.sort();
        let n = input.len();
        // if n is odd
        if input.len() % 2 == 1 {
            input.remove((n + 1) / 2)
        }
        // if n is even
        else {
            (input.remove(n / 2) + input.remove(n / 2)) / 2
        }
    }
}

pub struct BenchResult {
    pub decomp_size_comp_size: (f64, f64),
    pub decomp_size_comp_time: (f64, f64),
    pub comp_size_decomp_time: (f64, f64),
}
