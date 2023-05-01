use std::fs::{OpenOptions, File};
use std::io::Write;
use std::sync::{Arc, Mutex};
use chrono::Local;

pub trait Logger {
    fn logln<S : Into<String>>(&mut self, line : S);
}

impl<'a, T : Logger> Logger for Arc<Mutex<T>> {
    #[inline(always)]
    fn logln<S : Into<String>>(&mut self, line : S) {
        self.lock().unwrap().logln(line)
    }
}

pub struct FileLogger {
    file : File
}

impl FileLogger {
    pub fn new(path: String) -> Self { 
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path).unwrap();

        Self { file } 
    }
}

impl Logger for FileLogger {
    fn logln<S : Into<String>>(&mut self, line : S) {
        let line_str = line.into();
        let time_stamp = Local::now().format("%H:%M:%S");

        println!("[{}] {}", time_stamp, line_str); 
        writeln!(&mut self.file, "[{}] {}", time_stamp, line_str).unwrap(); 
    }
}