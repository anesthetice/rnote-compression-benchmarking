use plotters::prelude::*;

pub const COLOR_WHEEL: [RGBColor; 20] = [
    RGBColor(0x00, 0xff, 0xff),
    RGBColor(0x65, 0x37, 0x00),
    RGBColor(0x13, 0xea, 0xc9),
    RGBColor(0x06, 0x9a, 0xf3),
    RGBColor(0xe6, 0xda, 0xa6),
    RGBColor(0xc1, 0xf8, 0x0a),
    RGBColor(0x7f, 0xff, 0x00),
    RGBColor(0xd2, 0x69, 0x1e),
    RGBColor(0xfc, 0x5a, 0x50),
    RGBColor(0x03, 0x07, 0x64),
    RGBColor(0x00, 0x64, 0x00),
    RGBColor(0xed, 0x0d, 0xd9),
    RGBColor(0xff, 0xd7, 0x00),
    RGBColor(0x4b, 0x00, 0x82),
    RGBColor(0xc7, 0x9f, 0xef),
    RGBColor(0xaa, 0xff, 0x32),
    RGBColor(0xff, 0xa5, 0x00),
    RGBColor(0xf9, 0x73, 0x06),
    RGBColor(0xda, 0x70, 0xd6),
    RGBColor(0xfa, 0x80, 0x72),
];

// taken from Syracuse
pub mod interpolation {
    use itertools::Itertools;

    // plotters.rs has a lineseries options but I dislike it as the width is not consistent depending on the slope
    // this method gives us a more consistent line width
    pub fn linear(points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
        let nb_points = 1000;
        points
            .into_iter()
            .tuple_windows()
            .map(|((x_i, y_i), (x_ip1, y_ip1))| {
                let mut local_points: Vec<(f64, f64)> = Vec::new();

                let a = (y_ip1 - y_i) / (x_ip1 - x_i);
                let b = y_i - a * x_i;
                let f = move |x: f64| a * x + b;

                // lots of "off-by-one" bs but it's not really important
                let step_size = (x_ip1 - x_i) / nb_points as f64;
                let mut x = x_i;
                while x < x_ip1 {
                    local_points.push((x, f(x)));
                    x += step_size;
                }
                local_points
            })
            .fold(Vec::new(), |mut global_points, local_points| {
                global_points.extend(local_points);
                global_points
            })
    }
}
