use chrono::{Local, Datelike, Timelike};

const MONTHS:[&str;12]=[
    "января", "февраля", "марта", "апреля", "мая", "июня",
    "июля", "августа", "сентября", "октября", "ноября", "декабря"
];
const WEEKDAYS:[&str;7]=[
    "понедельник", "вторник", "среда", "четверг", "пятница", "суббота", "воскренье"
];
fn main() {

    let now = Local::now();
    println!("Сегодня: {} {}, {}, текущее: {}:{:02}.",
        now.day(), MONTHS[(now.month()-1) as usize], WEEKDAYS[(now.weekday().num_days_from_monday()-1) as usize],
        now.hour(), now.minute()
    );
}
