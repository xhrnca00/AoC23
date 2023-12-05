use std::collections::BTreeMap;

use anyhow::Result;
use itertools::Itertools;

const DAY: u8 = 05;

fn part_1(input: &str) -> Result<i64> {
    let mut lines = input.lines().peekable();
    let mut nums = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    lines.next(); // empty line
    while lines.next().is_some() {
        let mut map = BTreeMap::new();
        while let Some(l) = lines.next() {
            if l.is_empty() {
                break;
            }
            let (dst, src, len) = l
                .split(' ')
                .map(|s| s.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();
            map.insert(src, (len, dst - src));
        }
        for n in nums.iter_mut() {
            if let Some((key, (len, offset))) = map.range(..=*n).next_back() {
                if *n < *key + *len {
                    *n += offset;
                    continue;
                }
            }
        }
    }
    Ok(nums.into_iter().min().unwrap())
}

fn part_2(input: &str) -> Result<u32> {
    todo!("Implement part 2. Input len: {}", input.len())
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
        let res = "35";
        aoc::assert_output_matches_str(DAY, "example1", part_1, res)?;
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let res = "2";
        aoc::assert_output_matches_str(DAY, "example2", part_2, res)?;
        Ok(())
    }
}
