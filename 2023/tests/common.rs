//! Helper functions for all the tests

use std::{
    fmt::Display,
    fs::File,
    io::BufReader,
    path::Path
};

/// Attempts to init the logging subsystem.
pub fn log_init() {
    let _ = env_logger::builder().format_timestamp(None).try_init();
}

/// Takes a path to a file and returns a [BufReader](std::io::BufReader) over its contents.
pub fn get_reader<P: AsRef<Path> + Copy + Display>(path: P) -> BufReader<File> {
    let input = std::fs::File::open(path)
        .expect(format!("failed to open test input file: {path}").as_str());
    std::io::BufReader::new(input)
}


pub use InputKind::*;
pub enum InputKind {
    Example,
    Example2,
    Full,
}

fn get_filename(day: u8, kind: InputKind) -> String {
    let file = match kind {
        Example => "example",
        Example2 => "example2",
        Full => "input",
    };

    format!("inputs/day{day:02}/{file}")
}

pub fn get_input(day: u8, kind: InputKind) -> BufReader<File> {
    get_reader(get_filename(day, kind).as_str())
}

#[macro_export]
macro_rules! get_day {
    () => {
        {
            let modpath = module_path!().as_bytes();
            let tens = modpath[modpath.len() - 2] - 0x30;
            let ones = modpath[modpath.len() - 1] - 0x30;
            tens * 10 + ones
        }
    }
}
