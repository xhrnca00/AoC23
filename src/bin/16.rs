use std::{fmt::Debug, mem};

use anyhow::Result;

const DAY: u8 = 16;

/// y, x
type Vec2 = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Part {
    Blank = 0b00,
    RLMirror = 0b10,
    LRMirror = 0b11,
    HPipe = 0b100,
    VPipe = 0b110,
}

impl Part {
    fn is_pipe(&self) -> bool {
        return unsafe { mem::transmute(*self as u8 >> 2) };
        // matches!(self, Self::HPipe | Self::VPipe)
    }
}

fn parse_contraption(input: &str) -> Vec<Part> {
    input
        .lines()
        .flat_map(|l| l.bytes())
        .map(|b| match b {
            b'/' => Part::LRMirror,
            b'\\' => Part::RLMirror,
            b'.' => Part::Blank,
            b'-' => Part::HPipe,
            b'|' => Part::VPipe,
            _ => unreachable!(),
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum LightDir {
    Left = 0b00,
    Right = 0b01,
    Up = 0b10,
    Down = 0b11,
}

impl LightDir {
    fn perp(self) -> (Self, Self) {
        // // this one has the same amount of instructions -> we trust the compiler
        // unsafe {
        //     return mem::transmute((self as u8 ^ 0b10, self as u8 ^ 0b11));
        // }
        use LightDir::*;
        match self {
            Left | Right => (Up, Down),
            Up | Down => (Left, Right),
        }
    }
    fn inv(self) -> Self {
        return unsafe { mem::transmute(self as u8 ^ 0b01) };
        // use LightDir::*;
        // match self {
        //     Left => Right,
        //     Right => Left,
        //     Up => Down,
        //     Down => Up,
        // }
    }
}

fn is_pipe_par(pipe: Part, dir: LightDir) -> bool {
    return dir as u8 & 0b10 == pipe as u8 & 0b10;
    // use LightDir::*;
    // use Part::*;
    // match pipe {
    //     HPipe => matches!(dir, Left | Right),
    //     VPipe => matches!(dir, Up | Down),
    //     _ => unreachable!(),
    // }
}

fn reflect(dir: LightDir, part: Part) -> LightDir {
    return unsafe { mem::transmute(dir as u8 ^ part as u8) };
    // use LightDir::*;
    // match part {
    //     Part::LRMirror => match dir {
    //         Left => Down,
    //         Right => Up,
    //         Up => Right,
    //         Down => Left,
    //     },
    //     Part::RLMirror => match dir {
    //         Left => Up,
    //         Right => Down,
    //         Up => Left,
    //         Down => Right,
    //     },
    //     Part::Blank => dir,
    //     _ => unreachable!(),
    // }
}

fn get_next(dir: LightDir, dim: Vec2, curr: Vec2) -> Option<Vec2> {
    use LightDir::*;
    match dir {
        Left if curr.1 > 0 => Some((curr.0, curr.1 - 1)),
        Right if curr.1 < dim.1 - 1 => Some((curr.0, curr.1 + 1)),
        Up if curr.0 > 0 => Some((curr.0 - 1, curr.1)),
        Down if curr.0 < dim.0 - 1 => Some((curr.0 + 1, curr.1)),
        _ => None,
    }
}

fn get_idx(dim: Vec2, pos: Vec2) -> usize {
    pos.0 * dim.1 + pos.1
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct LitData {
    _data: u8,
}

impl LitData {
    const FULLY_LIT: u8 = 0b1111;

    fn new() -> Self {
        Self { _data: 0 }
    }
    fn set(&mut self, dir: LightDir) {
        self._data |= 1 << dir as u8;
    }
    fn flit(&mut self) {
        self._data = Self::FULLY_LIT;
    }
    fn get(&self, dir: LightDir) -> bool {
        self._data & (1 << dir as u8) != 0
    }
    fn is_unlit(&self) -> bool {
        self._data == 0
    }
}

impl Debug for LitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.get(LightDir::Left) {
            write!(f, "L")?;
        } else {
            write!(f, ".")?;
        }
        if self.get(LightDir::Right) {
            write!(f, "R")?;
        } else {
            write!(f, ".")?;
        }
        if self.get(LightDir::Up) {
            write!(f, "T")?;
        } else {
            write!(f, ".")?;
        }
        if self.get(LightDir::Down) {
            write!(f, "B")?;
        } else {
            write!(f, ".")?;
        }
        Ok(())
    }
}

fn go_through(contraption: &[Part], dim: Vec2, start: Vec2, dir: LightDir) -> Vec<LitData> {
    let mut do_next = vec![(start, dir)];
    let mut ldata = vec![LitData::new(); contraption.len()];
    while let Some((mut pos, mut dir)) = do_next.pop() {
        loop {
            let idx = get_idx(dim, pos);
            let mut part = contraption[idx];
            if part.is_pipe() && is_pipe_par(part, dir) {
                part = Part::Blank;
            }
            if part.is_pipe() {
                ldata[idx].flit();
                let (p1, p2) = dir.perp();
                do_next.push((pos, p1));
                dir = p2;
            } else {
                let next_dir = reflect(dir, part);
                ldata[idx].set(dir);
                ldata[idx].set(next_dir.inv());
                dir = next_dir;
            }
            if let Some(next_pos) = get_next(dir, dim, pos) {
                if ldata[get_idx(dim, next_pos)].get(dir) {
                    break;
                }
                pos = next_pos;
            } else {
                break;
            }
        }
    }
    ldata
}

fn get_energized(ldata: &[LitData]) -> usize {
    ldata.iter().filter(|ld| !ld.is_unlit()).count()
}

fn part_1(input: &str) -> Result<usize> {
    let dim = (input.lines().count(), input.lines().next().unwrap().len());
    let contraption = parse_contraption(input);
    let ldata = go_through(&contraption, dim, (0, 0), LightDir::Right);
    Ok(get_energized(&ldata))
}

fn part_2(input: &str) -> Result<usize> {
    let dim = (input.lines().count(), input.lines().next().unwrap().len());
    let contraption = parse_contraption(input);
    let mut max = 0;
    for x in 0..dim.1 {
        let mut ldata = go_through(&contraption, dim, (0, x), LightDir::Down);
        max = max.max(get_energized(&ldata));
        ldata = go_through(&contraption, dim, (dim.1 - 1, x), LightDir::Up);
        max = max.max(get_energized(&ldata));
    }
    for y in 0..dim.0 {
        let mut ldata = go_through(&contraption, dim, (y, 0), LightDir::Right);
        max = max.max(get_energized(&ldata));
        ldata = go_through(&contraption, dim, (y, dim.0 - 1), LightDir::Left);
        max = max.max(get_energized(&ldata));
    }
    Ok(max)
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
        let res = "46";
        aoc::assert_output_matches_str(DAY, "example1", part_1, res)?;
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let res = "51";
        aoc::assert_output_matches_str(DAY, "example2", part_2, res)?;
        Ok(())
    }
}
