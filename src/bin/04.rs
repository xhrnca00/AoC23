use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;

const DAY: u8 = 04;

fn part_1(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .map(|l| l.split(": ").nth(1).unwrap())
        .map(|l| {
            l.split(" | ")
                .map(|nums| nums.split(' ').filter(|&n| n != "").collect::<HashSet<_>>())
                .collect_tuple()
                .unwrap()
        })
        .map(|(winning, ours)| winning.intersection(&ours).count() as u32)
        .filter(|&count| count > 0)
        .map(|count| 1 << (count - 1))
        .sum())
}

fn part_2(input: &str) -> Result<u32> {
    let mut sum = 0;
    let mut next_count = input
        .lines()
        .map(|l| l.split(": ").nth(1).unwrap())
        .map(|l| {
            l.split(" | ")
                .map(|nums| nums.split(' ').filter(|&n| n != "").collect::<HashSet<_>>())
                .collect_tuple()
                .unwrap()
        })
        .map(|(winning, ours)| winning.intersection(&ours).count() as u32);
    let mut counts = vec![1; input.lines().count()];
    let mut i = 0;
    while let Some(count) = next_count.next() {
        sum += counts[i];
        for j in 0..(count as usize).min(counts.len()) {
            counts[i + j + 1] += counts[i];
        }
        i += 1;
    }
    Ok(sum)
}

fn main() -> Result<()> {
    aoc::solve_all(DAY, part_1, part_2)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        let res = "13";
        aoc::assert_output_matches_str(DAY, "example1", part_1, res)?;
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let res = "30";
        aoc::assert_output_matches_str(DAY, "example2", part_2, res)?;
        Ok(())
    }
}
