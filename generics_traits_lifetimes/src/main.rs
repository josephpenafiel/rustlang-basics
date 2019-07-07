

// // Define generics in structs 

// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T
// } // when making this struct vars x,y must be of the same type 


// #[derive(Debug)]
// struct Pointt<T, U> {
//     x:T,
//     y:U
// } // this struct can mix types

// // generics in method definitions 

// impl <T> Point<T> { // impl <T> specify methods on the type Point<T>
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl <T,U> Pointt<T,U> {
//     fn mixup<V,W>(self, other: Pointt<V,W>) -> Pointt<T, W> {
//         Pointt{
//             x: self.x,
//             y: other.y
//         }
//     }
// }

// #[allow(dead_code)]
// fn main() {
//     let integer = Point {
//         x: 5,
//         y:10
//     };

//     let float = Point {
//         x: 10.0,
//         y: 5.0
//     }; 

//     let multi = Pointt {
//         x: 10.0,
//         y: 10
//     };

//     let multi2 = Pointt {
//         x: 5,
//         y: 10.0
//     };

//     println!("{:?} {:?} {:?} {:?}", integer, float, multi, multi2);
// }


// TRAITS 

/*
A trait tells the Rust compiler about functionality a particular type has and can share with other types. 
We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a 
generic can be any type that has certain behavior.
*/

trait Summary {

    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(read mode from {}", self.summarize_author()) // default implementation 
                                                // can take methods from the same trait 
    }
}

struct NewsArticle {
    headline: String,
    location: String, 
    author: String, 
    content: String
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String{
        format!("{}", self.author)
    }
} 


struct Tweet {
    username: String,
    content: String, 
    reply: bool, 
    retweet: bool
}

impl Summary for Tweet {

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


#[allow(dead_code)]
fn main() {
    let tweet = Tweet {
        username: "firslast".to_string(),
        content: "iasndoiasdn".to_string(),
        reply: false,
        retweet: false
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };

    println!("1 new tweet: {}", tweet.summarize() );
    println!("New article available! {}", article.summarize());


}


// TRAITS AS PARAMETERS // 

fn notify(item: impl Summary) { // this function accepts any type that has the Summary trait
    println!("Breaking News! {}", item.summarize());
}

// fn some_function<T: Display + Clone, U: Clone + Debug>(t:T, u: U) -> i32 {

// } 

// // same as above 

// fn some_function<T,U>(t:T, u:U) -> i32 
//     where T: Display + Clone, 
//           U: Clone + Debug{}


// RETURNING TYPES THAT IMPLEMENT TRAITS 

fn returns_summarizable() -> impl Summary { // function returns some type that implemets the Summary trait
    Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}