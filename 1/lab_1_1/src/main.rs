
fn main() {
    let a:i32 = 5;
    let b:i32 = 6;
    
    let c:i32 = a + b;
    let d:i32 = a - b;
    let e:i32 = a * b;
    let f:f32 = (a / b) as f32;
    let g:f32 = (a % b) as f32;
    println!("{}", format!("
Операции и их результаты для чисел a = {a} и b = {b}:\n
  a+b = {c}
  a-b = {d}
  a*b = {e}
  a/b = {f}
  a%b = {g}\n"));
}
