fn do_something(x: u32) {
    if x > 0 {
        println!("x is {}", x);
        do_something(x - 1);
    }
}

fn main() {
    let number: u32 = 10;
    do_something(number);
}
