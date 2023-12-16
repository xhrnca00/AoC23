use std::{fmt::Display, fs::File, io::Read, path::PathBuf};

use anyhow::Result;
use project_root::get_project_root;

pub fn solve_all<T1: Display, T2: Display>(
    task_num: u8,
    part_1: fn(&str) -> Result<T1>,
    part_2: fn(&str) -> Result<T2>,
) -> Result<()> {
    let mut input_file = File::open(get_input_path(task_num, None)?)?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    let start = std::time::Instant::now();
    let res1 = part_1(&input)?;
    let res2 = part_2(&input)?;
    let elapsed = start.elapsed();
    eprintln!("Part 1: {res1}");
    eprintln!("Part 2: {res2}");
    eprintln!("Finished in {:?}!", elapsed);
    Ok(())
}

pub fn get_input_path(task_num: u8, suffix: Option<&str>) -> Result<PathBuf> {
    let mut path = get_project_root()?;
    path.push("data");
    let mut filename = format!("{:02}", task_num);
    if let Some(suffix) = suffix {
        filename.push('_');
        filename.push_str(suffix);
    }
    filename.push_str(".in");
    path.push(filename);
    Ok(path)
}

pub fn assert_output_matches_str<T: Display>(
    task_num: u8,
    file_suffix: &str,
    solve: fn(&str) -> Result<T>,
    output: &str,
) -> Result<()> {
    let mut input_file = File::open(get_input_path(task_num, Some(file_suffix))?)?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    let res = solve(&input)?;
    let solve_output = format!("{res}");
    assert_eq!(solve_output, output);
    Ok(())
}
