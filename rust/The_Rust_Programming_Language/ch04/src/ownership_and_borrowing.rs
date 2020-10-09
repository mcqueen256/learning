fn main() {
    
    
    {
        let s1 = String::from("hello");
        // {
        //     let mut s2 = s1;
        //     s2.push_str(", world!");
        // }

        // println!("{}", s1);

        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    {
        let a = [1,2,3,4,5];
        let b= a;
        print_arr("b", b);
        print_arr("a", a);
    }

    {
        let a = vec![1,2,3,4,5];
        let b = &a;
        let c = &a;
        print_vec("c", &c);
        print_vec("b", &b);
        print_vec("a", &a);
    }

    // {
    //     let mut s = String::from("hello");
    //     let r1 = &mut s;
    //     let r2 = &mut s;
    //     r2.push_str(", world!");
    //     println!("{} {}", r1, r2);
    // }

    // {
    //     let mut s = String::from("hello");
    //     let r1 = &s; // no problemo;
    //     let r2 = &s; // easy as
    //     let r3 = & mut s; // AHhhhh! PROBLEM!
    //     r3.push_str(", world!");
    //     println!("{}", r1);
    // }
}


fn print_arr(name: &str, arr: [u32; 5]) {
    let arr = arr.iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join(", ");
    println!("{}: [ {} ]", name, arr);
}


fn print_vec(name: &str, arr: &Vec<u32>) {
    let arr = arr.iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join(", ");
    println!("{}: [ {} ]", name, arr);
}