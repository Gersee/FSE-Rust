fn doSomething(x: u32) {
    if x > 0 {
        println!("x is {}", x);
        doSomething(x - 1);
    }
}

fn main() {
    let number: u32 = 10;
    doSomething(number);
}
