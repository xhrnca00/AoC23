use anyhow::Result;

const DAY: u8 = 04;

fn part_1(input: &String) -> Result<u32> {
    Ok(input.trim().parse()?)
}

fn part_2(input: &String) -> Result<u32> {
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
        let res = "13";
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
