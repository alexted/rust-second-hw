pub mod second_app {
    use std::io;

    pub fn convert_to_pig_latin() {
        println!("Введите фразу (eng):");
        loop {
            let mut phraze = String::new();

            io::stdin().read_line(&mut phraze).expect("Failed to read line");

            let phraze: String = match phraze.trim().parse() {
                Ok(text) => text,
                Err(_) => {
                    println!("It's not a valid text!");
                    continue;
                }
            };
            println!("input text is {phraze}");
        }
    }
}