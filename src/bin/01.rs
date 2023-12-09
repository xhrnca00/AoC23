use anyhow::Result;

const DAY: u8 = 01;

fn part_1(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .map(|l| {
            let mut d = l.chars().filter_map(|c| c.to_digit(10));
            match (d.next(), d.next_back()) {
                (Some(a), Some(b)) => a * 10 + b,
                (Some(a), None) => a * 10 + a,
                _ => unreachable!("Invalid input"),
            }
        })
        .sum())
}

fn part_2(input: &str) -> Result<u32> {
    let lookup = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    Ok(input
        .lines()
        .map(|mut l| {
            let first;
            let last;
            let mut chars = l.chars().peekable();
            'o: loop {
                if let Some(d) = chars.peek().unwrap().to_digit(10) {
                    first = d;
                    break;
                }
                chars.next().unwrap();
                for (n, &pat) in lookup.iter().enumerate() {
                    if l.starts_with(pat) {
                        first = n as u32;
                        break 'o;
                    }
                }
                l = &l[1..];
            }
            'o: loop {
                if let Some(d) = chars.next_back().unwrap().to_digit(10) {
                    last = d;
                    break;
                }
                for (n, &pat) in lookup.iter().enumerate() {
                    if l.ends_with(pat) {
                        last = n as u32;
                        break 'o;
                    }
                }
                l = &l[..l.len() - 1];
            }
            first * 10 + last
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
        let res = "142";
        aoc::assert_output_matches_str(DAY, "example1", part_1, res)?;
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let res = "281";
        aoc::assert_output_matches_str(DAY, "example2", part_2, res)?;
        Ok(())
    }
}
