use anyhow::Result;
use itertools::Itertools;

const DAY: u8 = 13;

/// (y, x)
type Vec2 = (usize, usize);

fn find_pattern(
    block: &[u8],
    dim: Vec2,
    get_coord: impl Fn(Vec2) -> usize,
    wanted_diff: u32,
) -> Option<usize> {
    let diff = |first, second| {
        (0..dim.1)
            .map(|i| (block[get_coord((first, i))] != block[get_coord((second, i))]) as u32)
            .sum::<u32>()
    };
    let mut candidates = vec![];
    for first in 0..dim.0 - 1 {
        if diff(first, first + 1) <= wanted_diff {
            candidates.push(first);
        }
    }
    for candidate in candidates {
        let mut first = candidate;
        let mut second = candidate + 1;
        let mut d = 0;
        loop {
            d += diff(first, second);
            if d > wanted_diff {
                break;
            }
            if first == 0 || second == dim.0 - 1 {
                if d == wanted_diff {
                    return Some(candidate + 1);
                }
                break;
            }
            first -= 1;
            second += 1;
        }
    }
    None
}

fn part_1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .batching(|it| {
            let mut block = vec![];
            let mut height = 0;
            let mut width = 0;
            while let Some(line) = it.next() {
                if line.is_empty() {
                    break;
                }
                block.extend(line.bytes());
                width = line.len();
                height += 1;
            }
            if block.is_empty() {
                return None;
            }
            Some((block, (height, width)))
        })
        .map(|(block, dim)| {
            if let Some(horizontal) = find_pattern(&block, dim, |(ser, fnd)| ser * dim.1 + fnd, 0) {
                return horizontal * 100;
            }
            if let Some(vertical) =
                find_pattern(&block, (dim.1, dim.0), |(ser, fnd)| fnd * dim.1 + ser, 0)
            {
                return vertical;
            }
            unreachable!()
        })
        .sum())
}

fn part_2(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .batching(|it| {
            let mut block = vec![];
            let mut height = 0;
            let mut width = 0;
            while let Some(line) = it.next() {
                if line.is_empty() {
                    break;
                }
                block.extend(line.bytes());
                width = line.len();
                height += 1;
            }
            if block.is_empty() {
                return None;
            }
            Some((block, (height, width)))
        })
        .map(|(block, dim)| {
            if let Some(horizontal) = find_pattern(&block, dim, |(ser, fnd)| ser * dim.1 + fnd, 1) {
                return horizontal * 100;
            }
            if let Some(vertical) =
                find_pattern(&block, (dim.1, dim.0), |(ser, fnd)| fnd * dim.1 + ser, 1)
            {
                return vertical;
            }
            unreachable!()
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
        let res = "405";
        aoc::assert_output_matches_str(DAY, "example1", part_1, res)?;
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let res = "400";
        aoc::assert_output_matches_str(DAY, "example2", part_2, res)?;
        Ok(())
    }
}
