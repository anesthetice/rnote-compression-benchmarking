use crate::{bfunc::Bfunc, graph::COLOR_WHEEL};
use itertools::Itertools;
use plotters::prelude::*;

pub struct Bencher<'input, F1, F2>
where
    F1: Fn(&[u8]) -> Vec<u8>,
    F2: Fn(&[u8]),
{
    functions: Vec<Bfunc<F1, F2>>,
    inputs: Vec<&'input [u8]>,
}

impl<'input, F1, F2> Bencher<'input, F1, F2>
where
    F1: Fn(&[u8]) -> Vec<u8>,
    F2: Fn(&[u8]),
{
    pub fn new(functions: Vec<Bfunc<F1, F2>>, inputs: Vec<&'input [u8]>) -> Self {
        Self { functions, inputs }
    }

    pub fn run(self, num_of_samples: u8) {
        let cpu_name = sysinfo::System::new_with_specifics(
            sysinfo::RefreshKind::new().with_cpu(sysinfo::CpuRefreshKind::new()),
        )
        .cpus()
        .first()
        .unwrap()
        .brand()
        .replace(" ", "_");

        let mut title = self
            .functions
            .iter()
            .fold(cpu_name, |acc, x| acc + "_" + x.title);
        title.push_str(".png");

        let root = BitMapBackend::new(&title, (1200, 2100)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let (top, middle_bottom) = root.split_vertically(600);
        let (middle, bottom) = middle_bottom.split_vertically(600);

        let mut decomp_size_comp_size_chart = ChartBuilder::on(&top)
            .caption("DS-CS", ("sans-serif", 25).into_font())
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0f64..130f64, 0f64..30f64)
            .unwrap();

        decomp_size_comp_size_chart
            .configure_mesh()
            .x_desc("decompressed size [MB]")
            .y_desc("compressed size [MB]")
            .axis_desc_style(("sans-serif", 20).into_font())
            .draw()
            .unwrap();

        let mut decomp_size_comp_time_chart = ChartBuilder::on(&middle)
            .caption("DS-CT", ("sans-serif", 25).into_font())
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0f64..130f64, 0f64..5f64)
            .unwrap();

        decomp_size_comp_time_chart
            .configure_mesh()
            .x_desc("decompressed size [MB]")
            .y_desc("compression time [s]")
            .axis_desc_style(("sans-serif", 20).into_font())
            .draw()
            .unwrap();

        let mut comp_size_decomp_time_chart = ChartBuilder::on(&bottom)
            .caption("CS-DT", ("sans-serif", 25).into_font())
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0f64..30f64, 0f64..1f64)
            .unwrap();

        comp_size_decomp_time_chart
            .configure_mesh()
            .x_desc("compressed size [MB]")
            .y_desc("decompression time [s]")
            .axis_desc_style(("sans-serif", 20).into_font())
            .draw()
            .unwrap();

        for (idx, bfunc) in self.functions.into_iter().enumerate() {
            let color = COLOR_WHEEL.get(idx).unwrap_or_else(|| {
                eprintln!("Not enough colors in COLOR_WHEEL");
                &RED
            });
            let (ds_cs_res, ds_ct_res, cs_dt_res): (
                Vec<(f64, f64)>,
                Vec<(f64, f64)>,
                Vec<(f64, f64)>,
            ) = self
                .inputs
                .iter()
                .map(|input| {
                    let result = bfunc.bench(input, num_of_samples);
                    (
                        result.decomp_size_comp_size,
                        result.decomp_size_comp_time,
                        result.comp_size_decomp_time,
                    )
                })
                .multiunzip();

            decomp_size_comp_size_chart
                .draw_series(
                    crate::graph::interpolation::linear(ds_cs_res)
                        .into_iter()
                        .map(|coord| Circle::new(coord, 1, color.stroke_width(1))),
                )
                .unwrap()
                .label(bfunc.title)
                .legend(move |(x, y)| Circle::new((x + 10, y), 7, color.stroke_width(2)));

            decomp_size_comp_time_chart
                .draw_series(
                    crate::graph::interpolation::linear(ds_ct_res)
                        .into_iter()
                        .map(|coord| Circle::new(coord, 1, color.stroke_width(1))),
                )
                .unwrap();

            comp_size_decomp_time_chart
                .draw_series(
                    crate::graph::interpolation::linear(cs_dt_res.clone())
                        .into_iter()
                        .map(|coord| Circle::new(coord, 1, color.stroke_width(1))),
                )
                .unwrap();
        }
        decomp_size_comp_size_chart
            .configure_series_labels()
            .label_font(("sans-serif", 20).into_font())
            .border_style(BLACK)
            .draw()
            .unwrap();
        root.present().unwrap();
    }
}
