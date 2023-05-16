fn main() {
    let mut a:f32 = 3.4;
    let b:f32 = 4.6;
    println!(" x  y    f");
    let mut i = 0;
    while a <= b{
        println!("{:2} {:.1} {:.3}", i, a, get_value(a));
        a+=0.1;
        i +=1;
    }
    let zilihudmain:f32 = find_integral_by_rectangles(&3.4, &4.6);
    println!("метод прямоугольников: {:.3}", zilihudmain);
    println!("Метод симпсона: {}", num_3())
}

fn get_value(x:f32)-> f32{
   let a:f32 = 2.71828182845904523536028747135266249775724709369995957496696762772407663035354759457138217852516642742746639193200305992181741359662904357290033429526059563073813232862794349076323382988075319525101901157383418793070215408914993488416750924476146066808226480016847741185374234544243710753907774499206955170276183860626133138458300075204493382656029760673711320070932870912744374704723069697720931014169283681902551510865746377211125238978442505695369677078544996996794686445490598793163688923009879312773617821542499922957635148220826989519366803318252886939849646510582093923982948879332036250944311730123819706841614039701983767932068328237646480429;
   let stepen = -(x*x)/2.0;
   return 20.0*a.powf(stepen);
}

fn find_integral_by_rectangles(&a:&f32, &b:&f32,)-> f32{
    let mut result:f32 = 0.0;

    let n:i32 = 1000;
    let h:f32 = (b-a) / n as f32;
    println!("h={h}, n={n}");

    for k in 0..n{
        let xk:f32 = a + (k as f32 * h);
        let step_result:f32 = get_value(xk-h/2.0)*h;
        result = result + step_result;
    }
    return result
}

fn num_3()-> f32{
    let a:f32 = 3.4;
    let b:f32 = 4.6;
    let multiplier1 = (b-a)/(3.0*16.0);
    let mut multiplier2 = get_value(3.4);
    let interval:f32 = (b-a)/16.0;

    for i in 1..16{
        multiplier2+=get_value(a+i as f32 * interval) + 2.0 + (i % 2) as f32 *2.0;
    }

    multiplier2+=get_value(b);

    multiplier1*multiplier2
}