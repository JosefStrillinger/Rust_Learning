use std::io;

fn main() {
    let mut temp = String::new();
    io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
    let temp: u32 = temp.trim().parse().expect("Failed to parse");

    let new_temp:f32 = (temp * 9/5 + 32) as f32;

    println!("Temperature in Fahrenheit: {new_temp}");
}
