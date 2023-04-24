use std::fs::File;
use rand::Rng;
use std::io::prelude::*;

fn main() {
    let mut path:String = String::new();
    println!("Введите название файла для записи:");
    // /home/s1uad_dw/sys_prog/11/lab_12_1/file.txt
    std::io::stdin().read_line(&mut path).expect("Не удалось считать строку.");

    let mut file:File = File::create(path.trim()).expect("Не удалось создать файл.");

    let mut data:Vec<u8> = Vec::new();

    gen_data(&mut data, &mut file);

    fn gen_data(data:&mut Vec<u8>, file:&mut File){
        let number:u8 = rand::thread_rng().gen_range(0..200);
        data.push(number);
        file.write_all(&number.to_le_bytes()).expect("Не удалось записать данные в файл.");
        if data.len()<20{
            gen_data(data, file);
        }
        return;
    }
    drop(data);
    let mut file = File::open(path.trim()).expect("Не удалось открыть файл.");
    let mut buffer:Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("Не удалось прочитать файл.");
    println!("{:?}", buffer);
}
