use std::fs::rename;
use std::io::{stdin, stdout, Write};

fn main() -> std::io::Result<()> {
    let old_file_name = "old.txt";
    let mut new_file_name = String::new();
    print!("Enter new file name: ");
    stdout().flush()?;
    stdin().read_line(&mut new_file_name)?;
    let new_file_name = new_file_name.trim();
    rename(old_file_name, new_file_name)?;
    println!("File renamed successfully.");
    Ok(())
}