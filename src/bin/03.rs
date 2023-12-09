use std::ops::Range;

use anyhow::Result;
use itertools::{any, izip, Itertools};

const DAY: u8 = 03;

fn part_1(input: &str) -> Result<u32> {
    let line_len = input.lines().next().unwrap().len();
    let blank_line = ".".repeat(line_len);
    // Add blank lines to the top and bottom of the input
    let lines_it = blank_line
        .lines()
        .chain(input.lines())
        .chain(blank_line.lines());
    Ok(lines_it
        .tuple_windows()
        .map(|(top, mid, bot)| {
            let mut sum = 0;
            let mut is_valid = false;
            let mut num = 0;
            for (t, m, b) in izip!(top.chars(), mid.chars(), bot.chars()) {
                let digit = m.to_digit(10);
                let this_valid = any([t, m, b], |c| !c.is_digit(10) && c != '.');
                is_valid = is_valid || this_valid;
                if let Some(d) = digit {
                    num = num * 10 + d;
                    continue;
                }
                if is_valid && num != 0 {
                    sum += num;
                }
                num = 0;
                is_valid = this_valid;
            }
            if is_valid {
                sum += num;
            }
            sum
        })
        .sum())
}

/// loads digit to the left from BUT NOT INCLUDING i
fn read_digit_l(table: &[u8], mut i: usize, w: usize) -> Range<usize> {
    let end = i;
    while i % w != 0 && table[i - 1].is_ascii_digit() {
        i -= 1;
    }
    i..end
}

/// loads digit to the right and INCLUDING i
fn read_digit_mr(table: &[u8], mut i: usize, w: usize) -> Range<usize> {
    if i % w == 0 || !table[i].is_ascii_digit() {
        return i..i;
    }
    let start = i;
    i += 1;
    while i % w != 0 && table[i].is_ascii_digit() {
        i += 1;
    }
    start..i
}

fn part_2(input: &str) -> Result<u32> {
    let mut sum = 0;
    let w = input.lines().next().unwrap().len();
    let h = input.lines().count();
    let table = input
        .lines()
        .flat_map(|l| l.as_bytes().to_owned())
        .collect_vec();
    for gear in (0..table.len()).filter(|&i| table[i] == b'*') {
        let y = gear / w;
        let mut nums = Vec::new();
        // middle left
        {
            let l = read_digit_l(&table, gear, w);
            if !l.is_empty() {
                nums.push(l);
            }
        }
        // middle right
        {
            let mr = read_digit_mr(&table, gear + 1, w);
            if !mr.is_empty() {
                nums.push(mr);
            }
        }
        // bottom
        if y != h - 1 {
            let l = read_digit_l(&table, gear + w, w);
            let mid = read_digit_mr(&table, gear + w, w);
            if !mid.is_empty() {
                nums.push(l.start..mid.end);
            } else {
                if !l.is_empty() {
                    nums.push(l);
                }
                let r = read_digit_mr(&table, gear + w + 1, w);
                if !r.is_empty() {
                    nums.push(r);
                }
            }
        }
        // top
        if y != 0 {
            let l = read_digit_l(&table, gear - w, w);
            let mid = read_digit_mr(&table, gear - w, w);
            if !mid.is_empty() {
                nums.push(l.start..mid.end);
            } else {
                if !l.is_empty() {
                    nums.push(l);
                }
                let r = read_digit_mr(&table, gear - w + 1, w);
                if !r.is_empty() {
                    nums.push(r);
                }
            }
        }

        if nums.len() == 2 {
            unsafe {
                let num1 = String::from_utf8_unchecked(table[nums[0].to_owned()].to_owned())
                    .parse::<u32>()?;
                let num2 = String::from_utf8_unchecked(table[nums[1].to_owned()].to_owned())
                    .parse::<u32>()?;
                sum += num1 * num2;
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
        let res = "4361";
        aoc::assert_output_matches_str(DAY, "example1", part_1, res)?;
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let res = "467835";
        aoc::assert_output_matches_str(DAY, "example2", part_2, res)?;
        Ok(())
    }
}
