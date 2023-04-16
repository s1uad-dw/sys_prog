use std::io::Write;

fn main() {
    let path: &str = "file.txt";
    let mut file = std::fs::File::create(path).expect("Не удалось создать файл.");
    write!(file,
"Я помню чудное мгновенье:
Передо мной явилась ты,
Как мимолетное виденье,
Как гений чистой красоты.").expect("Не удалось записать данные в файл."); 
    println!("Файл записан!");
}
