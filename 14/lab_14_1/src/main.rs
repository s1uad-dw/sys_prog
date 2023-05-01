use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let file_name = "example.txt";
    let mut file = File::create(file_name)?;
    file.write_all(b"Hello, world!")?;
    let full_path = PathBuf::from(file_name).canonicalize()?;
    println!("Drive: {:?}", full_path.as_path().components().nth(0).unwrap());
    print!("Folders: ", );
    for i in full_path.parent().unwrap().components().skip(1).collect::<Vec<_>>(){
        print!("{:?} ", i);
    }
    println!();
    println!("File name: {:?}", full_path.file_name().unwrap());
    println!("Extension: {:?}", full_path.extension().unwrap());
    Ok(())
}