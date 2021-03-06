#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {
    env_logger::init().unwrap();

    info!("starting up");
    warn!("warning");
    error!("error");

    println!("println: Hello, world!");

    trace!("ending");
}
