#![feature(test)]

extern crate test;

use test::Bencher;
use std::hint::black_box;
use rust_max_bench::{custom_max, stdlib_max};

pub const ITEM_COUNT : i64 = 100_000;

#[bench]
fn stdlib(bencher: &mut Bencher) {
    let my_array = (0..ITEM_COUNT).collect::<Vec<_>>();

    bencher.iter(|| {
        black_box(stdlib_max(&my_array));
    });
}

#[bench]
fn custom(bencher: &mut Bencher) {
    let my_array = (0..ITEM_COUNT).collect::<Vec<_>>();

    bencher.iter(|| {
        black_box(custom_max(&my_array));
    });
}

#[bench]
fn itertools(bencher: &mut Bencher) {
    let my_array = (0..ITEM_COUNT).collect::<Vec<_>>();

    bencher.iter(|| {
        black_box(itertools::max(&my_array));
    });
}