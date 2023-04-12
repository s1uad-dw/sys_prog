use termion::terminal_size;

fn main() {
    let mut text:String = String::new();
    println!("Введите текст:");
    std::io::stdin().read_line(&mut text).expect("Не удалось считать строку");
    let line_size:usize = terminal_size().expect("Не удалось получить размер терминала.").0 as usize;
    while text.len()<=line_size {
        text = " ".to_string() + &text;
    }
    std::thread::sleep(std::time::Duration::from_secs(7));
    loop{
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);
        println!("\x1b[38;5;99m\x1b[48;5;245m{}\x1b[0m", text);
        std::thread::sleep(std::time::Duration::from_millis(100));
        text =  String::new() + &text[1..].replace("\n", "").to_string() + &text[..1].to_string();
    }
}
