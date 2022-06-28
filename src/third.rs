pub mod third_app {
    use std::collections::hash_map::Entry;
    use std::collections::HashMap;
    use std::io;

    pub fn personnel_accounting() {
        println!("Введите команду (eng):");
        let mut company: HashMap<String, Vec<String>> = HashMap::new();
        const ADD: &str = "add";
        const LIST: &str = "list";

        loop {
            let mut user_command = String::new();

            io::stdin().read_line(&mut user_command).expect("Failed to read line");

            user_command = match user_command.trim().parse() {
                Ok(text) => text,
                Err(_) => {
                    println!("It's not a valid text command!");
                    continue;
                }
            };

            let command_words = user_command.split_whitespace().collect::<Vec<_>>();

            match command_words[0] {
                ADD => {
                    if command_words.len() < 4 || command_words[2] != "to" {
                        println!("некорректно введена команда!");
                        continue;
                    };
                    let department_name = command_words[3].parse().unwrap();
                    let employer_name = command_words[1].parse().unwrap();
                    let department_unit = company.entry(department_name).or_insert(Vec::new());
                    department_unit.push(employer_name);
                    // println!("{company:#?}");
                }
                LIST => {
                    match command_words.len() {
                        1 => {
                            println!("{company:#?}");
                            continue;
                        }
                        2 => {
                            let department_name = command_words[1].parse().unwrap();
                            match company.entry(department_name) {
                                Entry::Occupied(department_unit) => println!("{department_unit:#?}"),
                                _ => {
                                    println!("Такого отдела в компании нет");
                                    continue
                                }
                            }
                        }
                        _ => {
                            println!("Некорректное количество аргументов переданных команде.");
                            continue;
                        }
                    };
                }
                _ => {
                    println!("Нет такой команды!");
                    continue;
                }
            }
            println!("FOR DEBUG");
        }
    }
}