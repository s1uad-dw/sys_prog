use std::io::Write;


fn main() {
    let mut path:String = String::new();
    println!("Введите полный путь к файлу который хотите проверить:");
    // /home/s1uad_dw/sys_prog/11/lab_11_3/file.txt
    std::io::stdin().read_line(&mut path).expect("Не удалось считать строку.");

    let mut file_text = std::fs::read_to_string(path.trim()).expect("Не удалось открыть файл.");
    let index = file_text.trim().find(" ");
    let mut slice:String = String::new();
    match index {
        Some(i) => {
            slice = file_text[..i].to_string();
        }
        None => println!("Substring not found"),
    }
    file_text.push_str(&slice);
    let mut file = std::fs::File::create(path.trim()).expect("Не удалось открыть файл.");
    write!(file, "{}", file_text).expect("Не удалось записать данные в файл");
    println!("Готово!");
}
