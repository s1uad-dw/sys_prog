use std::fs::File;
use std::io::{Read, Write};

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
        Data::Numbers(num) => "\0".to_string(),
    }
}

fn data_to_value(data: &Data) -> [u8; 3]{
    match data {
        Data::Text(s) => [(0 as u8); 3],
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
            name: "Jane".to_string(),
            gender: "Female".to_string(),
            group: "Group B".to_string(),
            grades: [90, 95, 92],
        },
        Student {
            name: "Bob".to_string(),
            gender: "Male".to_string(),
            group: "Group A".to_string(),
            grades: [85, 80, 90],
        },
        Student {
            name: "Alice".to_string(),
            gender: "Female".to_string(),
            group: "Group C".to_string(),
            grades: [95, 90, 85],
        },
        Student {
            name: "David".to_string(),
            gender: "Male".to_string(),
            group: "Group B".to_string(),
            grades: [80, 85, 90],
        },
        Student {
            name: "Sara".to_string(),
            gender: "Female".to_string(),
            group: "Group A".to_string(),
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
        for index in 0..count{
            for item in &mut student_data{
                match item {
                    Data::Text(s) => {
                        offset+=1;
                        for i in 0..buffer[offset-1]{
                            *s+=std::str::from_utf8(&[buffer[offset+(i as usize)];1]).unwrap();
                        }
                        *s +="\0";
                        offset += buffer[offset-1] as usize;
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