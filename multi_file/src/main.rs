mod myfile;
mod subfolder;

fn main() {
    println!("Start main method");

    //Access function defined in myfile.rs
    myfile::my_function();

    //Access function defined in ./subfolder/mod.rs
    subfolder::my_function();
}
