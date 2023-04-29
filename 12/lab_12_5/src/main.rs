use std::fs::File;
use std::io::{Read, Write};

fn main() -> std::io::Result<()>{
    let mut file = File::open("img.bmp")?;
    let metadata = file.metadata()?;
    let len_buf = (metadata.len()+1) as usize;
    let mut buf = vec![0; len_buf];
    file.read(&mut buf)?;
    let mut offset = 54;
    while offset < buf.len()-1 {
        if buf[offset+2] > 145 && buf[offset+1] < 85 && buf[offset] < 85 {
            buf[offset+2] = 0;
        }
        offset+=3;
    }
    let mut file = File::create("nonRed.bmp")?;
    file.write_all(&buf)?;
    Ok(())
}
