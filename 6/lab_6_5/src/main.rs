fn main() {
    let alphabet:&str ="abcdefghigklmnopqrstuvwxyz" ;
    let mut k:String = String::new();

    //Получиение числа К
    println!("Введите число k:");
    std::io::stdin()
        .read_line(&mut k)
        .expect("Не удалось считать строку!");
    let k:usize = k.trim().parse().expect("Число k не соответствует требованиям!");

    //Получиение строки которую надо зашифровать
    let mut user_input:String = String::new();
        println!("Введите строку которую хотите зашифровать:");
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Не удалось считать строку!");
    
    let mut result:String = String::new();
    
    for user_input_char in user_input.trim().chars(){
        if user_input_char.is_alphabetic(){
            let new_char_position:usize = alphabet.find(user_input_char).expect("msg");
            let new_char_position:usize = (new_char_position+k)%alphabet.len();
            result+=&alphabet[new_char_position..new_char_position+1];
        }else{
            result.push(user_input_char);
        }
        
    }
    println!("Зашифрованная строка:\n{}", result);
}
