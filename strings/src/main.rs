fn main() {
    // These two variants re exactly the same
    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    println!("{}", s);

    let s = String::from("initial contents");

    println!("{}", s);

    // All valid

    /* let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    println!("{}", hello); */


    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    //let s3 = format!("{}{}", s1, s2); // both work
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);

    for i in s.chars() {
        println!("{}", i);
    }



}
