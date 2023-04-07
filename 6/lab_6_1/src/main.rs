use std::io;
fn main() {
    println!("Введите текст для подсчёта в нём слов:");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Не удалось прочитать ввод");

    let words_quantity = user_input.trim().len() - user_input.trim().replace(" ", "").len() + 1;
    println!("Количество слов в введённом вами тексте - {words_quantity}");
}
