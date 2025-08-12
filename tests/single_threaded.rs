use another_logger::{SingleThreadLogger as Logger, LogLevel};
use std::path::PathBuf;
use std::fs;

#[test]
fn writes_to_file() {
    let path = PathBuf::from("test_integration.log");
    let logger = Logger::new(path.clone()).unwrap();
    //logger.clone();

    logger.write(LogLevel::Warn, "Integration test").unwrap();
    let contents = fs::read_to_string(&path).unwrap();
    assert!(contents.contains("Integration test"));
}

#[test]
fn clear_file() {
    let path = PathBuf::from("test_integration.log");
    let logger = Logger::new(path.clone()).unwrap();

    logger.clear().unwrap();
    let contents = fs::read_to_string(&path).unwrap();
    assert!(contents.contains(""));
}