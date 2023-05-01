use std::fs::{File, remove_file};
use std::io::{Read, Write};

fn main() -> std::io::Result<()>{
    let buf = {
        let mut f = File::open("data.txt")?;
        let metadata = f.metadata()?;
        let len =  metadata.len() as usize;
        let mut buf=vec![0; len];
        f.read(&mut buf)?;
        buf
    };
    {
        let mut new_buf = Vec::new();
        let mut file = File::create(".data.txt")?;
    
        for i in 10..buf.len(){
            new_buf.push(buf[i]);
        }
        for i in 0..10 {
            new_buf.push(buf[i]);
        }
        file.write_all(&new_buf)?;
    }    
        
    let write_data = {
        let mut file = File::open(".data.txt")?;
        let metadata = file.metadata()?;
        let len = metadata.len() as usize;
        let mut buffer = vec![0; len];
        file.read(&mut buffer)?;
        remove_file(".data.txt")?;
        println!("{:?}", buffer);
        buffer
    };

    let mut f = File::create("data.txt")?;
    f.write_all(&write_data)?;
    Ok(())
}
