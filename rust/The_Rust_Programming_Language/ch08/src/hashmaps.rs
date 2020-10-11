use std::collections::HashMap;

fn main() {
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        println!("Scores: {:?}", scores);
    }

    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];
        let mut _scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    }

    {
        let property= String::from("Color");
        let value = String::from("Yellow");
        let mut m = HashMap::new();
        m.insert(property, value);
        // property and value are not valid past this point as m owns them.
    }

    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        let team = String::from("Blue");
        let score = scores.get(&team);
        if let Some(&score) = score {
            println!("Score for blue is: {}", score);
        }
    }

    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        
        for (team, score) in &scores {
            println!("team {}, score {}", team, score);
        }
    }

    // updating the HashMap

    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        println!("{:?}", scores);
    }

    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
    }

    {
        let text = "hello nic wonderful nic";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}