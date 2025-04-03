use clap::{Arg, ArgAction, Command, ArgMatches};
use std::{fs, path::Path};

fn main() {
    let matches: ArgMatches = get_cli_args();

    let files: Vec<String> = fs::read_dir(".")
        .unwrap()
        .filter_map(|entry| {
            entry
                .ok()
                .map(|e| e.file_name().into_string().ok())
                .flatten()
        })
        .collect();

    let nameloop_flag: bool = matches.get_flag("nameloop");
    let remove_arg: Option<&String> = matches.get_one::<String>("remove");

    if nameloop_flag {
        let filenames_without_ext: Vec<String> =
            files.iter().map(|name| remove_extension(name)).collect();

        if let Some(common_str) = longest_common_substring(&filenames_without_ext) {
            println!("Longest common substring: {}", common_str);

            if remove_arg.is_some() && remove_arg.unwrap().is_empty() {
                // If --remove is used without a value and --nameloop is present, use the longest common substring
                remove_string_from_filenames(&files, &common_str);
            }
        } else {
            println!("No common substring found.");
        }
    }

    if let Some(remove_str) = remove_arg {
        if !nameloop_flag && !remove_str.is_empty() {
            remove_string_from_filenames(&files, remove_str);
        }
    }

    if !nameloop_flag && remove_arg.is_none() {
        for file in &files {
            println!("{}", file);
        }
    }
}

// Handle command line arguments
fn get_cli_args() -> ArgMatches {
    return Command::new("rustfilelooper")
        .version("1.0")
        .about("Lists files, finds the longest common substring, or removes a substring from filenames")
        .arg(
            Arg::new("nameloop")
                .long("nameloop")
                .short('n')
                .help("Finds the longest common substring in filenames")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("remove")
                .long("remove")
                .short('r')
                .help("Removes a specified substring from filenames; if used with --nameloop, removes the longest common substring")
                .num_args(0..=1) // Accepts zero or one argument
                .require_equals(true) // Allows `--remove=` syntax
                .default_missing_value(""), // If used without a value, gets an empty string
        )
        .get_matches();
}

// Remove file extension from filename
fn remove_extension(filename: &str) -> String {
    Path::new(filename)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or(filename)
        .to_string()
}

// Finds the longest common substring in a list of filenames
fn longest_common_substring(filenames: &[String]) -> Option<String> {
    if filenames.is_empty() {
        return None;
    }
    let first: &String = &filenames[0];

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

// Remove a specified string from all filenames if possible
fn remove_string_from_filenames(files: &[String], remove_str: &str) {
    for file in files {
        if file.contains(remove_str) {
            let new_name = file.replace(remove_str, "");
            if let Err(e) = fs::rename(file, &new_name) {
                eprintln!("Error renaming {}: {}", file, e);
            } else {
                println!("Renamed {} -> {}", file, new_name);
            }
        }
    }
}
