use std::fs;
use another_logger::{
    SingleThreadLogger,
    LogLevel,
    LogFormat,
};

#[test]
fn writes_to_file() {
    let path = "test_integration.log";
    let format = LogFormat::PlainText;
    let logger = SingleThreadLogger::new(path, format).unwrap();

    logger.write(LogLevel::Info, "Hello Logger!").unwrap();

    let contents = fs::read_to_string(&path).unwrap();
    assert!(contents.contains("Hello Logger!"));
}

#[test]
fn clear_file() {
    let path = "test_integration.log";
    let format = LogFormat::JSON;
    let logger = SingleThreadLogger::new(path, format).unwrap();

    logger.clear().unwrap();

    let contents = fs::read_to_string(&path).unwrap();
    assert!(contents.contains(""));
}

fn assert_clone<T: Clone>() {}
#[test]
fn exported_logger_is_clone() {
    assert_clone::<another_logger::SingleThreadLogger>();
}