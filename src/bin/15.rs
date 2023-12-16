use std::collections::BTreeMap;

use anyhow::Result;

const DAY: u8 = 15;

fn hash(part: &str) -> usize {
    part.bytes().fold(0, |mut acc, b| {
        acc += b as usize;
        acc *= 17;
        acc % 256
    })
}

fn part_1(input: &str) -> Result<usize> {
    Ok(input.lines().flat_map(|l| l.split(',')).map(hash).sum())
}

#[derive(Debug)]
enum CmdType {
    Remove,
    Add(usize),
}

fn parse_cmd(s: &str) -> (&str, CmdType) {
    if s.ends_with('-') {
        (&s[..s.len() - 1], CmdType::Remove)
    } else {
        let (label, num) = s.split_once('=').unwrap();
        (label, CmdType::Add(num.parse().unwrap()))
    }
}

fn part_2(input: &str) -> Result<usize> {
    const EMPTY: BTreeMap<usize, &str> = BTreeMap::new();
    let mut boxes = [EMPTY; 256];
    let mut label_to_data = BTreeMap::new();
    for (i, part) in input.lines().flat_map(|l| l.split(',')).enumerate() {
        let (label, cmd) = parse_cmd(part);
        let h = hash(label);
        match cmd {
            CmdType::Remove => {
                if let Some((order, _)) = label_to_data.remove(label) {
                    boxes[h].remove(&order);
                }
            }
            CmdType::Add(flen) => {
                if let Some((_, old_flen)) = label_to_data.get_mut(label) {
                    *old_flen = flen;
                } else {
                    label_to_data.insert(label, (i, flen));
                    boxes[h].insert(i, label);
                }
            }
        }
    }
    let mut total = 0;
    for (i, b) in boxes.into_iter().enumerate() {
        for (order, &part) in b.values().enumerate() {
            let flen = label_to_data[part].1;
            total += (i + 1) * (order + 1) * flen;
        }
    }
    Ok(total)
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
