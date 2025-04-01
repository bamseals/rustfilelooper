use clap::{Arg, ArgAction, Command};
use std::{fs, path::Path};

fn main() {
    let matches = Command::new("rustfilelooper")
        .version("1.0")
        .about("Lists files or finds the longest common substring in filenames")
        .arg(
            Arg::new("nameloop")
                .long("nameloop")
                .short('n')
                .help("Finds the longest common substring in filenames")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let files: Vec<String> = fs::read_dir(".")
        .unwrap()
        .filter_map(|entry| entry.ok().map(|e| e.file_name().into_string().ok()).flatten())
        .collect();

    if matches.get_flag("nameloop") {
        let filenames_without_ext: Vec<String> = files
            .iter()
            .map(|name| remove_extension(name))
            .collect();
        if let Some(common_str) = longest_common_substring(&filenames_without_ext) {
            println!("Longest common substring: {}", common_str);
        } else {
            println!("No common substring found.");
        }
    } else {
        for file in &files {
            println!("{}", file);
        }
    }
}

// Function to remove the file extension
fn remove_extension(filename: &str) -> String {
    Path::new(filename)
        .file_stem()  // Gets the filename without extension
        .and_then(|stem| stem.to_str())
        .unwrap_or(filename)
        .to_string()
}

// Finds the longest common substring in a list of filenames
fn longest_common_substring(filenames: &[String]) -> Option<String> {
    if filenames.is_empty() {
        return None;
    }
    let first = &filenames[0];

    for len in (1..=first.len()).rev() {
        for start in 0..=first.len() - len {
            let substr = &first[start..start + len];
            if filenames.iter().all(|name| name.contains(substr)) {
                return Some(substr.to_string());
            }
        }
    }
    None
}