use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("example.bin").unwrap();
    let mut buffer = [0; 240];
    file.read(&mut buffer).unwrap();

    for (i, byte) in buffer.iter().enumerate() {
        if i % 60 == 0 {
            println!("");
        }
        print!("\x1b[32;40m{:02X} ", byte);
    }
    println!("\x1b[0m");
}


// ----------generate bin file----------

// use std::fs::File;
// use std::io::Write;
// use rand::Rng;

// fn main() {
//     let mut file = File::create("example.bin").unwrap();
//     let mut buffer = [0; 240];
//     let mut rng = rand::thread_rng();
//     for i in 0..buffer.len(){
//         buffer[i] = rng.gen_range(0..10);
//     }

//     file.write_all(&buffer).unwrap();
// }