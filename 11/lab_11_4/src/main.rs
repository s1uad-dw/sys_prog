const VOWELS:[&str; 2] = ["A", "O"];
const CONSONANTS:[&str; 3] = ["C", "D", "F"];

fn main() {
    let mut path:String = String::new();
    println!("Введите полный путь к файлу который хотите проверить:");
    // /home/s1uad_dw/sys_prog/11/lab_11_4/file.txt
    std::io::stdin().read_line(&mut path).expect("Не удалось считать строку.");

    let file_text = std::fs::read_to_string(path.trim()).expect("Не удалось открыть файл.");
    
    let mut streek:u32 = 0;
    let mut i: usize = 0;
    while i < file_text.len(){
        let current_symbol: &str = &file_text[i..i+1];
        let next_symbol: &str = &file_text[i+1..i+2];

        if CONSONANTS.contains(&current_symbol) &&
            VOWELS.contains(&next_symbol){
            streek+=1;
            i+=1;
        }else{
            streek = 0;
        }
        // println!("{}|{}|{}|{}", i, current_symbol, next_symbol, streek);
        i+=1;
    }
    println!("Максимальное количество идущих подряд пар вида согласная + гласная: {streek}");
}
