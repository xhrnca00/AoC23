use std::{cmp::Ordering, collections::BTreeMap, ops::RangeInclusive, str::FromStr};

use anyhow::Result;
use itertools::Itertools;

const DAY: u8 = 19;

const MAX_VAL: u32 = 4000;

type Part = [u32; 4];
type PartRange = [RangeInclusive<u32>; 4];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

impl FromStr for Category {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::ExtremelyCoolLooking),
            "m" => Ok(Self::Musical),
            "a" => Ok(Self::Aerodynamic),
            "s" => Ok(Self::Shiny),
            _ => anyhow::bail!("Invalid category: {}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule<'a> {
    what: Category,
    ord: Ordering,
    num: u32,
    then: &'a str,
}

impl<'a> Rule<'a> {
    const ALWAYS_TRUE: Self = Self {
        what: Category::ExtremelyCoolLooking,
        ord: Ordering::Greater,
        num: 0,
        then: "CHANGE THIS",
    };

    fn from_str(s: &'a str) -> Result<Self> {
        if let Some((s, then)) = s.split_once(':') {
            let (what, s) = s.split_at(1);
            let (ord, s) = s.split_at(1);
            let ord = match ord {
                "<" => Ordering::Less,
                ">" => Ordering::Greater,
                _ => anyhow::bail!("Invalid ordering: {}", ord),
            };
            Ok(Self {
                what: what.parse()?,
                ord,
                num: s.parse()?,
                then,
            })
        } else {
            Ok(Self {
                then: s,
                ..Self::ALWAYS_TRUE
            })
        }
    }

    fn matches(&self, part: &Part) -> bool {
        part[self.what as usize].cmp(&self.num) == self.ord
    }

    /// returns ranges that are (accepted, rejected) for this rule
    fn ranges(&self) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
        match self.ord {
            Ordering::Less => (1..=self.num - 1, self.num..=MAX_VAL),
            Ordering::Greater => (self.num + 1..=MAX_VAL, 1..=self.num),
            _ => unreachable!(),
        }
    }

    fn split_range(&self, pr: &PartRange) -> (Option<PartRange>, Option<PartRange>) {
        let idx = self.what as usize;
        let (accepted, rejected) = self.ranges();
        let res = (
            intersect_range(&accepted, &pr[idx]).map(|r| {
                let mut res = pr.clone();
                res[idx] = r;
                res
            }),
            intersect_range(&rejected, &pr[idx]).map(|r| {
                let mut res = pr.clone();
                res[idx] = r;
                res
            }),
        );
        res
    }
}

fn intersect_range(
    r1: &RangeInclusive<u32>,
    r2: &RangeInclusive<u32>,
) -> Option<RangeInclusive<u32>> {
    let start = r1.start().max(r2.start());
    let end = r1.end().min(r2.end());
    if start <= end {
        Some(*start..=*end)
    } else {
        None
    }
}

fn part_1(input: &str) -> Result<u32> {
    let mut lines = input.lines();
    let mut workflows = BTreeMap::new();
    let mut next = lines.next().unwrap();
    while !next.is_empty() {
        let (name, rules) = next[..next.len() - 1].split_once('{').unwrap();
        workflows.insert(
            name,
            rules
                .split(',')
                .map(|r| Rule::from_str(r).unwrap())
                .collect_vec(),
        );
        next = lines.next().unwrap();
    }
    Ok(lines
        .map(|l| {
            let mut part = [0; 4];
            for (idx, val) in l[1..l.len() - 1].split(',').enumerate() {
                part[idx] = val[2..].parse().unwrap();
            }
            part
        })
        .map(|part| {
            let mut curr = "in";
            while curr != "R" && curr != "A" {
                curr = workflows[curr]
                    .iter()
                    .find(|r| r.matches(&part))
                    .unwrap()
                    .then;
            }
            if curr == "A" {
                part.into_iter().sum()
            } else {
                0
            }
        })
        .sum())
}

fn get_val(pr: PartRange) -> u64 {
    pr.into_iter().map(|r| r.count() as u64).product()
}

fn part_2(input: &str) -> Result<u64> {
    const VAL_RANGE: RangeInclusive<u32> = 1..=MAX_VAL;
    let mut lines = input.lines();
    let mut workflows = BTreeMap::new();
    let mut next = lines.next().unwrap();
    while !next.is_empty() {
        let (name, rules) = next[..next.len() - 1].split_once('{').unwrap();
        workflows.insert(
            name,
            rules
                .split(',')
                .map(|r| Rule::from_str(r).unwrap())
                .collect_vec(),
        );
        next = lines.next().unwrap();
    }
    let mut stack = vec![("in", [VAL_RANGE; 4])];
    let mut sum = 0;
    while let Some((start, mut ranges)) = stack.pop() {
        for w in &workflows[start] {
            let (accepted, rejected) = w.split_range(&ranges);
            if let Some(a) = accepted {
                if w.then == "A" {
                    sum += get_val(a);
                } else if w.then != "R" {
                    stack.push((w.then, a));
                }
            }
            if let Some(r) = rejected {
                ranges = r;
            } else {
                break;
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
        let res = "19114";
        aoc::assert_output_matches_str(DAY, "example1", part_1, res)?;
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let res = "167409079868000";
        aoc::assert_output_matches_str(DAY, "example2", part_2, res)?;
        Ok(())
    }
}
