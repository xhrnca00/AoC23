use std::{
    fmt::Display,
    fs::File,
    io::{BufWriter, Read, Write},
    path::PathBuf,
};

use anyhow::Result;
use project_root::get_project_root;

pub fn solve_all<T1: Display, T2: Display>(
    task_num: u8,
    part_1: fn(&String) -> Result<T1>,
    part_2: fn(&String) -> Result<T2>,
) -> Result<()> {
    let start = std::time::Instant::now();
    let output_file = File::create(get_output_path(task_num, None)?)?;
    let mut writer = BufWriter::new(output_file);
    let mut input_file = File::open(get_input_path(task_num, None)?)?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    let res1 = part_1(&input)?;
    write!(writer, "{res1}\n")?;
    eprintln!("Part 1: {res1}");
    let res2 = part_2(&input)?;
    write!(writer, "{res2}\n")?;
    eprintln!("Part 2: {res2}");
    writer.flush()?;
    eprintln!("Finished in {:?}!", start.elapsed());
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

pub fn get_output_path(task_num: u8, suffix: Option<&str>) -> Result<PathBuf> {
    let mut path = get_project_root()?;
    path.push("data");
    let mut filename = format!("{:02}", task_num);
    if let Some(suffix) = suffix {
        filename.push('_');
        filename.push_str(suffix);
    }
    filename.push_str(".out");
    path.push(filename);
    Ok(path)
}

pub fn assert_output_matches_str<T: Display>(
    task_num: u8,
    file_suffix: &str,
    solve: fn(&String) -> Result<T>,
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
