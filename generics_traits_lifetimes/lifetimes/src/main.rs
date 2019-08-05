#![allow(unused_variables)]
#![allow(dead_code)]
use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}


fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("{}", i.part);
    println!("{}", first_sentence);
    println!("{}", novel);

}
