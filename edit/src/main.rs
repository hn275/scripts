use std::fs;
use std::process::Command;
use std::{env, io::Error};

fn main() {
    let home_dir = env::var("HOME").unwrap();
    let work_place: Vec<String> = get_paths().unwrap_or_else(|e| {
        panic!("failed to get path: {}", e);
    });

    let mut fzf_dirs: Vec<String> = vec![];

    for wp in work_place {
        let output = Command::new("ls")
            .arg(format!("{}/{}", &home_dir, wp))
            .output()
            .expect("failed ls");

        let dirs = String::from_utf8(output.stdout).unwrap();
        let dirs = dirs.split('\n').collect::<Vec<&str>>();

        for dir in dirs {
            if dir == "" {
                continue;
            }

            let file = format!("{}/{}", wp, dir);
            fzf_dirs.push(file);
        }
    }

    let wps = fzf_dirs.join("\n");
    println!("{wps}");
}

fn get_paths() -> Result<Vec<String>, Error> {
    let home_dir = env::var("HOME").unwrap();
    let file_path = format!("{}/_scripts/edit/locations.txt", home_dir);

    let file = fs::read_to_string(file_path)
        .map_err(|e| e)?
        .trim()
        .split("\n")
        .map(|el| {
            return el.to_owned();
        })
        .collect::<Vec<String>>();

    return Ok(file);
}
