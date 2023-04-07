use std::io;
fn main() {
    let mut user_input = String::new();
        println!("Введите ДНК:");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Не удалось прочитать ввод");
        print!("РНК, полученый из введённого вами ДНК:\n{}", user_input.replace("T", "U"))
}
