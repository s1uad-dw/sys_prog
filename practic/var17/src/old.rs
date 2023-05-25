use std::fs::File;
use std::io::{Read, Write, stdin};

struct Participant{
    club:String,
    surname:String,
    start_time:f64,
    finish_time:f64,
}

fn main(){
    //Получить количество заносимых участников
    let mut participants: Vec<Participant> = Vec::new();
    // Заполнение структуры данными
    loop{
        let current_p:Participant = create_participant();
        participants.push(current_p);
        write_struct(&mut participants).unwrap();
        for i in &mut participants{
            if i.finish_time - i.start_time <= 100.0{
                println!("{}", i.surname);
            }
        }   
    }
}

fn create_participant() -> Participant {
    let mut club:String = String::new();
    let mut surname:String = String::new();
    let mut start_time:String = String::new();
    let mut finish_time:String = String::new();

    println!("Введите клуб:");
    stdin().read_line(&mut club).expect("Не удалось считать строку.");
    println!("Введите фамилию:");
    stdin().read_line(&mut surname).expect("Не удалось считать строку.");
    println!("Введите время старта hh:mm:ss:");
    stdin().read_line(&mut start_time).expect("Не удалось считать строку.");
    println!("Введите время финиша hh:mm:ss:");
    stdin().read_line(&mut finish_time).expect("Не удалось считать строку.");

    let start_time: Vec<&str> = start_time.trim().split(':').collect();

    let finish_time: Vec<&str> = finish_time.trim().split(':').collect();


    let start_h:i32 = start_time[0].parse().expect("Не удалось преобразовать строку к числу");
    let start_m:i32 = start_time[1].parse().expect("Не удалось преобразовать строку к числу");
    let start_s:i32 = start_time[2].parse().expect("Не удалось преобразовать строку к числу");
    let start_time:f64 = start_h as f64 * 3600.0 + start_m as f64 * 60.0 + start_s as f64;

    let finish_h:i32 = finish_time[0].parse().expect("Не удалось преобразовать строку к числу");
    let finish_m:i32 = finish_time[1].parse().expect("Не удалось преобразовать строку к числу");
    let finish_s:i32 = finish_time[2].parse().expect("Не удалось преобразовать строку к числу");
    let finish_time:f64 = finish_h as f64 * 3600.0 + finish_m as f64 * 60.0 + finish_s as f64;

    return Participant { club: club, surname: surname, start_time: start_time, finish_time: finish_time }    
}
fn write_struct(participants:&mut Vec<Participant>)-> std::io::Result<()>{
    let mut file = File::create("db.bin")?;
    file.write_all(&[participants.len() as u8])?;
    for i in participants{
        let len_club = i.club.len() as u8;
        let club = i.club.as_bytes();
        let len_surname = i.surname.len() as u8;
        let surname = i.surname.as_bytes();
        let start_time = i.start_time as u8;
        let finish_time = i.finish_time as u8;
        file.write_all(&[len_club])?;
        file.write_all(club)?;
        file.write_all(&[len_surname])?;
        file.write_all(surname)?;
        file.write_all(&[start_time])?;
        file.write_all(&[finish_time])?;
    }
    Ok(())
}



//14
