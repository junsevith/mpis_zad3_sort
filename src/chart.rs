use std::iter;
use std::ops::Range;
use plotters::prelude::*;

pub fn draw_chart2<T>(x_range: Range<f64>, scalar: f64, values: &Vec<T>, extractor: fn(&T) -> (f64, f64, Vec<f64>), transformator: fn(f64, f64) -> f64, filename: &str, all_vals: bool) {
    let file = format!("charts/{}.png", filename);

    let min = 0f64;
    let firstval = extractor(values.first().unwrap());
    let first = transformator(firstval.0, firstval.1);
    let lastval = extractor(values.last().unwrap());
    let last = transformator(lastval.0, lastval.1);
    let used;
    if first > last {
        used = firstval;
    } else {
        used = lastval;
    }
    let max = transformator(used.0, used.1) * scalar;
    let y_range = min..max;

    let drawing_area = BitMapBackend::new(&file, (1024, 768))
        .into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&drawing_area)
        .caption(filename, ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(x_range.clone(), y_range.clone())
        .unwrap();
    ctx.configure_mesh().draw().unwrap();

    for value in values {
        let (x, avg, vals) = extractor(value);
        if all_vals {
            ctx.draw_series(vals.iter().map(|y| Circle::new((x, transformator(x, *y)), 2, RED.filled()))).unwrap();
        }
        ctx.draw_series(iter::once(Circle::new((x, transformator(x, avg)), 2, BLUE.filled()))).unwrap();
    }
}

pub fn draw_chart<T>(x_range: Range<f64>, scalar: f64, values: &Vec<T>, extractor: fn(&T) -> (f64, f64, Vec<f64>), transformator: fn(f64, f64) -> f64, filename: &str){
    draw_chart2(x_range, scalar, values, extractor, transformator, filename, true);
}