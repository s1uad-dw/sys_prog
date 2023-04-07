
fn main() {
    let systems: [&str; 9] = ["Авиокомпания", "Мир", "American Express", "Visa", "Mastercard", "Union Play", "Топливная сфера", "Сектор телекоммуникаций", "Государственный сектор"];
    let mut user_input = String::new();
        println!("Введите номер вашей банковской карты:");
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Не удалось прочитать ввод");
    
    let system_id:&str = &user_input[0..1];
    let system_id:usize = system_id.parse().expect("Введенный номер карты некорректен.");
    let card_system = systems[system_id-1];
    let bin:&str = &user_input[0..6];

    println!("Ваша банковская карта принадлежит системе {}.\nВаш БИН(Банковский идентификационный номер) - {}", card_system, bin);
}
