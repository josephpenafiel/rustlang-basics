/*
the type HashMap<K, V> stores a mapping of keys of type K to values of type V/ 
*/

use::std::collections::HashMap;

fn main() {
    println!("Hello, Hash Maps!");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); // asociate key Blue (String) with value 10 (int)
    scores.insert(String::from("Yellow"), 50);

    /* Hash maps store their data on the heap */

    // another way of constructing a hash map // 

    let teams = vec![String::from("blue"), String::from("yellow")]; // create a vector with two strings
    let initial_scores =  vec![10, 50]; 

    // using <_, _> rust infers the types of the hash map based on the type of the vectors
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // the zip method creates a vector of tuples where blue is paired with 10 and so on


    /* OWNERSHIP */
    /*
    For types that implement the Copy trait, like i32, the values are copied into the hash map. 
    For owned values like String, the values will be moved to the hash map and so the ownership.
    */

    let field_name = String::from("favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value); // field_name and field_value moved into map 

    // println!("{}, {}", field_name, field_value) // won't work

    // ACCESSING VALUES IN A HASHMAP // 

    let team_name = "blue".to_string();

    let score = scores.get(&team_name); // score = 10

     // .get method returns an Option<&V> in this case returns Some(&10)

    match score {
        Some(v) => println!("blue score is {}", v),
        None => println!("there's no team with that name")
    }

    // Iterate through HashMaps 

    for (key, value) in &scores {
        println!("{}'s score is: {}", key, value );
    }


    // UPDATING A HASH MAP //

    // OverWrite

    // this scores variable was created using vectors. Because of this we need to update the Hashmap using
    // references as arguments

    scores.insert(&team_name, &25); // blue is now 25
    println!("{:?}", scores);

    // new key value 

    let green = "green".to_string();

    scores.insert(&green, &30);
    println!("{:?}", scores);


    // insert a value only if the key has no value 

    // this scores variable has no association with a vector. We can update without using references

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10); // blue: 10
    scores.insert("blue".to_string(), 20); // blue: 20 

    scores.entry("yellow".to_string()).or_insert(50); // will insert yellow with value 50
    scores.entry("blue".to_string()).or_insert(25); // will not update. blue exists

    println!("{:?}", scores);


    // Updating a Value based on the Old Value 

    /*
    Another common use case for hash maps is to look up a key’s value and then update it based on the old value. 
    For instance, Listing 8-26 shows code that counts how many times each word appears in some text. We use a hash 
    map with the words as keys and increment the value to keep track of how many times we’ve seen that word. If 
    it’s the first time we’ve seen a word, we’ll first insert the value 0.
    */

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        println!("word: {}", word);
        let count = map.entry(word).or_insert(0); // count has a reference returned by or_insert() to the value of the key (&mut V)
        *count += 1; // dereference the variable
        println!("count :{}", count);
    }

    println!("{:?}", map);

   

}
