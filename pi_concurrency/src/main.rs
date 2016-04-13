use std::thread;

/// Calculate Pi with threads
///
/// PVR Übung 1 Aufgabe 2
fn main() {
	
	// Number of threads for the calculation
	let thread_number: f64 = 10.0;
	// Number of subtotals for the calculation
	let subtotals: f64 = 1000000.0;
	// Size of each intervall
	let intervall: f64 = subtotals / thread_number;
	// Vector for the threads
	let mut children = vec![];
	
	// Loop to create and implement threads
	for j in 0..thread_number as i32 {
		
		// Calculate the start of the range to calculate
		let start: i32 = 1 + intervall as i32 * j;
		// Calculate the end of the range to calculate
		let end: i32 = (j + 1) * intervall as i32;
		
		// Create a thread and push it in the vector
		children.push(thread::spawn(move || {
			// Call the calculation with specific parameters		
			return calc(start, end, j, subtotals);	
		}));
		
		println!("Thread {}: Thread created!", j);	
	}
	
	// Result of the whole calculation	
	let mut pi: f64 = 0.0;
	
	// Wait for the child thread to finish and extract its result
	for child in children {
    	// Result of the thread
        let sub_result: f64 = child.join().unwrap();
        // Add thread result to the complete result
        pi = pi + sub_result;
    }
	
	println!("---------------------------------");
	println!("Pi: {}", pi);
}

/// Function to calculate pi for a given range
///
/// Parameters: start of the range, end of the range,
/// number of the thread (j), subtotals
///
fn calc(start: i32, end: i32, j: i32, subtotals: f64) -> f64 {
	
	println!("Thread {}: Started calculation!", j);	
	
	let dx: f64 = 1 as f64 / subtotals;
	let mut sum: f64 = 0.0;
	
	for i in start..end as i32 {
		let x: f64 = dx * (i as f64 - 0.5 as f64);
		sum = sum + (4 as f64 / (1 as f64 + x * x));
	}
	
	let sub_result: f64 = dx * sum;
		
	println!("Thread {}: Subresult: {}", j, sub_result);
	println!("Thread {}: Finished calculation!", j);
	return sub_result;
}
