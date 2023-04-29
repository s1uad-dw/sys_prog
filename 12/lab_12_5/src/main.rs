use std::fs::File;

fn main() -> Result<()>{
    let mut file = File::open("img.bmp");
    println!("{}", file.read().unwrap())?;
}
