fn main() {
    let x = 5;
    println!("Outer scope: {}", x); // x=5
    {
        let x = x + 2;
        println!("Middle scope: {}", x); // x=7
        {
            let x = x * 2;
            println!("Inner scope: {}", x); // x=14
        }
        println!("Middle scope: {}", x); // x=7
    }
    println!("Outer scope: {}", x); // x=5
}
