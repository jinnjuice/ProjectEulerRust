#![crate_name = "prob0067"]
#![crate_type = "rlib"]

use std::cmp;
use std::io::{BufferedReader, File};

pub static EXPECTED_ANSWER: &'static str = "7273";

pub fn solve() -> String {
    let mut br = BufferedReader::new(
        File::open(&Path::new("files/p067_triangle.txt")).ok().expect("file not found."));

    let triangle = br.lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .map(|line| line.as_slice().words().filter_map(from_str::<uint>).collect::<Vec<uint>>())
        .collect::<Vec<Vec<uint>>>();
    let init = triangle.init();
    let last = triangle.last().unwrap().clone();
    init.iter()
        .rev()
        .fold(last, |prev, elem| {
            Vec::from_fn(elem.len(), |i| elem[i] + cmp::max(prev[i], prev[i + 1]))
        })[0]
        .to_string()
}
