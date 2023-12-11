//! Helper functions for all the tests

/// Attempts to init the logging subsystem.
pub fn log_init() {
    let _ = env_logger::builder().format_timestamp(None).try_init();
}

/// Takes a path to a file and returns a [BufReader](std::io::BufReader) over its contents.
pub fn get_reader<P: AsRef<std::path::Path> + Copy + std::fmt::Display>(path: P)
-> std::io::BufReader<std::fs::File> {
    let input = std::fs::File::open(path)
        .expect(format!("failed to open test input file: {path}").as_str());
    std::io::BufReader::new(input)
}
