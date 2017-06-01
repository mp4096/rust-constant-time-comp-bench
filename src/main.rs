#![feature(test)]

extern crate constant_time_eq;
extern crate test;


#[inline(never)]
pub fn ctc_imperative_generic<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut res = true;
    for i in 0..a.len() {
        res &= a[i].eq(&b[i]);
    }
    res
}

#[inline(never)]
pub fn ctc_imperative_specialised(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut res: u8 = 0;
    for i in 0..a.len() {
        res |= a[i] ^ b[i];
    }
    res == 0
}

#[inline(never)]
pub fn ctc_imperative_generic_no_bound_checking<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let len = a.len();
    let a = &a[..len];
    let b = &b[..len];

    let mut res = true;
    for i in 0..a.len() {
        res &= a[i].eq(&b[i]);
    }
    res
}

#[inline(never)]
pub fn ctc_imperative_specialised_no_bound_checking(a: &[u8], b: &[u8]) -> bool {
    use constant_time_eq::constant_time_eq;

    constant_time_eq(a, b)
}

#[inline(never)]
pub fn ctc_functional_generic<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    a.iter().zip(b.iter()).fold(true, |c, (x, y)| c & (x.eq(y)))
}

#[inline(never)]
pub fn ctc_functional_specialised(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    a.iter().zip(b.iter()).fold(0, |c, (x, y)| c | (x ^ y)) == 0
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const INPUT_LEN: usize = 100_000;

    #[bench]
    fn bench_functional_generic(b: &mut Bencher) {
        let l = test::black_box(vec![0; INPUT_LEN]);
        let r = test::black_box(vec![0; INPUT_LEN]);

        b.iter(|| ctc_functional_generic(&l, &r));
    }

    #[bench]
    fn bench_functional_specialised(b: &mut Bencher) {
        let l = test::black_box(vec![0; INPUT_LEN]);
        let r = test::black_box(vec![0; INPUT_LEN]);

        b.iter(|| ctc_functional_specialised(&l, &r));
    }

    #[bench]
    fn bench_imperative_generic(b: &mut Bencher) {
        let l = test::black_box(vec![0; INPUT_LEN]);
        let r = test::black_box(vec![0; INPUT_LEN]);

        b.iter(|| ctc_imperative_generic(&l, &r));
    }

    #[bench]
    fn bench_imperative_specialised(b: &mut Bencher) {
        let l = test::black_box(vec![0; INPUT_LEN]);
        let r = test::black_box(vec![0; INPUT_LEN]);

        b.iter(|| ctc_imperative_specialised(&l, &r));
    }

    #[bench]
    fn bench_imperative_generic_no_bound_checking(b: &mut Bencher) {
        let l = test::black_box(vec![0; INPUT_LEN]);
        let r = test::black_box(vec![0; INPUT_LEN]);

        b.iter(|| ctc_imperative_generic_no_bound_checking(&l, &r));
    }

    #[bench]
    fn bench_imperative_specialised_no_bound_checking(b: &mut Bencher) {
        let l = test::black_box(vec![0; INPUT_LEN]);
        let r = test::black_box(vec![0; INPUT_LEN]);

        b.iter(|| ctc_imperative_specialised_no_bound_checking(&l, &r));
    }
}
