use anyhow::Result;
use itertools::Itertools;

pub fn solve() -> Result<()> {
    let lines = include_str!("d01.txt")
        .lines()
        .map(|line| line.split_once(" ").expect("split_once"))
        .map(|(a, b)| (num(a), num(b)))
        .collect_vec();

    let left = lines.iter().map(|(a, _)| a).copied().sorted().collect_vec();
    let right = lines.iter().map(|(_, b)| b).copied().sorted().collect_vec();

    println!(
        "{}",
        left.iter()
            .zip(right.iter())
            .map(|(a, b)| (a - b).abs())
            .sum::<i64>()
    );

    println!(
        "{}",
        left.iter()
            .map(|&a| right.iter().filter(|&&b| b == a).count() as i64 * a)
            .sum::<i64>()
    );

    Ok(())
}

fn num(s: &str) -> i64 {
    s.trim().parse().expect(&format!("parse: {s}"))
}
