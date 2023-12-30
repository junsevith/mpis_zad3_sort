use std::iter::StepBy;
use std::ops::RangeInclusive;
use rand::prelude::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_pcg::*;
use crate::result::SimResult;

fn random_array(size: usize, rng: &mut impl Rng) -> Vec<usize> {
    let mut arr = (1..=size).collect::<Vec<usize>>();
    arr.shuffle(rng);
    return arr;
}

fn insertion_sort(arr: &mut Vec<usize>) -> [usize; 2] {
    let mut comps: usize = 0;
    let mut swaps: usize = 0;
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && {
            comps += 1;
            arr[j - 1] > arr[j]
        } {
            swaps += 1;
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
    return [comps, swaps];
}

fn simulate(size: usize, rng: &mut impl Rng) -> [usize; 2] {
    let mut arr = random_array(size, rng);
    return insertion_sort(&mut arr);
}

fn many_sim(size: usize, repetitions: usize, rng: &mut impl Rng) -> SimResult {
    let mut result = SimResult::new(size);
    for _i in 0..repetitions {
        let res = simulate(size, rng);
        result.add(res[0], res[1]);
    }
    result.average(repetitions);
    return result;
}

pub(crate) fn range_sim(range: StepBy<RangeInclusive<usize>>, repetitions: usize) -> Vec<SimResult> {
    let mut results = Vec::new();
    let mut rng = Pcg64Mcg::from_entropy();
    for i in range {
        let res = many_sim(i, repetitions, &mut rng);
        results.push(res);
    }
    return results;
}