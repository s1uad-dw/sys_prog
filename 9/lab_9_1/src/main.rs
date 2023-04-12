fn main() {
    struct Purchase{
        product: String,
        price: f64,
        quantity: u64
    }
    
    loop {
        println!("Введите название продукта:");
        let mut product:String = String::new();
        std::io::stdin().read_line(&mut product)
            .expect("Не удалось считать строку!");
        
        println!("Введите цену продукта:");
        let mut price:String = String::new();
        std::io::stdin().read_line(&mut price)
            .expect("Не удалось считать строку!");
        let price:f64 = price.trim().parse().expect("Цена продукта введена некорректно");


        println!("Введите количество продуктов:");
        let mut quantity:String = String::new();
        std::io::stdin()
            .read_line(&mut quantity)
            .expect("Не удалось считать строку!");
        let quantity:u64 = quantity.trim().parse().expect("Количество продуов введено некорректно");

        let current_purchase: Purchase = Purchase {
            product: product,
            price: price,
            quantity: quantity
        };

        println!("Стоимость вашей покупки ({}) составит {}",
        current_purchase.product.trim(),
        current_purchase.price * (current_purchase.quantity) as f64 );

        println!("Хотите посчитать стоимость еще одной покупки? (Y/N)");
        let mut answer: String = String::new();
        loop{
            std::io::stdin().read_line(&mut answer)
                .expect("Не удалось считать строку!");
            answer = answer.trim().to_uppercase();
            if answer == "Y" || answer == "N"{
                break;
            }else{
                println!("Хотите посчитать стоимость еще одной покупки? (Y/N)");
            }
        }
        if answer== "N"{
            break;
        }
    }
}
