#![crate_id = "prob0105"]
#![crate_type = "rlib"]

use std::io::{BufferedReader, File};
use std::iter::AdditiveIterator;

pub static EXPECTED_ANSWER: &'static str = "73702";

fn is_sss(nums: &[uint]) -> bool {
    let mut sums = vec![0u];
    for &n in nums.iter() {
        let mut i = 0;
        let mut j = 0;
        let len = sums.len();
        let mut new_sums = Vec::with_capacity(len * 2);
        while i < len {
            assert!(j <= i);
            match sums.get(i).cmp(&(*sums.get(j) + n)) {
                Equal => { return false; }
                Less => {
                    new_sums.push(*sums.get(i));
                    i += 1;
                }
                Greater => {
                    new_sums.push(*sums.get(j) + n);
                    j += 1;
                }
            }
        }

        while j < len {
            new_sums.push(*sums.get(j) + n);
            j += 1;
        }

        sums = new_sums;
    }

    true
}

pub fn solve() -> StrBuf {
    let mut br = BufferedReader::new(
        File::open(&Path::new("files/sets.txt")).ok().expect("file not found."));

    br.lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.as_slice()
                .trim()
                .split(',')
                .filter_map(from_str::<uint>)
                .collect::<Vec<uint>>()
        }).map(|mut nums| { nums.sort(); nums })
        .filter(|nums| {
            let len = nums.len();
            let len_hd = (len + 1) / 2;
            let len_tl = len_hd - 1;
            let mut hd = nums.slice(0, len_hd).iter().map(|&x| x);
            let mut tl = nums.slice(len - len_tl, len).iter().map(|&x| x);
            hd.sum() > tl.sum()
        }).filter(|nums| is_sss(nums.as_slice()))
        .map(|nums| nums.iter().map(|&x| x).sum())
        .sum()
        .to_str()
}
