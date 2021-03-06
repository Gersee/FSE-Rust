extern crate rand;
extern crate time;
use rand::Rng;
use time::PreciseTime;
use std::f32;

// Define struct for employees

#[derive(Copy, Clone)]
struct Employee {
	employee_number: u32,
	salary: u32,
}

trait HasDoSomething {
    fn do_something(&self) -> u32;
}
impl HasDoSomething for Employee {
    fn do_something(&self) -> u32 {
        (((((self.salary * std::f32::consts::PI as u32) * std::f32::consts::PI as u32) as f32).sqrt() as f32).ln() as f32).exp() as u32
    }
}

fn main() {

	// Create vector
	let mut in_employees = vec![];

	for x in 0..50000 {
		let rand_salary = rand::thread_rng().gen_range(1, 9000000);
		in_employees.push(Employee {employee_number: x, salary: rand_salary});
	}

	println!("Compare of Schwartzian Transoformation and Rust Sort by Attribut and by Method (with calculation)");
	println!("");

	// Print vector of employees
	//for x in 0..in_employees.len() {
	//	println!("{0},{1}", in_employees[x].employee_number, in_employees[x].salary);
	//}


	// Start time of Schwartzian Transformation
	let start_schwartzian = PreciseTime::now();

	let out_schwartzian = schwartzian_transformation(in_employees.clone());

	// End time of Schwartzian Transoformation
	let end_schwartzian = PreciseTime::now();

	println!("Attribut: Runtime Schwartzian Transformation (Complete): {}", start_schwartzian.to(end_schwartzian));

	// Start time of Rust Sort
	let start_sort = PreciseTime::now();

	let out_sort = sort(in_employees.clone());

	// End time of Rust Sort
	let end_sort = PreciseTime::now();

	println!("Attribut: Runtime Rust Sort: {}", start_sort.to(end_sort));

	//println!("");
	//println!("Result of Schwartzian Transoformation");
	//println!("");

	//for x in 0..out_schwartzian.len() {
	//	println!("{0},{1}", out_schwartzian[x].employee_number, out_schwartzian[x].salary);
	//}

	//println!("");
	//println!("Result of Rust Sort");
	//println!("");

	//for x in 0..out_sort.len() {
	//	println!("{0},{1}", out_sort[x].employee_number, out_sort[x].salary);
	//}

	// Start time of Schwartzian Transformation
	let method_start_schwartzian = PreciseTime::now();

	let method_out_schwartzian = method_schwartzian_transformation(in_employees.clone());

	// End time of Schwartzian Transoformation
	let method_end_schwartzian = PreciseTime::now();

	println!("Method: Runtime Schwartzian Transformation (Complete): {}", method_start_schwartzian.to(method_end_schwartzian));

	// Start time of Rust Sort
	let method_start_sort = PreciseTime::now();

	let method_out_sort = method_sort(in_employees.clone());

	// End time of Rust Sort
	let method_end_sort = PreciseTime::now();

	println!("Method: Runtime Rust Sort: {}", method_start_sort.to(method_end_sort));

}


fn sort(in_employees: Vec<Employee>) -> Vec<Employee> {

	// Sort vector by key
	// |k| k.employee_number --> sort by employee_number
	// |k| k.salary --> sort by salary
	let mut out_employees = in_employees.clone();
	out_employees.sort_by_key(|k| k.salary);
	return out_employees;

}


fn schwartzian_transformation(in_employees: Vec<Employee>) -> Vec<Employee> {

	let start_copy_to_tuple = PreciseTime::now();
	// Create temporary vector for transformation
	let mut temp_employees = vec![];

	// Create Tuples with Employee and sort attribute (salary)
	for x in 0..in_employees.len() {
		let tuple = (in_employees[x].clone(), in_employees[x].salary);
		temp_employees.push(tuple);
	}

	let end_copy_to_tuple = PreciseTime::now();
	println!("Attribut: Runtime Schwartzian Transformation (Copy-to-tuple): {}", start_copy_to_tuple.to(end_copy_to_tuple));
	// Start time of innersort
	let start_innersort = PreciseTime::now();

	temp_employees.sort_by_key(|k| k.1);

	// End time of innersort
	let end_innersort = PreciseTime::now();

	println!("Attribut: Runtime Schwartzian Transformation (Inner-Sort): {}", start_innersort.to(end_innersort));

	println!("Attribut: Runtime Schwartzian Transformation (Inner-Sort + copy-to-tuple): {}", start_innersort.to(end_innersort) + start_copy_to_tuple.to(end_copy_to_tuple));

	// Create vector for sorted result
	let mut out_employees = vec![];

	// Push sorted result to the output vector
	for x in 0..temp_employees.len() {
		out_employees.push(temp_employees[x].0);
	}

	return out_employees;
}

fn method_sort(in_employees: Vec<Employee>) -> Vec<Employee> {

	// Sort vector by key
	// |k| k.employee_number --> sort by employee_number
	// |k| k.salary --> sort by salary
	let mut out_employees = in_employees.clone();
	out_employees.sort_by_key(|k| k.do_something());
	return out_employees;

}


fn method_schwartzian_transformation(in_employees: Vec<Employee>) -> Vec<Employee> {

	let start_copy_to_tuple = PreciseTime::now();
	// Create temporary vector for transformation
	let mut temp_employees = vec![];

	// Create Tuples with Employee and sort attribute (salary)
	for x in 0..in_employees.len() {
		let tuple = (in_employees[x].clone(), in_employees[x].do_something());
		temp_employees.push(tuple);
	}

	let end_copy_to_tuple = PreciseTime::now();
	println!("Method: Runtime Schwartzian Transformation (Copy-to-tuple): {}", start_copy_to_tuple.to(end_copy_to_tuple));
	// Start time of innersort
	let start_innersort = PreciseTime::now();

	temp_employees.sort_by_key(|k| k.1);

	// End time of innersort
	let end_innersort = PreciseTime::now();

	println!("Method: Runtime Schwartzian Transformation (Inner-Sort): {}", start_innersort.to(end_innersort));

	println!("Method: Runtime Schwartzian Transformation (Inner-Sort + copy-to-tuple): {}", start_innersort.to(end_innersort) + start_copy_to_tuple.to(end_copy_to_tuple));

	// Create vector for sorted result
	let mut out_employees = vec![];

	// Push sorted result to the output vector
	for x in 0..temp_employees.len() {
		out_employees.push(temp_employees[x].0);
	}

	return out_employees;
}
