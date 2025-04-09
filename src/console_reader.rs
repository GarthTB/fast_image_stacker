use std::path::PathBuf;
use std::{env, io};

pub(crate) fn read_line() -> String {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => return input.trim().to_string(),
            Err(message) => println!("Error: {message}\nPlease try again."),
        }
        input.clear();
    }
}

fn gen_result_path(dir: &PathBuf) -> PathBuf {
    let mut result_name = "stacked_result.tif".to_string();
    let mut i: usize = 2;
    while dir.join(&result_name).exists() {
        result_name = format!("stacked_result_{i}.tif");
        i += 1;
    }
    dir.join(result_name)
}

fn collect_paths(dir: &PathBuf) -> Result<(Vec<PathBuf>, PathBuf), &'static str> {
    if !dir.exists() {
        Err("Directory does not exist.")
    } else if !dir.is_dir() {
        Err("Path is not a directory.")
    } else {
        let dir_entries = dir
            .read_dir()
            .map_err(|_| "Could not read directory contents.")?;
        let paths: Vec<PathBuf> = dir_entries
            .filter_map(|result| result.ok().map(|entry| entry.path()))
            .collect();
        let result_path = gen_result_path(dir);
        println!(
            "Found {} images in directory {}",
            paths.len(),
            dir.display(),
        );
        Ok((paths, result_path))
    }
}

pub(crate) fn get_paths() -> Result<(Vec<PathBuf>, PathBuf), &'static str> {
    let exe_path = env::current_exe().map_err(|_| "Could not get executable path.")?;
    let dir = exe_path.parent().ok_or("Could not get parent directory.")?;
    let mut image_dir = dir.join("images");
    if let Ok((image_paths, result_path)) = collect_paths(&image_dir) {
        return Ok((image_paths, result_path));
    }
    println!("Could not find images in default directory. Please enter a directory path:");
    loop {
        image_dir = PathBuf::from(read_line());
        match collect_paths(&image_dir) {
            Ok((image_paths, result_path)) => return Ok((image_paths, result_path)),
            Err(message) => println!("{message}\nPlease try again."),
        }
    }
}

pub(crate) fn get_mode() -> u8 {
    println!("Please select a mode:");
    println!("0. mean");
    println!("1. max");
    println!("2. min");
    loop {
        if let Ok(code) = read_line().parse() {
            if code < 3 {
                return code;
            }
        }
        println!("Invalid input. Please try again.")
    }
}
