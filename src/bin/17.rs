use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashSet},
};

use anyhow::Result;
use itertools::Itertools;

const DAY: u8 = 17;

type Vec2 = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rev(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn perp(&self) -> [Self; 2] {
        match self {
            Direction::North | Direction::South => [Direction::East, Direction::West],
            Direction::East | Direction::West => [Direction::North, Direction::South],
        }
    }
}

fn get_idx(pos: Vec2, dim: Vec2) -> usize {
    pos.0 * dim.1 + pos.1
}

fn get_next_pos(pos: Vec2, dim: Vec2, dir: Direction, amount: usize) -> Option<Vec2> {
    match dir {
        Direction::North if pos.0 >= amount => Some((pos.0 - amount, pos.1)),
        Direction::South if pos.0 < dim.0 - amount => Some((pos.0 + amount, pos.1)),
        Direction::East if pos.1 < dim.1 - amount => Some((pos.0, pos.1 + amount)),
        Direction::West if pos.1 >= amount => Some((pos.0, pos.1 - amount)),
        _ => None,
    }
}

fn part_1(input: &str) -> Result<u32> {
    const LEN_START: usize = 1;
    const LEN_END: usize = 3;
    let dim = (input.lines().count(), input.lines().next().unwrap().len());
    let table = input
        .lines()
        .flat_map(str::bytes)
        .map(|b| b - b'0')
        .collect_vec();
    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();
    pq.push(Reverse((0, (0, 0), Direction::North)));
    while let Some(Reverse((heat, pos, dir))) = pq.pop() {
        if visited.get(&(pos, dir)).is_some() || visited.get(&(pos, dir.rev())).is_some() {
            continue;
        }
        visited.insert((pos, dir));
        if pos == (dim.0 - 1, dim.1 - 1) {
            return Ok(heat);
        }
        for next_dir in dir.perp() {
            let mut next_heat = heat;
            for i in 1..=LEN_END {
                let next_pos;
                if let Some(np) = get_next_pos(pos, dim, next_dir, i) {
                    next_pos = np;
                } else {
                    break;
                }
                next_heat += table[get_idx(next_pos, dim)] as u32;
                if i < LEN_START {
                    continue;
                }
                // if visited.get(&(next_pos, next_dir)).is_some() {
                //     continue;
                // }
                pq.push(Reverse((next_heat, next_pos, next_dir)));
            }
        }
    }
    anyhow::bail!("Did not find a path");
}

fn part_2(input: &str) -> Result<u32> {
    const LEN_START: usize = 4;
    const LEN_END: usize = 10;
    let dim = (input.lines().count(), input.lines().next().unwrap().len());
    let table = input
        .lines()
        .flat_map(str::bytes)
        .map(|b| b - b'0')
        .collect_vec();
    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();
    pq.push(Reverse((0, (0, 0), Direction::North)));
    while let Some(Reverse((heat, pos, dir))) = pq.pop() {
        if visited.get(&(pos, dir)).is_some() || visited.get(&(pos, dir.rev())).is_some() {
            continue;
        }
        visited.insert((pos, dir));
        if pos == (dim.0 - 1, dim.1 - 1) {
            return Ok(heat);
        }
        for next_dir in dir.perp() {
            let mut next_heat = heat;
            for i in 1..=LEN_END {
                let next_pos;
                if let Some(np) = get_next_pos(pos, dim, next_dir, i) {
                    next_pos = np;
                } else {
                    break;
                }
                next_heat += table[get_idx(next_pos, dim)] as u32;
                if i < LEN_START {
                    continue;
                }
                // if visited.get(&(next_pos, next_dir)).is_some() {
                //     continue;
                // }
                pq.push(Reverse((next_heat, next_pos, next_dir)));
            }
        }
    }
    anyhow::bail!("Did not find a path");
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
        let res = "102";
        aoc::assert_output_matches_str(DAY, "example1", part_1, res)?;
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let res = "94";
        aoc::assert_output_matches_str(DAY, "example2", part_2, res)?;
        Ok(())
    }
}
