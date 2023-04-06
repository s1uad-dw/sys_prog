use std::io;
fn main() {
    let mut user_input = String::new();
    loop {
        println!("Введите фразу из 3 отдельных слов:");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Не удалось прочитать ввод");

        let words_quantity = user_input.trim().len() - user_input.trim().replace(" ", "").len() + 1;

        if words_quantity == 3 {
            let term = console::Term::stdout();
            term.clear_screen().expect("Не удалось очистить консоль");
            println!("Ваша фраза:\n{}", user_input.trim());
            return;
        }
    }   
    
}
