use std::io;
fn main() {
    println!("Введите первое целое число:");
    let mut first_number = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect("Не удалось прочитать ввод!");

    println!("Введите второе целое число:");
    let mut second_number = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Не удалось прочитать ввод!");


    let first_number:i32 = first_number.trim().parse()
        .expect("Введите целое число!");

    let second_number:i32 = second_number.trim().parse()
    .expect("Введите целое число!");


    let term = console::Term::stdout();
    term.clear_screen().expect("Не удалось очистить консоль");
    
    println!(
        "{0: <10} | {1: <60}",
        "", "Система счисления"
    );println!(
        "{0: <10} | {1: <20} | {2: <20} | {3: <20}",
        "Значение", "Десятеричная", "Восьмеричная", "Шестнадцатеричная"
    );println!(
        "{0: <10} | {1: <20} | {2: <20} | {3: <20}",
        "a", first_number, format!("0o{:o}", first_number), format!("0x{:x}", first_number)
    );println!(
        "{0: <10} | {1: <20} | {2: <20} | {3: <20}",
        "b", second_number, format!("0o{:o}", second_number), format!("0x{:x}", second_number)
    );println!(
        "{0: <10} | {1: <20} | {2: <20} | {3: <20}",
        "a+b", first_number+second_number, format!("0o{:o}", first_number+second_number), format!("0x{:x}", first_number+second_number)
    );println!(
        "{0: <10} | {1: <20} | {2: <20} | {3: <20}",
        "a-b", first_number-second_number, format!("0o{:o}", first_number-second_number), format!("0x{:x}", first_number-second_number)
    );println!(
        "{0: <10} | {1: <20} | {2: <20} | {3: <20}",
        "a*b", first_number*second_number, format!("0o{:o}", first_number*second_number), format!("0x{:x}", first_number*second_number)
    );println!(
        "{0: <10} | {1: <20} | {2: <20} | {3: <20}",
        "a/b", first_number/second_number, format!("0o{:o}", first_number/second_number), format!("0x{:x}", first_number/second_number)
    );
}
