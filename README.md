## SingleThreadLogger
### Important
- No locking mechanisms are in place: behavior is undefined in multi-threaded or async code
- Uses heap allocation
  - Planning to remove heap allocation soon
- Only supports logging to a file (no stdout)

### Methods
- SingleThreadLogger::new(path, format):
  - Creates new file if does not already exist
  - Does not overwrite an existing file
  - Results in an error if any directory in the path does not exist

 - SingleThreadLogger::write(log_level, message):
   - Functions as an append

 - SingleThreadLogger::clear():
   - Truncates log file

### Minimal Example
#### Code
```rust
use another_logger::{
    SingleThreadLogger,
    LogLevel,
    LogFormat,
};

let path = "test.log"
let format = LogFormat::PlainText;
let logger = SingleThreadLogger::new(path, format).unwrap();

logger.clear().unwrap();
logger.write(LogLevel::Info, "Hello Logger!").unwrap();
```

#### Output
```text
2025-08-12 18:52:32 [INFO] Hello Logger!
```


### Dependency Injection Example
- Simply clone
#### Code
```rust
use another_logger::{
    SingleThreadLogger,
    LogLevel,
    LogFormat,
};

let path = "test.log"
let format = LogFormat::PlainText;
let logger = SingleThreadLogger::new(path, format).unwrap();

logger.write(LogLevel::Info, "Hello Logger!").unwrap();

let consumer = Consumer::new(logger.clone());
// call logger.write() from within consumer
```

### TODO
- async/multi-threaded safe logger
- no heap alloc
- more formatting options
- option to disable logging in build --release