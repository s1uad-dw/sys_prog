use std::fs::File;
use std::{io, fs};
use std::io::Write;

fn main() {
    let err = "error read line";
    let mut file_name = String::new();
    println!("input filename: ");
    io::stdin().read_line(&mut file_name).expect(err);
    if let Ok(metadata) = fs::metadata(&file_name) {
        println!("file is responsing. len file is {}", metadata.len());
    } else {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect(err);
        let count =  input.parse().unwrap();
        let mut lines: Vec<String> = vec!["\0".to_string(); count];
        for i in 0..input.parse().unwrap() {
            io::stdin().read_line(&mut lines[i]).expect(err);
        }
        let mut _file = File::open(&file_name).expect("error create file");
        for i in lines {
            _file.write_all(i.as_bytes()).expect("error write in file");
        }
    }
}
