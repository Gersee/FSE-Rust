use std::thread;
use std::time::Duration;

fn main() {
	
	let mut threadNumber: f64 = 4.0;
	let mut subtotals: f64 = 1000.0;
	let mut intervall: f64 = subtotals / threadNumber;
	let mut result: f64 = 0.0;
	
	let mut children = vec![];
	
	for j in 0..4 {
		let mut start: i32 = 1 + intervall as i32 * j;
		let mut end: i32 = (j + 1) * intervall as i32;
		children.push(thread::spawn(move || {

			calc(start, end, j, subtotals);
		}));
	}
		
	for child in children {
    	// Wait for the thread to finish. Returns a result.
        let result = child.join();
    }
}

fn calc(start: i32, end: i32, j: i32, subtotals: f64) {
	println!("Thread: {} started!", j);
	println!("start = {}", start);
	println!("end = {}", end);
	println!("subtotals = {}", subtotals);
		
	let dx: f64 = 1 as f64 / subtotals;
	let mut sum: f64 = 0.0;
	let mut x: f64 = 0.0;
	
	for i in start..end as i32 {
		x = dx * (start as f64 - 0.5 as f64);
		sum = sum + (1 as f64 / (1 as f64 + x * x));
		//println!("sum: {}", sum);
	}
	let pi: f64 = dx * sum;
	
	println!("Pi: {}", pi);
	println!("Thread: {} finished!", j);
	println!("----------------------------------------------------------");
}
