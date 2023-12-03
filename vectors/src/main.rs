fn main() {
    let mut v = Vec::new();// create new vector
    let _v2 = vec![1, 2, 3]; // create new vector with initial values

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2]; // panics if index is out of bounds
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2); // returns None if index is out of bounds
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    //let _does_not_exist = &v[100];// panics
    //let _does_not_exist = v.get(100);// returns None

    let first = &v[0];

    //v.push(6); // does not work, because of the ownership rules of rust

    println!("The first element is: {first}");

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

    // You can use Enums to store different types in a vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
