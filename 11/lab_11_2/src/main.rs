
fn main() {
    let mut path:String = String::new();
    println!("Введите полный путь к файлу который хотите проверить:");
    // /home/s1uad_dw/sys_prog/11/lab_11_2/file.txt
    std::io::stdin().read_line(&mut path).expect("Не удалось считать строку.");

    let file = std::fs::read_to_string(path.trim()).expect("Не удалось открыть файл.");
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
    println!("Результаты сканирования файла {}:", path.trim());
    for i in 18..28{
        println!("Цифр {i} в файле - {}", file.matches(&i.to_string()).count());
    }
}
