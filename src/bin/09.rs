use anyhow::Result;
use itertools::Itertools;

const DAY: u8 = 09;

fn get_differences(seq: &[i64]) -> Vec<i64> {
    seq.iter().tuple_windows().map(|(a, b)| b - a).collect()
}

fn part_1(input: &str) -> Result<i64> {
    Ok(input
        .lines()
        .map(|l| {
            l.split(' ')
                .map(str::parse)
                .map(Result::unwrap)
                .collect_vec()
        })
        .map(|s| {
            // ENHANCE: O(N) memory
            let mut difs = vec![s];
            difs.push(get_differences(difs.last().unwrap()));
            while !difs.last().unwrap().iter().all_equal() {
                difs.push(get_differences(difs.last().unwrap()));
            }
            difs.into_iter().map(|mut v| v.pop().unwrap()).sum::<i64>()
        })
        .sum())
}

fn part_2(input: &str) -> Result<i64> {
    Ok(input
        .lines()
        .map(|l| {
            l.split(' ')
                .map(str::parse)
                .map(Result::unwrap)
                .collect_vec()
        })
        .map(|s| {
            let mut difs = vec![s];
            difs.push(get_differences(difs.last().unwrap()));
            while !difs.last().unwrap().iter().all_equal() {
                difs.push(get_differences(difs.last().unwrap()));
            }
            difs.into_iter()
                .map(|v| v.first().unwrap().clone())
                .rev()
                .fold(0, |acc, x| x - acc)
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
        let res = "114";
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
