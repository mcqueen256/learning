fn main() {
    {
        let mut _s = String::new();
    }

    {
        let data = "this is some data";
        let _s = data.to_string();
        let _s = "this is another datum".to_string();
    }

    {
        let mut s = String::from("foo");
        s.push_str("bar");
        println!("{}" ,s);
    }

    { // push_str
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 = {}", s2);
    }

    {
        let mut s = String::from("lo");
        s.push('l');
        println!("s = {}", s);
    }

    {
        let s1 =  String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2 + "!!!";
        // println!("s1 = {}", s1); // s1 was moved.
        println!("s2 = {}", s2);
        println!("s3 = {}", s3);
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        // let _s = s1 + "-" + &s2 + "-" + &s3;
        let _s = format!("{}-{}-{}", s1, s2, s3);
    }

    // {
    //     let s = String::from("nicholas");

    //     let c = &s[0];
    // }

    {
        let hello = String::from("Hola");
        println!("len of {} is {}", hello, hello.len());
        let hello = String::from("Здравствуйте");
        println!("len of {} is {}", hello, hello.len());
    }

    {
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        println!("string slice: {}", s);
    }

    {
        for c in "नमस्ते".chars() {
            println!("{} is {} bytes long", c, c.len_utf8());
        }
    }
}