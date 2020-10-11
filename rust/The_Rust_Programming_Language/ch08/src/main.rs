use std::collections::HashMap;


fn exersice1() {
    let numbers = vec![32, 42, 100, 90, 7, 10, 15, 76, 53, 32, 31, 20, 49];

    let average = {
        let sum: i32 = numbers.iter().sum();
        if numbers.len() > 0 {
            Some(f64::from(sum) / numbers.len() as f64)
        } else {
            None
        }
    };

    let medium = {
        let mut sorted = numbers.clone();
        sorted.sort();
        let medium = sorted.get(sorted.len() / 2);
        if let Some(&number) = medium {
            Some(number)
        } else {
            None
        }
    };

    let mode = {
        let mut mode_map = HashMap::new();
        for &number in &numbers {
            let count = mode_map.entry(number).or_insert(0);
            *count += 1;
        }

        if numbers.len() == 0 {
            None
        } else {
            let max = mode_map.into_iter().max_by(|x, y| x.1.cmp(&y.1));
            if let Some((mode_num, _mode_count)) = max {
                Some(mode_num)
            } else {
                None
            }
        }
    };

    println!("avg: {:?}, med: {:?}, mod: {:?}", average, medium, mode);
}

fn starts_with_consonant(string: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let first_char = string.chars().next();
    if let Some(first_char) = first_char {
        vowels.contains(&first_char)
    } else {
        false
    }
}

fn exersice2() {
    let text = "This could get quite interesting if I don't say so myself";
    

    let mut s = String::new();
    for word in text.split_whitespace() {
        let lower_case_word = word.to_lowercase();
        let new_word = if starts_with_consonant(&lower_case_word) {
            let mut new_word = String::from(&word[..]);
            new_word.push_str("-hay");
            new_word
        } else {
            let mut new_word = String::from(&word[1..]);
            new_word.push_str(&format!("-{}ay", &word[0..1]));
            new_word
        };
        s.push_str(&format!("{} ", new_word));
    }

    println!("{}", s);
}

fn exersice3() {
    
}

fn main() {
    exersice1();
    exersice2();
}