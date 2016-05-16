use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

fn main() {
    //Create vector with Mutex and Arc to organize concurrency access
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    //open channel to synchronize / communicate
    let (tx, rx) = mpsc::channel();

    //Open 3 threads
    for i in 0..3 {
        //tupel for data and transmitter
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            //Do the things many times
            for k in 0..10000 as i32 {
                //Access mutex an unwrap result
                let mut data = data.lock().unwrap();
                //Increment data-index
                data[i] += 1;
                //print only every 1000 times something out
                if k.checked_rem(1000).unwrap() == 0 {
                    println!("1000er run: Data[{}] = {}", i, data[i]);
                }
            }

            //send result - need to get access first, but without mutable
            let data = data.lock().unwrap();
            tx.send(data[i]);
        });
    }

    for _ in 0..3 {
        //Wait for results and print message
        println!("Got result from thread: {}", rx.recv().unwrap());
    }
    println!("End application");
}
