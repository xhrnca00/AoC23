use anyhow::Result;

const DAY: u8 = 14;

fn part_1(input: &str) -> Result<usize> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut total = 0;
    let mut buf = vec![0; width];
    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            if b == b'O' {
                total += height - buf[x];
                buf[x] += 1;
            } else if b == b'#' {
                buf[x] = y + 1;
            }
        }
    }
    Ok(total)
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
        let res = "136";
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
