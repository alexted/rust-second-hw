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

            let mut encoded_phraze = String::new();
            for (i, word) in user_phraze.split_whitespace().enumerate() {
                let encrypted_word = encode(word);
                if i != 0 {
                    encoded_phraze += " ";
                }
                encoded_phraze.push_str(&encrypted_word);
            }
            println!("{encoded_phraze}");
        }
    }

    fn encode(word: &str) -> String {
        let eng_vowels = ["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"];
        let first_letter = &word[0..1];
        let encrypted_word;

        if eng_vowels.contains(&first_letter) {
            encrypted_word = format!("{word}-hay");
        } else {
            let word_slice = &word[1..];
            encrypted_word = format!("{word_slice}-{first_letter}ay");
        };
        encrypted_word
    }
}