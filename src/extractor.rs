use std::ops::Range;
use crate::result::SimResult;

pub(crate) fn comp(value: &SimResult) -> (f64, f64, Vec<f64>) {
    let x = value.x as f64;
    let avg = value.compavg;
    let mut vals = value.comps.iter().map(|count| count.to_owned() as f64).collect();
    return (x, avg, vals);
}

pub(crate) fn swap(value: &SimResult) -> (f64, f64, Vec<f64>) {
    let x = value.x as f64;
    let avg = value.swapavg;
    let mut vals = value.swaps.iter().map(|count| count.to_owned() as f64).collect();
    return (x, avg, vals);
}

pub(crate) fn create_data(values: &Vec<SimResult>, extractor: fn(&SimResult) -> (f64, f64, Vec<f64>), transformator: fn(f64, f64) -> f64) -> (Range<f64>, Vec<(f64, f64, Vec<f64>)>) {
    let mut min = f64::INFINITY;
    let mut max = 0.0;
    let mut points = Vec::new();
    for value in values {
        let (x, avg, vals) = extractor(value);
        let mut results: Vec<f64> = Vec::new();
        vals.iter().for_each(|val| {
            let val = transformator(x, *val);
            if val < min {
                min = val;
            }else if val > max {
                max = val;
            }
            results.push(val);
        });
        points.push((x, transformator(x,avg), results));
    }
    return (min..max, points);
}
