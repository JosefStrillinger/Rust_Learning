fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    let x = 5;
    let x = plus_one(x);
    
    println!("The value of x is {x}");

    println!("The value of y is: {y}");

    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {//how to do return
    x + 1
}

