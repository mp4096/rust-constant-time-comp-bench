extern crate constant_time_eq;
extern crate chrono;


use chrono::{DateTime, Local};
use constant_time_eq::constant_time_eq;

const NUM_TRIALS: usize = 50000;
const VEC_LEN: usize = 1000;

fn main() {
    let mut time_imperative_specialised: Vec<usize> = Vec::with_capacity(NUM_TRIALS);
    let mut time_imperative_generic: Vec<usize> = Vec::with_capacity(NUM_TRIALS);
    let mut time_functional_generic: Vec<usize> = Vec::with_capacity(NUM_TRIALS);

    for _ in 0..NUM_TRIALS {
        for l in vec![VEC_LEN] {
            let left: Vec<u8> = vec![0; l];
            let mut right: Vec<u8> = vec![0; l];
            right[0] = 1;

            let tic = Local::now();
            ctc_imperative_generic(&left, &right);
            time_imperative_generic.push(count_ns_since_datetime(tic));

            let tic = Local::now();
            ctc_functional_generic(&left, &right);
            time_functional_generic.push(count_ns_since_datetime(tic));

            let tic = Local::now();
            constant_time_eq(&left, &right);
            time_imperative_specialised.push(count_ns_since_datetime(tic));
        }
    }

    println!("Avg. imperative:       {} ns", mean(&time_imperative_generic));
    println!("Avg. functional:       {} ns", mean(&time_functional_generic));
    println!("Avg. constant_time_eq: {} ns", mean(&time_imperative_specialised));
}

fn ctc_imperative_generic<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut res = true;
    for i in 0..a.len() {
        res &= a[i].eq(&b[i]);
    }
    res
}

fn ctc_functional_generic<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    a.iter().zip(b.iter()).fold(true, |c, (x, y)| c & (x.eq(y)))
}

#[inline]
fn count_ns_since_datetime(dt: DateTime<Local>) -> usize {
    Local::now().signed_duration_since(dt).num_nanoseconds().unwrap_or(0) as usize
}

fn mean(x: &[usize]) -> f64 {
    (x.iter().sum::<usize>() as f64)/(x.len() as f64)
}
