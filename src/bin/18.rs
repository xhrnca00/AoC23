use std::{mem, str::FromStr};

use anyhow::Result;
use itertools::Itertools;

const DAY: u8 = 18;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    R,
    D,
    L,
    U,
}

impl FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Dir::R),
            "L" => Ok(Dir::L),
            "D" => Ok(Dir::D),
            "U" => Ok(Dir::U),
            _ => anyhow::bail!("Invalid direction: {}", s),
        }
    }
}

/// Change the first character of the string to a `Dir` variant.
///
/// ## Safety
///
/// The first byte of `hex` **must** be one of `b"0123"`.
unsafe fn hex_to_dir(hex: &str) -> Dir {
    mem::transmute(hex.bytes().next().unwrap() - b'0')
}

fn part_1(input: &str) -> Result<i32> {
    let (perim, area, _) = input
        .lines()
        .map(|l| l.split(' ').collect_tuple().unwrap())
        .map(|(dir, len, _)| (dir.parse().unwrap(), len.parse::<i32>().unwrap()))
        .fold(
            (0, 0, (0, 0)),
            |(perim, area, (y, x)), (dir, len)| match dir {
                Dir::R => (perim + len, area, (y, x + len)),
                Dir::L => (perim + len, area, (y, x - len)),
                Dir::D => (perim + len, area + x * len, (y + len, x)),
                Dir::U => (perim + len, area - x * len, (y - len, x)),
            },
        );
    Ok(area + perim / 2 + 1)
}

fn part_2(input: &str) -> Result<i64> {
    let (perim, area, _) = input
        .lines()
        .map(|l| l.split(' ').collect_tuple().unwrap())
        .map(|(_, _, hex)| {
            (
                unsafe { hex_to_dir(&hex[7..8]) },
                i64::from_str_radix(&hex[2..7], 16).unwrap(),
            )
        })
        .fold(
            (0, 0, (0, 0)),
            |(perim, area, (y, x)), (dir, len)| match dir {
                Dir::R => (perim + len, area, (y, x + len)),
                Dir::L => (perim + len, area, (y, x - len)),
                Dir::D => (perim + len, area + x * len, (y + len, x)),
                Dir::U => (perim + len, area - x * len, (y - len, x)),
            },
        );
    Ok(area + perim / 2 + 1)
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
        let res = "62";
        aoc::assert_output_matches_str(DAY, "example1", part_1, res)?;
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let res = "952408144115";
        aoc::assert_output_matches_str(DAY, "example2", part_2, res)?;
        Ok(())
    }
}
