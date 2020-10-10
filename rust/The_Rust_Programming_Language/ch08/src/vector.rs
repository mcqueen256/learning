fn main() {
    {
        let _v: Vec<i32> = Vec::new();
        let _v2 = vec![1u32,2,3];
        let _v3:Vec<i32> = vec![];
    }

    {
        let mut v  = Vec::new();
        v.push(0);
        v.push(1);
        v.push(2);
        v.push(10);
    }

    {
        let v = vec![10, 20, 30, 40, 50];
        let third = &v[2];
        println!("the third element is {}", third);

        match v.get(2) {
            Some(third) => println!("the third element is {}.", third),
            None => println!("There is no third element."),
        }
    }

    // {
    //     let v = vec![1,2,3,4,5];

    //     let does_not_exist = &v[100];
    //     let does_not_exist = v.get(100);
    // }

    {
        let v = vec![100,200,300,400];
        for i in &v  {
            println!("{}", i);
        }

        let mut v = vec![-1,-2,-3,-4,-5];
        for i in &mut v { // borrow mut v
            *i *= 50;
        }
        for i in v { // move v
            println!("{}", i);
        }
    }

    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(43),
            SpreadsheetCell::Text(String::from("I am a cell of hanes text")),
            SpreadsheetCell::Float(10.42),
        ];
    }
}
