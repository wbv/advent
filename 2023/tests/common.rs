//! Helper functions for all the tests

use std::{
    fmt::Display,
    fs::File,
    io::BufReader,
    path::Path
};

#[allow(dead_code)]
/// Attempts to init the logging subsystem.
pub fn log_init() {
    let _ = env_logger::builder().format_timestamp(None).try_init();
}

#[allow(dead_code)]
/// Takes a path to a file and returns a [BufReader](std::io::BufReader) over its contents.
pub fn get_reader<P: AsRef<Path> + Copy + Display>(path: P) -> BufReader<File> {
    let input = std::fs::File::open(path.as_ref())
        .expect(format!("failed to open test input file: {path}").as_str());
    std::io::BufReader::new(input)
}

#[allow(dead_code)]
fn get_filename<S: AsRef<str>>(day: u8, file: S) -> String {
    format!("inputs/day{day:02}/{}", file.as_ref())
}

#[allow(dead_code)]
/// Calls [`get_reader`] with the correct path, given a day and filename
pub fn get_input<S: AsRef<str>>(day: u8, kind: S) -> BufReader<File> {
    let filename = get_filename(day, kind.as_ref());
    get_reader(filename.as_str())
}

#[macro_export]
/// Auto-expands to the day (as a [`u8`]) corresponding based on the current filename.
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
