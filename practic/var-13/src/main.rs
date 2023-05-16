fn main() {
    let mut a:f32 = 3.4;
    let mut b:f32 = 4.6;
    print_table(a.clone(), b.clone())
}

fn print_table(mut a:f32, b: f32){
    println!(" x  y");
    let mut i = 0;
    while a <= b{
        println!("{:2} {:.1}", i, a);
        a+=0.1;
        i +=1;
    }
}
