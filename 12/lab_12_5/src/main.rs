use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()>{
    let mut file = File::open("img.bmp")?;
    let metadata = file.metadata()?;
    let len_buf = (metadata.len()+1) as usize;
    let mut buffer = vec![0; len_buf];
    file.read(&mut buffer)?;
    println!("{:?}", buffer);
    Ok(())
}
