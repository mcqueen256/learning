fn main() {
    {
        let s = String::from("Hello. How are you?");
        println!("{}", first_word(&s));
        println!("{}", first_word("Why hello there."));
    }

    {
        let s = String::from("hello world");
        let _hello = &s[0..5];
        let _world = &s[6..11];
    }

    {
        let s = String::from("hello");
        let _slice = &s[0..2];
        let _slice = &s[..2];
    }

    {
        let s = String::from("hello");
        let len = s.len();
        let _slice = &s[3..len];
        let _slice = &s[3..len];
    }

    {
        let s = String::from("hello");
        let len = s.len();
        let _slice = &s[0..len]; // take whole slice
        let _slice = &s[..]; // take whole slice
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}