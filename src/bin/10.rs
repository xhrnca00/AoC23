use std::{collections::VecDeque, vec};

use anyhow::Result;
use itertools::Itertools;

const DAY: u8 = 10;

fn offset_ok(offset: isize, idx: usize, width: usize, max_len: usize) -> bool {
    let idx = idx as isize;
    let width = width as isize;
    let x = idx % width;
    let max_len = max_len as isize;
    !((idx + offset) < 0
        || (idx + offset) >= max_len
        || (x == width - 1 && offset == 1)
        || (x == 0 && offset == -1))
}

fn get_connections(c: u8, idx: usize, width: usize, max_len: usize) -> Vec<usize> {
    let iwidth = width as isize;
    let offsets = match c {
        b'S' => vec![1, -1, iwidth, -iwidth],
        b'|' => vec![iwidth, -iwidth],
        b'-' => vec![1, -1],
        b'J' => vec![-1, -iwidth],
        b'L' => vec![1, -iwidth],
        b'7' => vec![-1, iwidth],
        b'F' => vec![1, iwidth],
        b'.' => vec![],
        _ => unreachable!("Invalid char: {}", c),
    };
    offsets
        .into_iter()
        .filter(|&o| offset_ok(o, idx, width, max_len))
        .map(|o| (o + idx as isize) as usize)
        .collect()
}

fn part_1(input: &str) -> Result<u32> {
    let width = input.lines().next().unwrap().len();
    let table = input.lines().flat_map(|l| l.bytes()).collect_vec();
    let start = table.iter().position(|&c| c == b'S').unwrap();
    let mut a_list = table
        .iter()
        .enumerate()
        .map(|(i, &c)| get_connections(c, i, width, table.len()))
        .collect_vec();
    a_list[start] = a_list[start]
        .clone()
        .into_iter()
        .filter(|&a| a_list[a].iter().find(|&&elem| elem == start).is_some())
        .collect_vec();
    let mut visited = vec![false; table.len()];
    let mut q = VecDeque::with_capacity(2);
    let mut lens = vec![0u32; table.len()];
    q.push_back((start, 0u32));
    while let Some((v, len)) = q.pop_front() {
        if visited[v] {
            return Ok(len);
        }
        lens[v] = len;
        visited[v] = true;
        for &a in &a_list[v] {
            if !visited[a] {
                q.push_back((a, len + 1));
            }
        }
    }
    anyhow::bail!("No path found")
}

fn c_from_cons(idx: usize, connections: &[usize]) -> u8 {
    assert_eq!(connections.len(), 2);
    let diff0 = connections[0] as isize - idx as isize;
    let diff1 = connections[1] as isize - idx as isize;
    match (diff0, diff1) {
        (1, -1) | (-1, 1) => b'-',
        (x, y) if x == -y => b'|',
        (1, x) | (x, 1) if x > 0 => b'F',
        (1, _) | (_, 1) => b'L',
        (-1, x) | (x, -1) if x > 0 => b'7',
        (-1, _) | (_, -1) => b'J',
        _ => unreachable!("Invalid connections: {:?}", connections),
    }
}

fn part_2(input: &str) -> Result<u32> {
    let width = input.lines().next().unwrap().len();
    let mut table = input.lines().flat_map(|l| l.bytes()).collect_vec();
    let start = table.iter().position(|&c| c == b'S').unwrap();
    let mut a_list = table
        .iter()
        .enumerate()
        .map(|(i, &c)| get_connections(c, i, width, table.len()))
        .collect_vec();
    a_list[start] = a_list[start]
        .clone()
        .into_iter()
        .filter(|&a| a_list[a].iter().find(|&&elem| elem == start).is_some())
        .collect_vec();
    table[start] = c_from_cons(start, &a_list[start]);
    let mut visited = vec![false; table.len()];
    let mut s = Vec::with_capacity(2);
    s.push((start, a_list[start][0]));
    while let Some((v, prev)) = s.pop() {
        if visited[v] {
            break;
        }
        visited[v] = true;
        debug_assert!(a_list[v].len() == 2);
        s.push((*a_list[v].iter().filter(|&&a| a != prev).next().unwrap(), v));
    }
    for i in 0..table.len() {
        table[i] = match (visited[i], table[i]) {
            (false, _) => b'.',
            (true, b'-' | b'F' | b'7') => b'h',
            (true, _) => b'v',
        }
    }
    let mut sum = 0;
    for line in table.chunks(width) {
        let mut counting = false;
        for c in line {
            if *c == b'v' {
                counting = !counting;
            } else if counting && *c == b'.' {
                sum += 1;
            }
        }
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
        let res = "8";
        aoc::assert_output_matches_str(DAY, "example1", part_1, res)?;
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let res = "10";
        aoc::assert_output_matches_str(DAY, "example2", part_2, res)?;
        Ok(())
    }
}
