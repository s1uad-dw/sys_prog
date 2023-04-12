use std::mem::MaybeUninit;
fn main() {
    struct Student{
        name:String,
        exam1:u8,
        exam2:u8,
        exam3:u8
    }

    let term = console::Term::stdout();
    // let aboba:Student = Student { name: "".to_string(), exam1: 0, exam2: 0, exam3: 0 };
    let mut students: [MaybeUninit<Student>; 5] = unsafe { MaybeUninit::uninit().assume_init() };
    for i in 1..6{
        term.clear_screen().expect("Не удалось очистить консоль.");
        let mut student_name:String = String::new();
        let mut student_exam1:String = String::new();
        let mut student_exam2:String = String::new();
        let mut student_exam3:String = String::new();

        let links:[&mut String; 3] = [&mut student_exam1, &mut student_exam2, &mut student_exam3];

        println!("Заполняем информацию о студенте {i}.\nВведите ФИО студента:");
        std::io::stdin().read_line(&mut student_name)
                .expect("Не удалось считать строку.");
        
        for j in 1..4{
            println!("Введите баллы за {j} экзамен:");
            std::io::stdin().read_line(links[j-1])
                    .expect("Не удалось считать строку.");
        }
        let student:Student = Student{
            name: student_name.trim().to_string(),
            exam1: student_exam1.trim().parse().expect("Некорректно введены данные о первом экзамене."),
            exam2: student_exam2.trim().parse().expect("Некорректно введены данные о втором экзамене."),
            exam3: student_exam3.trim().parse().expect("Некорректно введены данные о третьем экзамене."),
        };
        students[i-1] = MaybeUninit::new(student);

    }
    trait Printer{
        fn print(&self);
    }
    impl Printer for Student{
        fn print(&self){
            println!("
 {}:
 Баллы за первый экзамен - {}
 Баллы за второй экзамен - {}
 Баллы за третий экзамен - {}", self.name, self.exam1, self.exam2, self.exam3);
        }
    }
    term.clear_screen().expect("Не удалось очистить консоль");
    let students:[Student; 5] = unsafe { std::mem::transmute::<_, [Student; 5]>(students) };
    println!("Информация о всех студентах:");
    for i in students{
        i.print();
    }
}
