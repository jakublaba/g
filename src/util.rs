use std::fmt::Display;
use std::fs;
use std::path::Path;

pub fn rm_file<P: AsRef<Path> + Display>(path: P) {
    if let Err(_) = fs::remove_file(&path) {
        println!("{path} doesn't exist, skipping");
    } else {
        println!("{path} removed");
    }
}