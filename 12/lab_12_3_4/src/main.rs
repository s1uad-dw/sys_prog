use std::fs::File;  
use std::io::{Read, Write};
use encoding_rs::UTF_8;

#[derive(Debug)]
struct Student {
    name: String,
    gender: String,
    group: String,
    grades: [u8; 3],
}

#[derive(Debug)]
enum Data {
    Numbers([u8; 3]),
    Text(String),
}

fn data_to_string(data: &Data) -> String {
    match data {
        Data::Text(s) => (*s).clone(),
        Data::Numbers(_num) => "\0".to_string(),
    }
}

fn data_to_value(data: &Data) -> [u8; 3]{
    match data {
        Data::Text(_s) => [(0 as u8); 3],
        Data::Numbers(num) => *num,
    }
}

fn print_struct(student: &Student) {
    println!("\n\n");
    println!("Name: {}", student.name);
    println!("Gendwer: {}", student.gender);
    println!("Group: {}", student.group);
    for i in 0..student.grades.len(){
        println!("Discipline {}: {}", i, student.grades[i]);
    }
}

fn main() -> std::io::Result<()> {
    // for write
    let students = vec![
        Student {
            name: "Группа Б".to_string(),
            gender: "Группа Б".to_string(),
            group: "Группа Б".to_string(),
            grades: [90, 95, 92],
        },
        Student {
            name: "Райн Гослинг".to_string(),
            gender: "мужык".to_string(),
            group: "обкончалась когда его увидела".to_string(),
            grades: [85, 80, 90],
        },
        Student {
            name: "Афанасий".to_string(),
            gender: "не понятный".to_string(),
            group: "пованивающих уебанов".to_string(),
            grades: [95, 90, 85],
        },
        Student {
            name: "иуууууууууууууууу".to_string(),
            gender: "запрэдельный ибаааааааааать".to_string(),
            group: "смайлфэйс".to_string(),
            grades: [80, 85, 90],
        },
        Student {
            name: "хочу".to_string(),
            gender: "бананы".to_string(),
            group: "ХОЧУ БАНАНЫ!".to_string(),
            grades: [90, 92, 95],
        },
    ];
    let mut file = File::create("students.bin")?;
    file.write_all(&[(students.len() as u8); 1])?;
    for student in &students {
        let student_data = vec![Data::Text(student.name.clone()), Data::Text(student.gender.clone()), Data::Text(student.group.clone()), Data::Numbers(student.grades)];
        for item in student_data{
            match item {
                Data::Numbers(item) =>{
                    for val in &item{
                        file.write_all(&[*val; 1])?;
                    }
                },
                Data::Text(item) => {
                    file.write_all(&[(item.len() as u8);1])?;
                    file.write_all(item.as_bytes())?;
                },
            }
        }
    }

    // for read
    let mut file = File::open("students.bin")?;
    let metadata = file.metadata()?;
    let len_buf = (metadata.len()+1) as usize;
    let mut buffer = vec![0; len_buf];
    let mut students = Vec::new();
    
    while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        let mut offset = 0;
        let mut student_data = vec![Data::Text(String::new()), Data::Text(String::new()), Data::Text(String::new()), Data::Numbers([0; 3])];
        let count = buffer[offset] as u8;
        offset+=1;
        for _index in 0..count{
            for item in &mut student_data{
                match item {
                    Data::Text(s) => {
                        offset+=1;
                        let trans:Vec<u8> = buffer[offset..offset+(buffer[offset-1]) as usize].to_vec();
                        let (decoded, _, _) = UTF_8.decode(&trans);
                        *s = decoded.to_string();
                        *s +="\0";
                        offset += (buffer[offset-1]) as usize;
                    },
                    Data::Numbers(num) => {
                        for i in 0..3{
                            num[i as usize] = buffer[offset+(i as usize)] as u8;
                        }
                        offset += 3;
                    },
                }
            }
            let student:Student = Student{
                name: data_to_string(&student_data[0]),
                gender: data_to_string(&student_data[1]),
                group: data_to_string(&student_data[2]),
                grades: data_to_value(&student_data[3]),
            };
            students.push(student);
            student_data = vec![Data::Text(String::new()), Data::Text(String::new()), Data::Text(String::new()), Data::Numbers([0; 3])];
        }
    }
    for student in students{
        print_struct(&student)
    }
    Ok(())
}