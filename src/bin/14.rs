use std::collections::BTreeMap;

use anyhow::Result;
use itertools::Itertools;

const DAY: u8 = 14;

type Vec2 = (usize, usize);

fn calculate_load(platform: &[u8], dim: Vec2) -> usize {
    let mut total = 0;
    for y in 0..dim.0 {
        for x in 0..dim.1 {
            if platform[y * dim.1 + x] == b'O' {
                total += dim.0 - y;
            }
        }
    }
    total
}

fn do_tilt(platform: &mut [u8], dim: Vec2, get_coord: impl Fn(usize, usize) -> usize) {
    let mut buf = vec![0; dim.1];
    for y in 0..dim.0 {
        for x in 0..dim.1 {
            let b = &mut platform[get_coord(y, x)];
            if *b == b'O' {
                *b = b'.';
                platform[get_coord(buf[x], x)] = b'O';
                buf[x] += 1;
            } else if *b == b'#' {
                buf[x] = y + 1;
            }
        }
    }
}

fn part_1(input: &str) -> Result<usize> {
    let dim = (input.lines().next().unwrap().len(), input.lines().count());
    let mut platform = input.lines().flat_map(|line| line.bytes()).collect_vec();
    do_tilt(&mut platform, dim, |y, x| y * dim.1 + x);
    Ok(calculate_load(&platform, dim))
}

fn do_cycle(platform: &mut [u8], dim: Vec2) {
    let turned_dim = (dim.1, dim.0);
    do_tilt(platform, dim, |y, x| y * dim.1 + x);
    do_tilt(platform, turned_dim, |y, x| x * dim.0 + y);
    do_tilt(platform, dim, |y, x| (dim.0 - y - 1) * dim.1 + x);
    do_tilt(platform, turned_dim, |y, x| x * dim.0 + (dim.1 - y - 1));
}

fn part_2(input: &str) -> Result<usize> {
    let dim = (input.lines().next().unwrap().len(), input.lines().count());
    let mut platform = input.lines().flat_map(|line| line.bytes()).collect_vec();
    println!("{}", platform.len());
    let mut seen = BTreeMap::new();
    let mut done = 0;
    while seen.get(&platform).is_none() {
        // OUCH
        seen.insert(platform.clone(), done);
        do_cycle(&mut platform, dim);
        done += 1;
    }
    let cycle_len = done - seen[&platform];
    let rest = (1_000_000_000 - done) % cycle_len;
    for _ in 0..rest {
        do_cycle(&mut platform, dim);
    }
    Ok(calculate_load(&platform, dim))
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
        let res = "136";
        aoc::assert_output_matches_str(DAY, "example1", part_1, res)?;
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let res = "64";
        aoc::assert_output_matches_str(DAY, "example2", part_2, res)?;
        Ok(())
    }
}
