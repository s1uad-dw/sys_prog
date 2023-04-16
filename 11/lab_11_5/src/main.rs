fn main() {
    let mut path:String = String::new();
    println!("Введите полный путь к файлу который хотите проверить:");
    // /home/s1uad_dw/sys_prog/11/lab_11_5/file.txt
    std::io::stdin().read_line(&mut path).expect("Не удалось считать строку.");

    let file_text = std::fs::read_to_string(path.trim()).expect("Не удалось открыть файл.");

    let mut boxes:Vec<&str> = file_text.split_whitespace().collect();
    let _boxes_quantity:i32 = boxes.remove(0).parse().expect("Исходный файл некорректен.");
    let mut no_dub:Vec<i32> = vec![];
    for i in boxes{
        if no_dub.contains(&i.parse().expect("Исходный файл некорректен.")) == false{
            no_dub.push(i.parse().expect("Исходный файл некорректен."));
        };
    }
    let mut boxes:Vec<i32> = no_dub;
    boxes.sort();
    let mut actual_comb:Vec<i32> = Vec::new();
    let mut trying_comb:Vec<i32> = Vec::new();
    for _k in 0..boxes.len(){
        for i in &boxes{
            if let Some(last) = trying_comb.last() {
                if *i-last>2{
                    trying_comb.push(*i);
                }
            } else {
                trying_comb.push(*i);
            }
        }
        boxes.remove(0);
        if trying_comb.len()>actual_comb.len(){
            actual_comb = trying_comb;
            trying_comb = Vec::new();
        }
    }
    println!("Размерность максимально возможной матрешки - {}. Матрёшка:\n{:?}",
    actual_comb.len(), actual_comb)
}
