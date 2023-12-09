use std::iter::zip;

use anyhow::Result;
use itertools::Itertools;

const DAY: u8 = 06;

fn part_1(input: &str) -> Result<u32> {
    let (times, distances) = input.lines().collect_tuple().unwrap();
    Ok(zip(
        times.split_ascii_whitespace(),
        distances.split_ascii_whitespace(),
    )
    .skip(1)
    .map(|(t, d)| (t.parse::<f32>().unwrap(), d.parse::<f32>().unwrap()))
    .map(|(t, d)| {
        let discriminant = t * t - 4.0 * d;
        (
            (t + discriminant.sqrt()) / 2.0,
            (t - discriminant.sqrt()) / 2.0,
        )
    })
    .map(|(a, b)| (a.ceil() - b.floor()) as u32 - 1)
    .product())
}

fn part_2(input: &str) -> Result<u32> {
    let (time, distance) = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .skip(1)
                .join("")
                .parse::<f64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();
    let discriminant = time * time - 4.0 * distance;
    let (a, b) = (
        (time + discriminant.sqrt()) / 2.0,
        (time - discriminant.sqrt()) / 2.0,
    );
    Ok((a.ceil() - b.floor()) as u32 - 1)
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
        let res = "288";
        aoc::assert_output_matches_str(DAY, "example1", part_1, res)?;
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let res = "71503";
        aoc::assert_output_matches_str(DAY, "example2", part_2, res)?;
        Ok(())
    }
}
