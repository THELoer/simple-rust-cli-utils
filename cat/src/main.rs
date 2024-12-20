use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use clap::{command, Arg, ArgMatches};



fn get_args() -> PathBuf {
    let match_result = command!()
        .arg(
            Arg::new("path")
                .help("The path of the file to use")
                .value_name("FILE")
                .required(true)
        ).get_matches();


    let path = PathBuf::from(match_result.get_one::<String>("path").unwrap());
    if path.is_dir() {
        panic!("{} is directory", path.display());
    }
    if !path.exists() {
        panic!("{} does not exist", path.display());
    }
    path
}

fn main() {
    let mut f = File::open(get_args()).unwrap();
    let mut file_data = String::new();
    f.read_to_string(&mut file_data).unwrap();

    println!("{}", file_data);
}
