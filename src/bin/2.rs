use anyhow::Result;
use itertools::Itertools;

const DAY: u8 = 02;

type Colors = [u8; 3];

// sorted
const COLOR_LABELS: [&str; 3] = ["blue", "green", "red"];

fn part_1(input: &String) -> Result<u32> {
    const MAX: Colors = [14, 13, 12];

    let mut sum = 0;
    for l in input.lines() {
        let (game, turns) = l.split(": ").collect_tuple().unwrap();
        let game_id: u32 = game.split(' ').next_back().unwrap().parse()?;
        let mut maxes: Colors = [0; 3];
        for turn in turns.split("; ") {
            for p in turn.split(", ") {
                let (count, color) = p.split(' ').collect_tuple().unwrap();
                let count = count.parse()?;
                let color_idx = COLOR_LABELS.binary_search(&color).unwrap();
                if maxes[color_idx] < count {
                    maxes[color_idx] = count;
                }
            }
        }
        if maxes.iter().enumerate().all(|(i, &c)| c <= MAX[i]) {
            sum += game_id;
        }
    }
    Ok(sum)
}

fn part_2(input: &String) -> Result<u32> {
    let mut sum = 0;
    for l in input.lines() {
        let turns = l.split(": ").last().unwrap();
        let mut maxes: Colors = [0; 3];
        for turn in turns.split("; ") {
            for p in turn.split(", ") {
                let (count, color) = p.split(' ').collect_tuple().unwrap();
                let count = count.parse()?;
                let color_idx = COLOR_LABELS.binary_search(&color).unwrap();
                if maxes[color_idx] < count {
                    maxes[color_idx] = count;
                }
            }
        }
        sum += maxes.iter().fold(1, |acc, n| *n as u32 * acc);
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
        let res = "2286";
        aoc::assert_output_matches_str(DAY, "example2", part_2, res)?;
        Ok(())
    }
}
