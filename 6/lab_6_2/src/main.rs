use std::{io};
fn main() {
    let mut _user_input = String::new();
    loop {
        _user_input = String::new();
        println!("Введите фразу из 3 отдельных слов:");
        io::stdin()
            .read_line(&mut _user_input)
            .expect("Не удалось прочитать ввод");

        let words_quantity = _user_input.trim().len() - _user_input.trim().replace(" ", "").len() + 1;

        if words_quantity == 3 {
            let term = console::Term::stdout();
            term.clear_screen().expect("Не удалось очистить консоль");
            println!("Ваша фраза:\n{}", _user_input.trim());
            break;
        }
    }

    let mut user_input_chars: Vec<char> = _user_input.chars().collect();

    let mut output = String::new();

    for i in 0..user_input_chars.len(){
        if user_input_chars[i] == ' '{
            user_input_chars[i+1] = user_input_chars[i+1].to_uppercase().next().unwrap();
        }else if i == 0{
            user_input_chars[i] = user_input_chars[i].to_uppercase().next().unwrap();
        }   
        output.push(user_input_chars[i])
    }
    println!("\nВаша фраза в верблюжей нотации:\n{}", output.trim().replace(' ', ""));
}
