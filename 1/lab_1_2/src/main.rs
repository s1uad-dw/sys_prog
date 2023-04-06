use std::io;
use round::{round};
fn main() {
    println!("Введите первое дейстительное число:");
    let mut first_number = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect("Не удалось прочитать ввод");

    println!("Введите второе дейстительное число:");
    let mut second_number = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Не удалось прочитать ввод");


    let first_number:f64 = first_number.trim().replace(',', ".").parse()
        .expect("Введите действительное число");

    let second_number:f64 = second_number.trim().replace(',', ".").parse()
    .expect("Введите действительное число");

    let summ:f64 = round(first_number + second_number, 3);
    let difference:f64 = round(first_number - second_number, 3);
    let composition:f64 = round(first_number * second_number, 3);
    let quotient:f64 = round(first_number / second_number, 3);
    
    println!("
Операции и их результаты для чисел a = {first_number} и b = {second_number}:\n
  a+b = {summ}
  a-b = {difference}
  a*b = {composition}
  a/b = {quotient}\n");
}
