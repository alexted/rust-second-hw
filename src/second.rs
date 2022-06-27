
pub mod second_app {
    use std::io;

    pub fn convert_to_pig_latin() {
        println!("Введите фразу (eng):");
        loop {
            let mut user_phraze = String::new();

            io::stdin().read_line(&mut user_phraze).expect("Failed to read line");

            user_phraze = match user_phraze.trim().parse() {
                Ok(text) => text,
                Err(_) => {
                    println!("It's not a valid text!");
                    continue;
                }
            };

            let words = user_phraze.split_whitespace();

            let vowels = ["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"];
            let mut encoded_phraze = String::new();
            let encrypted_word = String::new();
            for w in words {
                let first_letter = &w[0..1];
                if vowels.contains(&first_letter) {
                    let encrypted_word = format!("{w}-hay");
                } else {
                    let ew = &w[1..];
                    let encrypted_word = format!("{ew}-{first_letter}ay");
                }
                encoded_phraze.push_str(&format!(" {encrypted_word}"));
            }
            println!("{encoded_phraze}");
        }
    }
}