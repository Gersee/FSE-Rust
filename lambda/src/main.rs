fn ten_times<F>(f: F) where F: Fn(i32) {
    for index in 0..10 {
        f(index);
    }
}

fn main() {
    ten_times(|j| println!("run #{}", j));

    //Another example
    fn  function  (i: i32) -> i32 { i + 1 }

    let i = 1;
    //i wird nicht verändert, ist zum Einen nicht mutable, zum Anderen wird nur der Wert an die Funktion übergeben
    println!("anonymious function: {}", function(i));
    println!("anonymious function: {}", function(i));
    println!("anonymious function: {}", function(i));
    println!("anonymious function: {}", function(i));
    println!("anonymious function: {}", function(i));
}
