use std::fs::{create_dir, copy, remove_dir, remove_file};
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()>{
    create_dir("aboba")?;
    copy("data.txt", "aboba/data.txt")?;
    thread::sleep(Duration::from_secs(3));
    remove_file("aboba/data.txt")?;
    remove_dir("aboba")?;
    Ok(())
}
