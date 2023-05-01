use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut file = File::create("output.txt")?;
    writeln!(file, "{}", input.trim())?;
    println!("{}", &input);
    Ok(())
}