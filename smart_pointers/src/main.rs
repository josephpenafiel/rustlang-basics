#![allow(dead_code, unused_variables)]

/*
There are three main types of smart pointers: 
Box<T> for allocating values on the heap 
Rc<T> is a reference counting type that enables multiple ownership 
Ref<T> and RefMut<T>, accessed through RefCell<T> is a type that enforces borrowing rules at runtime. 

Many libraries have their own smart pointers, and there can be our own custom implementations as well. 

*/

// ########################################################################
// ############################### BOX ####################################
// ########################################################################

/*
at compile time rust needs to know how much space a type takes up. When this is uncertain a Box<T> is usefull
*/

// Recursive type 

// use crate::List::{Cons, Nil};

// enum List {
//     Cons(i32, List),
//     Nil
// }

// the enum above won't compile because it holds itself. Nil indicates the end of the list but Rust will not know
// how much space to allocate for it resulting in an infinite size. 

// the following code will compile 
// enum List {
//     Cons(i32, Box<List>),
//     Nil
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let list = Cons(1,
//         Box::new(Cons(2,
//             Box::new(Cons(3,
//                 Box::new(Nil))))) );
// }

// =========================================================
// ===== // SMART POINTERS IMPLEMENT THE DEREF TRAIT =======
// =========================================================
/*
let x = 5;
let y = &x; 

*y this is how the value of y is dereferenced using the dereference operator *

Box<T> can be treated as a normal pointer 

*/

// fn main() {
//     let x = 5; 
//     let y = Box::new(5);

//     println!("{}",*y); // prints 5
// }

// =========================================================
// =========== DEFINING OUR OWN SMART POINTER ==============
// =========================================================

// use std::ops::Deref;

// struct MyBox<T>(T); 

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T; 

//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

// fn main() {
//     let x = 10; 
//     let y = MyBox::new(x);

//     println!("{}", *y); // prints 5

//     // behind the scenes rust replaces the *y syntax with *(y.deref()) in other words *y call deref() for us
// }

// =========================================================
// =================== THE DROP TRAIT ======================
// =========================================================

/*
this trait lets us customize what happens when a value is about to go out of scope. 
*/
// struct CustomSmartPointer {
//     data: String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) { // this method can't be called directly 
//         println!("Dropping CustomSmartPointer with data `{}`!", self.data);
//     }
// }


// fn main() {
//     let c = CustomSmartPointer { data: String::from("my stuff") };
//     // let d = CustomSmartPointer { data: String::from("other stuff") };
//     println!("CustomSmartPointers created.");

//    // ======== dropping a value before it goes out of scope ===== // 
   
//    std::mem::drop(c); // drop value earlies 
//    println!("CustomSmartPointer dropped before the end of main.");
// }


// ########################################################################
// ############## Rc<T> the Referece Counted Smart Pointer ################
// ######################################################################## 

/*
Rc stands for reference counting. It allows for multiple ownership by keeping track of the number
of references to a value which determines whether or not a value is still in use. 
*/

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

/*
Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only. 
If Rc<T> allowed you to have multiple mutable references too, you might violate one of the borrowing rules: 
-- multiple mutable borrows to the same place can cause data races and inconsistencies.
*/
