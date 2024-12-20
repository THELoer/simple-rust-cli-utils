use std::path::PathBuf;
use std::process::exit;
use std::fs;
use std::fs::remove_file;
use clap::{Command, Arg, ArgMatches, command};

fn get_args() -> (Vec<String>, PathBuf) {
    let match_result = command!()
        .arg(
            Arg::new("path")
                .help("Path to file")
                .value_name("PATH")
                .required(true)
        )
        .arg(
            Arg::new("directory")
                .help("empty directory to remove")
                .short('d')
                .long("dir")
                .required(false)
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .required(false)
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("directory")
        )
        .get_matches();

    let flags = ["directory", "recursive"];

    let mut args: Vec<String> = Vec::new();
    for flag in flags {
        if match_result.get_flag(flag) {
            args.push(flag.to_string());
        }
    }
    let path = PathBuf::from(match_result.get_one::<String>("path").unwrap());
    if !path.exists() {
        println!("Path does not exist: \"{}\"", path.to_str().unwrap());
        exit(1);
    }
    (args, path)
}


fn flags(flags: Vec<String>, path: PathBuf)  {
    if flags.is_empty() {
        remove_file(path).unwrap();

    } else if flags.contains(&"directory".to_string()) {
        fs::remove_dir(path).unwrap();
    }
    else if flags.contains(&"recursive".to_string()) {
        recursive(path)
    }
}

fn recursive(paths: PathBuf) {

    fs::remove_dir_all(paths).unwrap();
}




fn main() {
    let (args, path) = get_args();
    flags(args, path);

}