use std::collections::HashMap;

fn main() {
    let names = ["Detlef", "Dieter", "Herbert", "Horst", "Sabine"];
    let mut salery_map = HashMap::new();

    salery_map.insert("Horst", "$ 40.000");
    salery_map.insert("Dieter", "$ 28.000");
    salery_map.insert("Detlef", "$ 33.000");
    salery_map.insert("Herbert", "$ 45.000");


    let mut array_iter = names.into_iter();

    // Ask HashMap and get Option-Return-Enum
    loop {
        match array_iter.next() {
            Some(x) => {
                match salery_map.get(x) {
                    Some(&salery) => println!("Salary of {} is: {}", x, salery),
                    _ => println!("Person ({}) is not in list.", x),
                }},
            None => break,
        }
    }
}
