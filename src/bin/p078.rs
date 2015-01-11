//! [Problem 78](https://projecteuler.net/problem=78) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#[macro_use(problem)] extern crate common;

use std::iter;

const MILLION: i32 = 1000000;

fn penta(n: i32) -> i32 { n * (3 * n - 1) / 2 }

fn solve() -> String {
    let mut v = [0; 65536];
    v[0] = 1;

    for n in iter::count(1, 1) {
        let mut way = 0;

        for i in iter::count(0, 1) {
            let k = i % 4;
            let p = if k == 0 || k == 2 {
                penta(i / 2 + 1)
            } else {
                penta(-i / 2 - 1)
            };
            if p > n { break; }

            way = match k {
                0 => way + v[(n - p) as uint],
                1 => way + v[(n - p) as uint],
                2 => way - v[(n - p) as uint],
                _ => way - v[(n - p) as uint]
            } % MILLION
        }
        v[n as uint] = way;

        if way == 0 { return n.to_string() }
    }

    unreachable!()
}

problem!("55374", solve);
