#[allow(dead_code)]

use std::io::Write;

pub struct Logger {
    enable_logger: bool,
    file_path: String
}

#[allow(dead_code)]
impl Logger {
    pub fn new_logger(fpath: &str) -> Logger {
        return Logger {
            file_path: String::from(fpath),
            enable_logger: false,
        };
    }
    pub fn enable(&mut self) {
        self.enable_logger = true;
    }
    pub fn disable(&mut self) {
        self.enable_logger = false;
    }
    pub fn log(&self, msg: &String) {
        if self.enable_logger {
            let path = std::path::Path::new(&self.file_path);
            if !path.exists() {
                std::fs::File::create(&path).expect("Ouch! Can't create log file.");
            }
            let mut file = match std::fs::OpenOptions::new().write(true).append(true).open(&path) {
                Err(why) => panic!("couldn't open {}: {}", path.display(), why),
                Ok(file) => file,
            };
            file.write(msg.as_bytes()).expect("can't log!");
        }
    }
}
