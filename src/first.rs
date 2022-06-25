pub mod first_app {
    use std::collections::HashMap;
    use std::io;

    pub fn get_median_and_mode() {
        println!("Задайте длину генерируемого списка чисел:");

        loop {
            let mut lenght = String::new();

            io::stdin().read_line(&mut lenght).expect("Failed to read line");

            let lenght: u64 = match lenght.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("It is not an integer!");
                    continue;
                }
            };

            let mut vector_of_numbers = (0u64..lenght).collect::<Vec<_>>();
            vector_of_numbers.sort();

            let mut median;
            if vector_of_numbers.len() % 2 == 0 {
                median = vector_of_numbers[vector_of_numbers.len() / 2];
            } else {
                median = vector_of_numbers[vector_of_numbers.len() / 2 + 1];
            }

            let mut nums_map = HashMap::new();

            for number in vector_of_numbers {
                let count = nums_map.entry(number).or_insert(0);
                *count += 1;
            }

            let mode = nums_map
                .into_iter()
                .max_by_key(|&(_, count)| count)
                .map(|(val, _)| val)
                .expect("Cannot compute the mode of zero numbers");

            println!("median is {median}, mode is {mode}");
        }
    }
}