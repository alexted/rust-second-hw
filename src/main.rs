use std::io;
use crate::first::first_app;
use crate::second::second_app;
use crate::third::third_app;

mod first;
mod second;
mod third;

fn main() {
    // Задачи задания №2
    loop {
        println!(
            "
Выберите программу:
    1. Генерируется список целых чисел - рассчитывается медиана и мода этого списка.
    2. Преобразование строк в поросячью латынь.
    3. Программа для кадрового учёта на предприятии.
Input the number:"
        );

        let mut app_num = String::new();

        io::stdin().read_line(&mut app_num).expect("Failed to read line");

        let app_num = match app_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("It's not a valid integer!");
                continue;
            }
        };

        match app_num {
            1 => first_app::get_median_and_mode(),
            2 => second_app::convert_to_pig_latin(),
            3 => third_app::personnel_accounting(),
            _ => println!("It's not a valid integer!"),
        }
    }
}
