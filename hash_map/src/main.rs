use std::collections::HashMap;

fn main() {
    let names = ["Detlef", "Dieter", "Herbert", "Horst", "Sabine"];
    let mut salery_map = HashMap::new();

    //fill Hash Map
    salery_map.insert("Horst", "$ 40.000");
    salery_map.insert("Dieter", "$ 28.000");
    salery_map.insert("Detlef", "$ 33.000");
    salery_map.insert("Herbert", "$ 45.000");

    //Transform array items into Iterator
    let mut array_iter = names.into_iter();

    //Loop to get all elements of the array
    loop {
        //Check if there is another element
        match array_iter.next() {
            //Element is there, do something
            Some(x) => {
                // Ask HashMap and get Option-Return-Enum
                match salery_map.get(x) {
                    Some(&salery) => println!("Salary of {} is: {}", x, salery),
                    _ => println!("Person ({}) is not in list.", x),
                }},
            //No elements any more
            None => break,
        }
    }
}
