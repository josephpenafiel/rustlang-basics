// Closures //
/*
They're anonymous functions that can be saved in a variable, or pass as arguments
to other functions. They can be created in one place and then be called in a
different context. Closures can capture values from the cope in which they're defined.
*/

use std::{collections::HashMap, thread, time::Duration};

// fn expensive_calculation_sim(intensity: u32) -> u32 {
//     println!("Calculating...(slow)");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

/* NORMAL */
// fn generate_workout(intensity: u32, rand_val: u32){
//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             expensive_calculation_sim(intensity)
//         );
//         println!(
//             "Next, do {} situps!",
//             expensive_calculation_sim(intensity)
//         );
//     } else {
//         if rand_val == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 expensive_calculation_sim(intensity)
//             );
//         }
//     }
// }

/* USING FUNCTIONS */
// fn generate_workout(intensity: u32, rand_val: u32){
//     let expensive_result = expensive_calculation_sim(intensity);
//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             expensive_result
//         );
//         println!(
//             "Next, do {} situps!",
//             expensive_result
//         );
//     } else {
//         if rand_val == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 expensive_result
//             );
//         }
//     }
// }

/* USING CLOSURES */
// fn generate_workout(intensity: u32, rand_val: u32) {
// //define the closure
//     let expensive_closure = |num| {
//         println!("calculating... (closure)");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };
//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             expensive_closure(intensity)
//         );
//         println!(
//             "Next, do {} situps!",
//             expensive_closure(intensity)
//         );
//     } else {
//         if rand_val == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 expensive_closure(intensity)
//             );
//         }
//     }

// }

/* OPTIMIZED CLOSURE */

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // T is a closure trait
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn get_or_set_value(&mut self, arg: u32) -> u32 {

        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let val = (self.calculation)(arg);
                self.value.insert(arg, val);
                val
            }
        }

    }
}


fn generate_workout(intensity: u32, rand_val: u32) {
    //define the closure
    let mut expensive_cacher = Cacher::new(|num| {
        println!("calculating... (cacher)");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_cacher.get_or_set_value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_cacher.get_or_set_value(intensity)
        );
    } else {
        if rand_val == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_cacher.get_or_set_value(intensity)
            );
        }
    }

}

fn main() {
    let sim_user_val = 10;
    let sim_rand_val = 7;

    // generate_workout(
    //     sim_user_val,
    //     sim_rand_val
    // );

    // CAPTURING THE ENVIRONMENT WITH CLOSURES //
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;

    assert!(equal_to_x(4));

    /*
        fn equal_to_x(z: i32) -> bool { z == x} // calling this function won't work since x is not in that scope

        Closures can capture values from their environment in three ways, which directly map to the three ways a
        function can take a parameter: taking ownership, borrowing mutably, and borrowing immutably. These are
        encoded in the three Fn traits as follows:

        * FnOnce consumes the variables it captures from its enclosing scope, known as the closure’s environment.
        To consume the captured variables, the closure must take ownership of these variables and move them into
        the closure when it is defined. The Once part of the name represents the fact that the closure can’t take
        ownership of the same variables more than once, so it can be called only once.

        * FnMut can change the environment because it mutably borrows values.

        * Fn borrows values from the environment immutably.

        Rust infers which trait to use based on how the closure uses the values from the environment.

        If you want to force the closure to take ownership of the values it uses in the environment, you can use
        the move keyword before the parameter list.

        fn main() {
        let x = vec![1, 2, 3];

        let equal_to_x = move |z| z == x;

        println!("can't use x here: {:?}", x);

        let y = vec![1, 2, 3];

        assert!(equal_to_x(y));
    }

        */

    // PROCESSING A SERIES OF ITEMS WITH ITERATORS //

    /*
    The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible
    for the logic of iterating over each item and determining when the sequence has finished. When you use iterators,
    you don’t have to reimplement that logic yourself.
    */

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter(); // create an iterator

    for val in v1_iter {
        println!("got: {:?}", val);
    }

    /*
    iterators implement a trait name Iterator that is defined in the standard library.
    The Iterator trait requires implementors to define the method next.
    */

    /*
    Other methods defined on the Iterator trait, known as iterator adaptors, allow you to change iterators
    into different kinds of iterators. You can chain multiple calls to iterator adaptors to perform complex
    actions in a readable way. But because all iterators are lazy, you have to call one of the consuming
    adaptor methods to get results from calls to iterator adaptors.
    The collect method consumes the iterator and collects the resulting value into a collection
    data type.
    */

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}


#[test]
fn call() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.get_or_set_value(1);
    assert_eq!(v1, 1);
    let v2 = c.get_or_set_value(2);
    assert_eq!(v2, 2);
}

#[test]
fn iter_demo() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum(); // can't use v1_iter since sum() took ownership

    assert_eq!(total, 6);
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: "sneaker".to_string(),
        },
        Shoe {
            size: 13,
            style: "sandal".to_string(),
        },
        Shoe {
            size: 10,
            style: "boot".to_string(),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: "sneaker".to_string()
            },
            Shoe {
                size: 10,
                style: "boot".to_string()
            }
        ]
    )
}
