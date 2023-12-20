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
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

fn get_idx(pos: Vec2, dim: Vec2) -> usize {
    pos.0 * dim.1 + pos.1
}

fn get_next_pos(pos: Vec2, dim: Vec2, dir: Direction) -> Option<Vec2> {
    match dir {
        Direction::North if pos.0 > 0 => Some((pos.0 - 1, pos.1)),
        Direction::South if pos.0 < dim.0 - 1 => Some((pos.0 + 1, pos.1)),
        Direction::East if pos.1 < dim.1 - 1 => Some((pos.0, pos.1 + 1)),
        Direction::West if pos.1 > 0 => Some((pos.0, pos.1 - 1)),
        _ => None,
    }
}

fn part_1(input: &str) -> Result<u32> {
    const MAX_LEN: u8 = 3;
    let dim = (input.lines().count(), input.lines().next().unwrap().len());
    let table = input
        .lines()
        .flat_map(str::bytes)
        .map(|b| b - b'0')
        .collect_vec();
    let mut pq = BinaryHeap::new();
    let mut visited = BTreeSet::new();
    pq.push(Reverse((0, (0, 0), Direction::North, 0)));
    while let Some(Reverse((heat, pos, dir, len))) = pq.pop() {
        if visited.get(&(pos, dir, len)).is_some() {
            continue;
        }
        visited.insert((pos, dir, len));
        if pos == (dim.0 - 1, dim.1 - 1) {
            return Ok(heat);
        }
        for next_dir in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            if next_dir == dir.opposite() {
                continue;
            }
            let next_pos = get_next_pos(pos, dim, next_dir);
            if next_pos.is_none() {
                continue;
            }
            let next_pos = next_pos.unwrap();
            let next_heat = heat + table[get_idx(next_pos, dim)] as u32;
            if next_dir == dir && len == MAX_LEN {
                continue;
            }
            let next_ins;
            if dir == next_dir {
                next_ins = (next_heat, next_pos, next_dir, len + 1);
            } else {
                next_ins = (next_heat, next_pos, next_dir, 1);
            }
            if visited.get(&(next_ins.1, next_ins.2, next_ins.3)).is_some() {
                continue;
            }
            // visited.insert((next_ins.1, next_ins.2, next_ins.3));
            pq.push(Reverse(next_ins));
        }
    }
    anyhow::bail!("Did not find a path");
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
        let res = "102";
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
