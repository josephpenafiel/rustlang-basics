fn main() {
    println!("Hello, world!");
    let mut s = String::new(); // create empty string
    let data = "initial contents"; // create a string literal
    let s = data.to_string();

    let s = "initial contents".to_string(); // also valid
    let s = String::from("initial contents"); // same


    let hello = String::from("السلام عليكم");
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

    let mut s = String::from("foo");
    s.push_str("bar"); // s is now foobar

    let s2 = "foobar";
    s.push_str(s2);

    println!("s says {}", s);
    
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 no longer valid

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("s says {}", s);

    // can't do this 
    // let h = s[3]; 

    // ITERATING STRINGS 

    for c in "नमस्ते".chars() {
    println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
    println!("{}", b);
}   


}
