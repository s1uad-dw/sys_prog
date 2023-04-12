struct Fraction{
    numerator: i32,
    denominator: u32
}
fn addition(f_fraction:&Fraction, s_fraction:&Fraction) -> Fraction{
    let mut f_fraction_numerator:i32 = f_fraction.numerator;
    let mut f_fraction_denominator:u32 = f_fraction.denominator;

    let mut s_fraction_numerator:i32 = s_fraction.numerator;
    // let s_fraction_denominator:u32 = s_fraction.denominator as i32;

    if f_fraction_denominator != s_fraction.denominator{
        f_fraction_numerator *= s_fraction.denominator as i32;
        f_fraction_denominator *= s_fraction.denominator;

        s_fraction_numerator *= f_fraction.denominator as i32;
    }

    return Fraction {
        numerator: f_fraction_numerator + s_fraction_numerator,
        denominator: f_fraction_denominator as u32,
    }
}
fn multiplication(f_fraction:&Fraction, s_fraction:&Fraction) -> Fraction{
    return Fraction {
        numerator: f_fraction.numerator * s_fraction.numerator,
        denominator: f_fraction.denominator * s_fraction.denominator
    }
}
fn take_fraction() -> Fraction{
    println!("Введите числитель дроби:");
    let mut numerator:String = String::new();
    std::io::stdin().read_line(&mut numerator)
    .expect("Не удалось считать строку");

    println!("Введите знаменатель дроби:");
    let mut demominator:String = String::new();
    std::io::stdin().read_line(&mut demominator)
    .expect("Не удалось считать строку");

    let numerator:i32 = numerator.trim().parse().expect("Числитель введен некорректно");
    let denominator:u32 = demominator.trim().parse().expect("Знаменатель ввден некорректно");

    return Fraction { numerator: numerator, denominator: denominator }
}
fn main() {
    let term = console::Term::stdout();

    term.clear_screen().expect("Не удалось очистить консоль");
    println!("Получение первой дроби:");
    let a:Fraction = take_fraction();
    term.clear_screen().expect("Не удалось очистить консоль");
    println!("Получение второй дроби:");
    let b:Fraction = take_fraction();
    term.clear_screen().expect("Не удалось очистить консоль");
    let addition = addition(&a, &b);
    let multiplication = multiplication(&a, &b);
    println!("a = {}/{}, b = {}/{}\na + b = {}/{}\na * b = {}/{}", 
a.numerator, a.denominator, b.numerator, b.denominator,
addition.numerator, addition.denominator, multiplication.numerator, multiplication.denominator);
}
