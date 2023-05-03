use std::fs::File;
use std::io::{Seek, SeekFrom, Read};

fn main(){
    let mut file = File::open("text.txt").unwrap();
    file.seek(SeekFrom::Start(10)).unwrap();
    let metadata = file.metadata().expect("error reading metadata");
    let file_len = metadata.len() as usize;
    let mut buf = vec![0 as u8; file_len-10];
    file.read(&mut buf).expect("error reading file");
    for i in &buf{
        print!("{}", *i as char);
    }
}