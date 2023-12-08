use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;
use num::integer::lcm;

const DAY: u8 = 08;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => anyhow::bail!("Invalid instruction: {}", c),
        }
    }
}

fn part_1(input: &str) -> Result<u32> {
    let mut lines = input.lines();
    let mut instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| Instruction::try_from(c).unwrap())
        .cycle();
    lines.next().unwrap();
    let graph = lines
        .map(|l| l.split(" = ").collect_tuple().unwrap())
        .map(|(name, edges)| {
            (
                name,
                edges[1..edges.len() - 1]
                    .split(", ")
                    .collect_tuple()
                    .unwrap(),
            )
        })
        .collect::<HashMap<_, (_, _)>>();
    let mut curr = "AAA";
    let mut steps = 0;
    while curr != "ZZZ" {
        if instructions.next().unwrap() == Instruction::Left {
            curr = graph[curr].0;
        } else {
            curr = graph[curr].1;
        }
        steps += 1;
    }
    Ok(steps)
}

fn part_2(input: &str) -> Result<u64> {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| Instruction::try_from(c).unwrap())
        .cycle();
    lines.next().unwrap();
    let mut starts = vec![];
    let graph = lines
        .map(|l| l.split(" = ").collect_tuple().unwrap())
        .map(|(name, edges)| {
            (
                name,
                edges[1..edges.len() - 1]
                    .split(", ")
                    .collect_tuple()
                    .unwrap(),
            )
        })
        .inspect(|(name, _)| {
            if name.ends_with('A') {
                starts.push(*name);
            }
        })
        .collect::<HashMap<_, (_, _)>>();
    let mut steps = vec![];
    for mut curr in starts {
        let mut instructions = instructions.clone();
        let mut step = 0;
        while !curr.ends_with('Z') {
            if instructions.next().unwrap() == Instruction::Left {
                curr = graph[curr].0;
            } else {
                curr = graph[curr].1;
            }
            step += 1;
        }
        steps.push(step);
    }
    Ok(steps.into_iter().fold(1, lcm))
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
        let res = "6";
        aoc::assert_output_matches_str(DAY, "example1", part_1, res)?;
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let res = "6";
        aoc::assert_output_matches_str(DAY, "example2", part_2, res)?;
        Ok(())
    }
}
