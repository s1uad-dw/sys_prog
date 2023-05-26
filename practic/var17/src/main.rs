use gtk::prelude::*;
use gtk::{gio, glib, ButtonsType, DialogFlags, MessageDialog, MessageType, Window};
use std::fs::File;
use std::io::{Read, Write};

struct Participant {
    club: String,
    surname: String,
    start_time: f64,
    finish_time: f64,
}

static mut participants: Vec<Participant> = Vec::new();

fn main() {
    // read_data();
    let application = gtk::Application::new(Some("aboba.bababababa"), Default::default());
    application.connect_activate(build_ui);

    // When activated, shuts down the application
    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(
        glib::clone!(@weak application => move |_action, _parameter| {
            application.quit();
        }),
    );
    application.connect_startup(|application| {
        application.set_accels_for_action("app.quit", &["<Primary>Q"]);
    });
    application.add_action(&quit);

    // Run the application
    application.run();
}

fn build_ui(application: &gtk::Application) {
    // create the main window
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Лыжники WinApi");
    window.set_border_width(5);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(840, 480);

    // Create a title label

    let view = gtk::Button::with_label("Показать лыжников");

    let completion_countries = gtk::EntryCompletion::new();
    // Use the first (and only) column available to set the autocompletion text
    completion_countries.set_text_column(0);
    // how many keystrokes to wait before attempting to autocomplete?
    completion_countries.set_minimum_key_length(1);
    // whether the completions should be presented in a popup window
    completion_countries.set_popup_completion(true);

    let title_club = gtk::Label::new(None);
    title_club.set_markup("<big>Введите Клуб</big>");
    let input_club = gtk::Entry::new();
    input_club.set_completion(Some(&completion_countries));

    let title_surname = gtk::Label::new(None);
    title_surname.set_markup("<big>Введите Фамилию</big>");
    let input_surname = gtk::Entry::new();
    input_surname.set_completion(Some(&completion_countries));

    let title_start_time = gtk::Label::new(None);
    title_start_time.set_markup("<big>Введите время старта hh:mm:ss</big>");
    let input_start_time = gtk::Entry::new();
    input_start_time.set_completion(Some(&completion_countries));

    let title_end_time = gtk::Label::new(None);
    title_end_time.set_markup("<big>Введите время финиша hh:mm:ss</big>");
    let input_end_time = gtk::Entry::new();
    input_end_time.set_completion(Some(&completion_countries));

    let add = gtk::Button::with_label("Добавить лыжника");

    // Create an EntryCompletion widget

    let row = gtk::Box::new(gtk::Orientation::Vertical, 5);
    row.add(&title_club);
    row.pack_start(&input_club, false, false, 10);

    row.add(&title_surname);
    row.pack_start(&input_surname, false, false, 10);

    row.add(&title_start_time);
    row.pack_start(&input_start_time, false, false, 10);

    row.add(&title_end_time);
    row.pack_start(&input_end_time, false, false, 10);

    row.add(&add);
    row.add(&view);

    // window.add(&win_title);
    window.add(&row);

    // show everything
    window.show_all();
    if let (Some(button_obj), Some(btn_obj)) = (add.accessible(), view.accessible()) {
        button_obj.set_description("Button to increase label value");
        add.connect_clicked(move |_| {
            let mut club: String = input_club.text().to_string();
            let mut surname: String = input_surname.text().to_string();
            let mut start_time: String = input_start_time.text().to_string();
            let mut finish_time: String = input_end_time.text().to_string();

            let start_time: Vec<&str> = start_time.trim().split(':').collect();

            let finish_time: Vec<&str> = finish_time.trim().split(':').collect();

            let start_h: i32 = start_time[0]
                .parse()
                .expect("Не удалось преобразовать строку к числу");
            let start_m: i32 = start_time[1]
                .parse()
                .expect("Не удалось преобразовать строку к числу");
            let start_s: i32 = start_time[2]
                .parse()
                .expect("Не удалось преобразовать строку к числу");
            let start_time: f64 = start_h as f64 * 3600.0 + start_m as f64 * 60.0 + start_s as f64;

            let finish_h: i32 = finish_time[0]
                .parse()
                .expect("Не удалось преобразовать строку к числу");
            let finish_m: i32 = finish_time[1]
                .parse()
                .expect("Не удалось преобразовать строку к числу");
            let finish_s: i32 = finish_time[2]
                .parse()
                .expect("Не удалось преобразовать строку к числу");
            let finish_time: f64 =
                finish_h as f64 * 3600.0 + finish_m as f64 * 60.0 + finish_s as f64;

            let mut new = Participant {
                club: club,
                surname: surname,
                start_time: start_time,
                finish_time: finish_time,
            };
            unsafe {
                participants.push(new);
                write_struct();
            }
        });
        view.connect_clicked(move |_| {
            let window_for: &Window = window.upcast_ref();
            let mut message = String::new();
            unsafe {
                println!("{}", participants.len());
                for i in 0..participants.len() {
                    println!("{}",participants[i].finish_time - participants[i].start_time);
                    if participants[i].finish_time - participants[i].start_time <= 100.0 {
                        message += format!("Клуб: {}\n", participants[i].club).as_str();
                        message += format!("Фамилия: {}\n", participants[i].surname).as_str();
                        message += format!("Время: {}\n", participants[i].finish_time - participants[i].start_time).as_str();
                    }
                }
            }

            show_modal_dialog(&window_for, message.as_str());
        });
    }
}

fn show_modal_dialog(window: &Window, message: &str) {
    let dialog = MessageDialog::new(
        Some(window),
        DialogFlags::MODAL,
        MessageType::Info,
        ButtonsType::Ok,
        message,
    );

    dialog.run();
    unsafe {
        dialog.destroy();
    }
}

// fn read_data() {
//     let mut file = File::create("db.bin").unwrap();
//     let metadata = file.metadata().expect("error read metadata");
//     let len = metadata.len();
//     let mut buffer: Vec<u8> = vec![0, len as u8];
//     file.read(&mut buffer).unwrap();
//     let mut offset = 0;
//     for _i in 0..buffer[offset] {
//         offset += 1;
//         let mut club = String::new();
//         match std::str::from_utf8(&buffer[offset + 1..buffer[offset] as usize]) {
//             Ok(contents) => club = contents.to_string(),
//             Err(error) => println!("Error reading file: {}", error),
//         }
//         offset += buffer[offset] as usize + 1;
//         let surname = String::new();
//         match std::str::from_utf8(&buffer[offset + 1..buffer[offset] as usize]) {
//             Ok(contents) => club = contents.to_string(),
//             Err(error) => println!("Error reading file: {}", error),
//         }
//         offset += buffer[offset] as usize + 1;
//         let start_time: f64 = f64::from_be_bytes(buffer[offset..offset + 8].try_into().unwrap());
//         offset += 8;
//         let finish_time: f64 = f64::from_be_bytes(buffer[offset..offset + 8].try_into().unwrap());
//         unsafe {
//             participants.push(Participant {
//                 club: (club),
//                 surname: (surname),
//                 start_time: (start_time),
//                 finish_time: (finish_time),
//             })
//         }
//     }
// }

fn write_struct() -> std::io::Result<()> {
    let mut file = File::open("db.bin")?;
    unsafe {
        file.write_all(&[participants.len() as u8])?;
        for i in participants.len() - 1..participants.len() {
            let len_club = participants[i].club.len() as u8;
            let club = participants[i].club.as_bytes();
            let len_surname = participants[i].surname.len() as u8;
            let surname = participants[i].surname.as_bytes();
            let start_time = participants[i].start_time as u8;
            let finish_time = participants[i].finish_time as u8;
            file.write_all(&[len_club])?;
            file.write_all(club)?;
            file.write_all(&[len_surname])?;
            file.write_all(surname)?;
            file.write_all(&[start_time])?;
            file.write_all(&[finish_time])?;
        }
    }
    Ok(())
}
