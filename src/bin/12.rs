use std::{collections::BTreeMap, iter::repeat};

use anyhow::Result;
use itertools::Itertools;

const DAY: u8 = 12;

fn part_1(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .map(|l| {
            let (chars, seq) = l.split_once(' ').unwrap();
            let seq = seq
                .split(',')
                .map(str::parse::<u8>)
                .map(Result::unwrap)
                .collect_vec();
            let mut max = 0;
            let mut curr = vec![(0, 0)];
            for b in chars.bytes() {
                let mut new = vec![];
                if b != b'.' {
                    for &(idx, count) in &curr {
                        if idx < seq.len() && count < seq[idx] {
                            new.push((idx, count + 1));
                        }
                    }
                }
                if b != b'#' {
                    for &(idx, count) in &curr {
                        if count == 0 {
                            new.push((idx, count))
                        } else if count == seq[idx] {
                            new.push((idx + 1, 0))
                        }
                    }
                }
                max = max.max(new.len());
                curr = new;
            }
            curr.into_iter()
                .filter(|(idx, count)| {
                    *idx == seq.len() || (*idx == seq.len() - 1 && *count == seq[*idx])
                })
                .count() as u32
        })
        .sum())
}

fn part_2(input: &str) -> Result<u64> {
    Ok(input
        .lines()
        .map(|l| {
            let (chars, seq) = l.split_once(' ').unwrap();
            let seq = repeat(seq)
                .take(5)
                .flat_map(|s| s.split(','))
                .map(str::parse::<u8>)
                .map(Result::unwrap)
                .collect_vec();
            let mut curr = BTreeMap::from([((0, 0), 1)]);
            let len = chars.bytes().len() * 5 + 4;
            let mut bytes = chars.bytes().chain("?".bytes()).cycle();
            for _ in 0..len {
                let b = bytes.next().unwrap();
                let mut new = BTreeMap::new();
                if b != b'.' {
                    for (&(idx, count), &amt) in &curr {
                        if idx < seq.len() && count < seq[idx] {
                            *new.entry((idx, count + 1)).or_insert(0) += amt;
                        }
                    }
                }
                if b != b'#' {
                    for (&(idx, count), &amt) in &curr {
                        if count == 0 {
                            *new.entry((idx, count)).or_insert(0) += amt;
                        } else if count == seq[idx] {
                            *new.entry((idx + 1, 0)).or_insert(0) += amt;
                        }
                    }
                }
                curr = new;
            }
            curr.into_iter()
                .filter(|&((idx, count), _)| {
                    idx == seq.len() || (idx == seq.len() - 1 && count == seq[idx])
                })
                .map(|(_, amt)| amt)
                .sum::<u64>()
        })
        .sum())
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
        let res = "21";
        aoc::assert_output_matches_str(DAY, "example1", part_1, res)?;
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let res = "525152";
        aoc::assert_output_matches_str(DAY, "example2", part_2, res)?;
        Ok(())
    }
}
