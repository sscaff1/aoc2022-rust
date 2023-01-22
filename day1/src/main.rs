// use std::{io::Error, fs::read_to_string};
use itertools::Itertools;
use std::cmp::Reverse;


fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("input.txt");
    // imperative method
    // let mut max = 0;
    // for group in input
    //     .replace("\r\n", "\n")
    //     .split("\n\n") {
    //         let mut sum = 0;
    //         for line in group.lines() {
    //             let value = line.parse::<u64>()?;
    //             sum += value;
    //         }
    //         if sum > max {
    //             max = sum;
    //         }
    // }
    // solution 2
    // let lines = input
    //     .lines()
    //     .map(|l| l.parse::<u64>().ok())
    //     .collect::<Vec<_>>();
    // let maxInGroups = lines
    //     .split(|line| line.is_none())
    //     .map(|group| group.iter().map(|l| l.unwrap()).sum::<u64>())
    //     .max();

    // println!("max in groups = {maxInGroups:?}");

    // println!("The burdenedst elf is carrying {max} calories");

    // for line in lines {
    //     println!("Got line: {line}");
    // }

    // final solution
    // let max = input
    //     .lines()
    //     .map(|l| l.parse::<u64>().ok())
    //     .batching(|l| {
    //         let mut sum = None;
    //         while let Some(Some(v)) = l.next() {
    //             sum = Some(sum.unwrap_or(0) + v);
    //         }
    //         sum
    //     })
    //     .max();
    // coalesce
    // let max = input
    //     .lines()
    //     .map(|l| l.parse::<u64>().ok())
    //     .coalesce(|a, b| match (a, b) {
    //         (None, None) => Ok(None),
    //         (None, Some(b)) => Ok(Some(b)),
    //         (Some(a), None) => Err((Some(a), None)),
    //         (Some(a), Some(b)) => Ok(Some(a + b))
    //     })
    //     .max()
    //     .flatten()
    //     .unwrap_or_default();
    //     println!("{max:?}");

    // taking the top 3
    // let answer = input
    //     .lines()
    //     .map(|l| l.parse::<u64>().ok())
    //     .coalesce(|a, b| match(a, b) {
    //         (None, None) => Ok(None),
    //         (None, Some(b)) => Ok(Some(b)),
    //         (Some(a), None) => Err((Some(a), None)),
    //         (Some(a), Some(b)) => Ok(Some(a + b))
    //     })
    //     .flatten()
    //     .sorted_by_key(|&v| std::cmp::Reverse(v))
    //     .take(3)
    //     .sum::<u64>();
    // println!("{answer}");

    // final final
    let answer = input
        .lines()
        .map(|l| l.parse::<u64>().ok())
        .batching(|it| it.map_while(|x| x).sum1::<u64>())
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u64>();
    println!("{answer:?}");
    Ok(())
}
