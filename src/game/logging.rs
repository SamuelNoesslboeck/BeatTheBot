use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};

use chrono::Utc;

pub trait Logger {
    fn logln(&mut self, line : String);
}

impl<'a, T : Logger> Logger for Arc<Mutex<T>> {
    #[inline(always)]
    fn logln(&mut self, line : String) {
        self.lock().unwrap().logln(line)
    }
}

#[derive(Clone)]
pub struct FileLogger {
    path : String
}

impl FileLogger {
    pub fn new(path: String) -> Self { 
        Self { path } 
    }
}

impl Logger for FileLogger {
    fn logln(&mut self, line : String) {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.path).unwrap();

        println!("[{}] {}", Utc::now().format("%H:%M:%S"), line); 
        writeln!(&mut file, "[{}] {}", Utc::now().format("%H:%M:%S"), line).unwrap(); 
    }
}