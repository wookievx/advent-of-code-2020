use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let pref = Path::new(env!("CARGO_MANIFEST_DIR"));
    let file = File::open(pref.join(filename)).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn resource_contents(filename: impl AsRef<Path>) -> String {
    let pref = Path::new(env!("CARGO_MANIFEST_DIR"));
    let file = File::open(pref.join(filename)).expect("no such file");
    let mut buf = BufReader::new(file);
    let mut res: String = String::new();
    buf.read_to_string(&mut res).expect("Failed to read input");
    res
}