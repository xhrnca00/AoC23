use anyhow::Result;

const DAY: u8 = 15;

fn part_1(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|part| {
                    part.bytes().fold(0, |mut acc, b| {
                        acc += b as u32;
                        acc *= 17;
                        acc % 256
                    })
                })
                .sum::<u32>()
        })
        .sum())
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
        let res = "1320";
        aoc::assert_output_matches_str(DAY, "example1", part_1, res)?;
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let res = "145";
        aoc::assert_output_matches_str(DAY, "example2", part_2, res)?;
        Ok(())
    }
}
