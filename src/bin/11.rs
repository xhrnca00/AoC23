use anyhow::Result;

const DAY: u8 = 11;

fn part_1(input: &str) -> Result<usize> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let mut galaxies = vec![];
    let mut empty_cols = vec![true; width];
    let mut empty_rows = vec![true; height];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                galaxies.push((y, x));
                empty_cols[x] = false;
                empty_rows[y] = false;
            }
        });
    });
    let mut cols_pref = vec![0; width + 1];
    let mut rows_pref = vec![0; height + 1];
    for i in 0..width {
        cols_pref[i + 1] = cols_pref[i] + empty_cols[i] as usize;
    }
    for i in 0..height {
        rows_pref[i + 1] = rows_pref[i] + empty_rows[i] as usize;
    }
    let mut sum = 0;
    for s in 0..galaxies.len() {
        let (sy, sx) = galaxies[s];
        for e in s + 1..galaxies.len() {
            let (ey, ex) = galaxies[e];
            sum += ex.abs_diff(sx)
                + (ey - sy)
                + (cols_pref[sx.max(ex) + 1] - cols_pref[sx.min(ex)])
                + (rows_pref[ey + 1] - rows_pref[sy]);
        }
    }
    Ok(sum)
}

fn part_2(input: &str) -> Result<usize> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let mut galaxies = vec![];
    let mut empty_cols = vec![true; width];
    let mut empty_rows = vec![true; height];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                galaxies.push((y, x));
                empty_cols[x] = false;
                empty_rows[y] = false;
            }
        });
    });
    let mut cols_pref = vec![0; width + 1];
    let mut rows_pref = vec![0; height + 1];
    for i in 0..width {
        cols_pref[i + 1] = cols_pref[i] + empty_cols[i] as usize;
    }
    for i in 0..height {
        rows_pref[i + 1] = rows_pref[i] + empty_rows[i] as usize;
    }
    let mut sum = 0;
    for s in 0..galaxies.len() {
        let (sy, sx) = galaxies[s];
        for e in s + 1..galaxies.len() {
            let (ey, ex) = galaxies[e];
            sum += ex.abs_diff(sx)
                + (ey - sy)
                + (cols_pref[sx.max(ex) + 1] - cols_pref[sx.min(ex)]) * 999_999
                + (rows_pref[ey + 1] - rows_pref[sy]) * 999_999;
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
        let res = "374";
        aoc::assert_output_matches_str(DAY, "example1", part_1, res)?;
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let res = "82000210";
        aoc::assert_output_matches_str(DAY, "example2", part_2, res)?;
        Ok(())
    }
}
