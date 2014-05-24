#![crate_id = "prob0065"]
#![crate_type = "rlib"]

extern crate num;
extern crate math;

use std::char;
use std::iter::AdditiveIterator;
use num::bigint::BigUint;
use math::cont_frac;

pub static EXPECTED_ANSWER: &'static str = "272";

fn napier_seq(i: uint) -> uint {
    match i {
        0 => 2,
        i if i % 3 == 2 => 2 * (i + 1) / 3,
        _ => 1
    }
}

pub fn solve() -> StrBuf {
    let len = 100;

    let napier = Vec::from_fn(len, napier_seq);

    let (n, _d) = cont_frac::fold::<BigUint>(napier.as_slice());
    n.to_str()
        .as_slice()
        .chars()
        .filter_map(|c| char::to_digit(c, 10))
        .sum()
        .to_str()
}

