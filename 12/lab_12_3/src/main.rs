use std::io;
use std::fs::File;
use std::io::Write;

#[repr(C)]
#[derive(Default, Clone)]
struct Student {
    name: String,
    surname: String,
    patronomic: String,
    sexual_orientation: String,
    group: String,
    discipline_1: u8,
    discipline_2: u8,
    discipline_3: u8, 
}

impl Student {
    fn clear(&mut self) {
        self.name.clear();
        self.surname.clear();
        self.sexual_orientation.clear();
        self.group.clear();
        self.discipline_1 = 0;
        self.discipline_2 = 0;
        self.discipline_3 = 0;
    }
}

fn get_int() -> u8{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return match input.trim().parse::<u8>() {
        Ok(value) => value,
        Err(_) => 0,
    };
}

fn print_struct(student: &Student) {
    println!("\n\n");
    print!("Name: {}", student.name);
    print!("Surname: {}", student.surname);
    print!("Patronomic: {}", student.patronomic);
    print!("Orientation: {}", student.sexual_orientation);
    print!("Group: {}", student.group);
    println!("Discipline 1: {}", student.discipline_1);
    println!("Discipline 2: {}", student.discipline_2);
    println!("Disuracy 3: {}", student.discipline_3);
}

fn main() {
    let count: u8 = 2;
    let mut students = Vec::new();
    
    let mut student = Student::default();
    let error_message = "Failed to read line";


    for i in 0..count {
        println!("Введите имя студента {}", i);
        io::stdin().read_line(&mut student.name).expect(error_message);
        println!("Введите фамилию студента {}", i);
        io::stdin().read_line(&mut student.surname).expect(error_message);
        println!("Введите отчество студента {}", i);
        io::stdin().read_line(&mut student.patronomic).expect(error_message);
        println!("Введите ориентацию студента {}", i);
        io::stdin().read_line(&mut student.sexual_orientation).expect(error_message);
        println!("Введите группу студента {}", i);
        io::stdin().read_line(&mut student.group).expect(error_message);
        println!("Введите баллы студента за 1-ый экзамен {}\n", i);
        student.discipline_1 = get_int();
        println!("Введите баллы студента за 2-ый экзамен {}\n", i);
        student.discipline_2 = get_int();
        println!("Введите баллы студента за 3-ый экзамен {}\n", i);
        student.discipline_3 = get_int();
        
        students.push(student.clone());
        student.clear();
    }

    let mut file = File::create("person.bin").expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA PIZDA TEBE NAHUI");
    for stud in &students {
        let name_bytes = stud.name.as_bytes();
        let name_len = name_bytes.len() as u32;
        let surname = stud.surname.as_bytes();
        let surname_len = surname.len() as u32;
        let patronomic = stud.patronomic.as_bytes();
        let patronomic_len = patronomic.len() as u32;
        let sexual_orientations = stud.patronomic.as_bytes();
        let sexual_orientations_len = sexual_orientations.len() as u32;
        let group = stud.group.as_bytes();
        let group_len = group.len() as u32;
        let discipline_1 = stud.discipline_1;
        let discipline_2 = stud.discipline_2;
        let discipline_3 = stud.discipline_3;
        file.write_all(&name_len.to_le_bytes()).expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA PIZDA TEBE NAHUI");
        file.write_all(name_bytes).expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA PIZDA TEBE NAHUI");
        file.write_all(&surname_len.to_le_bytes()).expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA PIZDA TEBE NAHUI");
        file.write_all(surname).expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA PIZDA TEBE NAHUI");
        file.write_all(&patronomic_len.to_le_bytes()).expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA PIZDA TEBE NAHUI");
        file.write_all(patronomic).expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA PIZDA TEBE NAHUI");
        file.write_all(&sexual_orientations_len.to_le_bytes()).expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA PIZDA TEBE NAHUI");
        file.write_all(sexual_orientations).expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA PIZDA TEBE NAHUI");
        file.write_all(&group_len.to_le_bytes()).expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA PIZDA TEBE NAHUI");
        file.write_all(group).expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA PIZDA TEBE NAHUI");
        file.write_all(&group_len.to_le_bytes()).expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA PIZDA TEBE NAHUI");
        file.write_all(group).expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA PIZDA TEBE NAHUI");
        file.write_all(&discipline_1.to_le_bytes()).expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA PIZDA TEBE NAHUI");
        file.write_all(&discipline_2.to_le_bytes()).expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA PIZDA TEBE NAHUI");
        file.write_all(&discipline_3.to_le_bytes()).expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA PIZDA TEBE NAHUI");
    }

}


