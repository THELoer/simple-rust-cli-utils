use clap::{command, Arg, ArgMatches};
use std::fs;
use std::fs::metadata;
use std::os::unix::fs::MetadataExt;
use std::time::{UNIX_EPOCH, SystemTime};
use std::path::{Path, PathBuf};
use chrono;

fn get_args() -> (Vec<String>, PathBuf) {
    let match_result = command!()
        .about("gives info about files")
        .arg(
            Arg::new("path")
                .help("path to file")
                .value_name("PATH")
                .default_value("."),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .required(false)
                .help("Show all files")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("recursive")
                .short('R')
                .long("recursive")
                .required(false)
                .help("Recursively search files")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("long")
                .short('l')
                .long("long")
                .required(false)
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let flags = ["all", "recursive", "long"];

    let mut args: Vec<String> = Vec::new();
    for flag in flags {
        if match_result.get_flag(flag) {
            args.push(flag.to_string());
        }
    }
    let path = PathBuf::from(match_result.get_one::<String>("path").unwrap());

    (args, path)
}

fn flag_searcher(flags: Vec<String>, path: PathBuf) -> Vec<String> {
    let mut files = Vec::new();
    let mut files2: Vec<String> = Vec::new();
    if flags.contains(&"all".to_string()) {
        files = all_out(path);
    } else {
        files = simple_out(path);
    }
    files.sort();
    if flags.contains(&"recursive".to_string()) {
        files2 = recursive(files.clone(), 2).1;
        files = recursive(files, 2).0;

    }


    if flags.contains(&"long".to_string()) {
        files = long(files2);
    } else {
        files = files;
    }

    println!("{:?}", files);
    return files;
}



fn recursive(paths: Vec<String>, indent: usize) -> (Vec<String>, Vec<String>) {
    let mut result = Vec::new();
    let mut result_o: Vec<String> = Vec::new();

    for path_str in paths {
        let path = Path::new(&path_str);

        if path.is_dir() {
            result_o.push(format!("{}", &path.display()));
            result.push(format!("{:indent$}{}", "", &path.display(), indent = indent)); // something happend and it's not working

            for entry in fs::read_dir(path).unwrap() {
                let entry = entry.unwrap();
                let entry_path = entry.path();

                if entry_path.is_dir() {
                    let mut subdirs=
                        recursive(vec![entry_path.to_string_lossy().to_string()], indent + 4).1;

                    result_o.append(&mut subdirs.clone());
                    result.append(&mut subdirs);
                }
            }
        } else {
            result.push(format!("{:indent$}{}", "", &path.display(), indent = indent));
            result_o.push(format!("{}", &path.display()));
        }
    }
    //println!("{:?}", result_o);
    (result, result_o)
}
fn simple_out(path: PathBuf) -> Vec<String> {
    let mut ans_vec = Vec::new();

    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid entry: {:?}", f)));
            if file_name.clone().unwrap().chars().nth(0).unwrap() != '.' {
                ans_vec.push(file_name.unwrap());
            }
        }
    }
    ans_vec
}

fn all_out(path: PathBuf) -> Vec<String> {
    let mut ans_vec: Vec<String> = Vec::new();
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid entry: {:?}", f)));
            ans_vec.push(file_name.unwrap());
        }
    }
    ans_vec
}


fn long(files: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    for paths in files {
        let path = Path::new(&paths);

        let metadata = fs::metadata(path);
        match metadata {
            Ok(metadata) => {
                ()
            }
            Err(_) => {
                println!("Cant get metadata {}", path.display());
                continue
            }
        }
        let metadata = fs::metadata(path).unwrap();

        let file_type = if metadata.is_dir() { "d" } else { "-" };

        let mode = metadata.mode();
        let permissions = format!(
            "{}{}{}{}{}{}{}{}{}",
            if mode & 0o400 != 0 { "r" } else { "-" },
            if mode & 0o200 != 0 { "w" } else { "-" },
            if mode & 0o100 != 0 { "x" } else { "-" },
            if mode & 0o040 != 0 { "r" } else { "-" },
            if mode & 0o020 != 0 { "w" } else { "-" },
            if mode & 0o010 != 0 { "x" } else { "-" },
            if mode & 0o004 != 0 { "r" } else { "-" },
            if mode & 0o002 != 0 { "w" } else { "-" },
            if mode & 0o001 != 0 { "x" } else { "-" }
        );

        let hard_links = metadata.nlink();
        let owner = metadata.uid();
        let group = metadata.gid();

        let file_size = metadata.len();
        let modified_time = metadata.modified().unwrap().duration_since(UNIX_EPOCH).unwrap();
        let datetime = SystemTime::UNIX_EPOCH + modified_time;
        let datetime = chrono::DateTime::<chrono::Local>::from(datetime);
        let time_str = datetime.format("%b %d %H:%M").to_string();

        result.push(format!("{}{} {:>3} {:>5} {:>5} {:>8} {} {} ",
        file_type, permissions, hard_links, owner, group, file_size, time_str, path.display()))
    }

    result
}

fn main() {
    let (flags, path) = get_args();
    if flags.contains(&"recursive".to_string()) && !flags.contains(&"long".to_string()) {
        for file in flag_searcher(flags, path) {
            println!("{}", file);
        }
    }
    else if flags.contains(&"long".to_string()) {
        for file in flag_searcher(flags, path) {
            println!("{}", file);
        }
    } else {
        println!("{}", flag_searcher(flags, path).join(" "));
    }
}
