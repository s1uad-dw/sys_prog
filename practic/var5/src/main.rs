use chrono::Duration;

fn main() {
    let mut start_lesson_time: String = String::new();
    let mut end_lesson_time: String = String::new();

    println!("Введите время начала уроков в формате hh:mm :");
    std::io::stdin().read_line(&mut start_lesson_time)
        .expect("Не удалось считать строку");
    
    println!("Введите время окончания уроков в формате hh:mm :");
    std::io::stdin().read_line(&mut end_lesson_time)
        .expect("Не удалось считать строку");

    let start_lesson_h:i64 = start_lesson_time.trim()[..2]
        .parse().expect("Не удалось преобразовать строку к числу");
    let start_lesson_m:i64 = start_lesson_time.trim()[3..]
        .parse().expect("Не удалось преобразовать строку к числу");

    let end_lesson_h:i64 = end_lesson_time.trim()[..2]
        .parse().expect("Не удалось преобразовать строку к числу");
    let end_lesson_m:i64 = end_lesson_time.trim()[3..]
        .parse().expect("Не удалось преобразовать строку к числу");

    let mut start_lesson_time:Duration = Duration::hours(start_lesson_h) + Duration::minutes(start_lesson_m);

    let end_lesson_time:Duration = Duration::hours(end_lesson_h) + Duration::minutes(end_lesson_m);
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
    while end_lesson_time-start_lesson_time>=Duration::minutes(45){
        let mut h = start_lesson_time.num_seconds()/60/60;
        let mut m = (start_lesson_time.num_seconds()/60)%60;
        print!("{}:{}-", h, m);
        start_lesson_time=start_lesson_time+Duration::minutes(45);
        h = start_lesson_time.num_seconds()/60/60;
        m = (start_lesson_time.num_seconds()/60)%60;
        println!("{}:{}", h, m);
        if end_lesson_time-start_lesson_time>=Duration::minutes(10) {
            start_lesson_time=start_lesson_time+Duration::minutes(10);
        }
    }
        
}
