#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate common;

use common::Solver;


// x by y => C = (1 + 2 + .. + x) * (1 + 2 + .. + y) recutangulars
//             = (x (1 + x) / 2) * (y (1 + y)) / 2
//             = xy (1 + x)(1 + y) / 4
fn count_rect((x, y): (uint, uint)) -> uint {
    x * y * (1 + x) * (1 + y) / 4
}

fn distance(a: uint, target: uint) -> uint {
    if a > target { a - target } else { target - a }
}

pub fn check_distance(min_dist: &mut uint, min_pos: &mut (uint, uint), pos: (uint, uint), target: uint) {
    let dist = distance(count_rect(pos), target);
    if dist < *min_dist {
        *min_dist = dist;
        *min_pos  = pos;
    }
}

pub fn solve() -> String {
    let target = 2000000;

    let mut x = 1;
    let mut y = 1;

    while count_rect((x, y)) < target { x += 1; y += 1; }
    assert!(count_rect((x, y)) >= target);

    let mut nearest = (x, y);
    let mut dist = distance(count_rect(nearest), target);
    while x >= 1 {
        while count_rect((x, y)) < target { y += 1; }
        assert!(count_rect((x, y)) >= target);
        check_distance(&mut dist, &mut nearest, (x, y), target);
        check_distance(&mut dist, &mut nearest, (x, y - 1), target);
        x -= 1;
    }

    let (x, y) = nearest;
    (x * y).to_string()
}

fn main() { Solver::new("2772", solve).run(); }