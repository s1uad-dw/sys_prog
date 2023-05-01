use std::time::{SystemTime};
use std::fs::remove_file;

fn main() -> std::io::Result<()> {
    let file_name = "exaple.txt";
    let created = std::fs::metadata(file_name)?.created()?;
    let system_time = SystemTime::now();
    let duration = system_time.duration_since(created).unwrap_or_else(|error| error.duration());
    let seconds = duration.as_secs();
    println!("Created: {} seconds ago", seconds);
    println!("Size is: {} bytes", std::fs::metadata(file_name)?.len());
    remove_file("exaple.txt")?;
    Ok(())
}