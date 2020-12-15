use chrono::prelude::*;
use std::fs;

fn list_files_in_directory(path: &str) -> Vec<String>{
    let mut files: Vec<String> = Vec::new();
    let directory = fs::read_dir(path).unwrap();
    for file in directory {
        files.push(file.unwrap().path().display().to_string());
    }
    files
}
fn main() {
    let date_format = "%d_%m_%Y";
    let dt = Utc::now();
    let a = dt.format(&date_format).to_string();
    //println!("{}", a);

    let files= list_files_in_directory("./logs");
    for file in files{
        println!("file: {}",file);
        let mut splits: Vec<&str>= file.split("/").collect();
        let file_name = splits.last().unwrap();
        println!("filename : {}", file_name);
    }
}
