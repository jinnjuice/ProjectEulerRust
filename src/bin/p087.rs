#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate common;
extern crate prime;

use common::Solver;
use prime::PrimeSet;

fn compute(limit: u64) -> uint {
    let prime = PrimeSet::new();
    let mut cnt = 0u;
    let mut set = Vec::from_elem(limit as uint, false);

    for p in prime.iter() {
        let p4 = p * p * p * p;
        if p4 >= limit { break }

        for q in prime.iter() {
            let q3 = q * q * q;
            if p4 + q3 >= limit { break }

            for r in prime.iter() {
                let r2 = r * r;
                let s = p4 + q3 + r2;
                if s >= limit { break }

                if set[s as uint] { continue }
                set[s as uint] = true;
                cnt += 1;
            }
        }
    }

    cnt
}

fn solve() -> String {
    compute(50000000).to_string()
}

fn main() { Solver::new("1097343", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn fifty() {
        assert_eq!(4, super::compute(50));
    }
}
