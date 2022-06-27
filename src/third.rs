pub mod third_app {
    use std::collections::HashMap;
    use std::io;

    pub fn personnel_accounting() {
        println!("Введите команду (eng):");
        let company: HashMap<String, Vec<String>> = HashMap::new();
        const ADD: &str = "add";
        const LIST: &str = "list";

        loop {
            let mut user_command = String::new();

            io::stdin().read_line(&mut user_command).expect("Failed to read line");

            user_command = match user_command.trim().parse() {
                Ok(text) => text,
                Err(_) => {
                    println!("It's not a valid command!");
                    continue;
                }
            };

            let command_words = user_command.split_whitespace().collect::<Vec<_>>();
            let verb = command_words[0].to_lowercase();
            println!("DUBUG");
        }
    }
}