extern crate rand;
use rand::Rng;

// Define struct for employees

#[derive(Copy, Clone)]
struct Employee {
	name: &'static str,
	salary: u32,
}



fn main() {

	// Create vector
	let mut in_employees = vec![];

	for x in 0..5000 {
		let rand_number = rand::thread_rng().gen_range(1, 101);
		let rand_name = format!("Employee {}", rand_number);
		let rand_salary = rand::thread_rng().gen_range(1, 9000000);
		in_employees.push(Employee {name: rand_name, salary: rand_salary});
	}

	println!("Compare of Schwartzian Transoformation and Rust Sort");
	println!("");

	// Print vector of employees
	for x in 0..in_employees.len() {
		println!("{0},{1}", in_employees[x].name, in_employees[x].salary);
	}
	
	let out_schwartzian = schwartzian_transformation(in_employees.clone());
	
	let out_sort = sort(in_employees.clone());
	
	println!("");	
	println!("Result of Schwartzian Transoformation");
	println!("");
	
	for x in 0..out_schwartzian.len() {
		println!("{0},{1}", out_schwartzian[x].name, out_schwartzian[x].salary);
	}
	
	println!("");
	println!("Result of Rust Sort");
	println!("");
	
	for x in 0..out_sort.len() {
		println!("{0},{1}", out_sort[x].name, out_sort[x].salary);
	}

	
}


fn sort(in_employees: Vec<Employee>) -> Vec<Employee> {
	
	// Sort vector by key
	// |k| k.name --> sort by name
	// |k| k.salary --> sort by salary
	let mut out_employees = in_employees.clone();
	out_employees.sort_by_key(|k| k.salary);
	return out_employees;
	
}

fn schwartzian_transformation(in_employees: Vec<Employee>) -> Vec<Employee> {
	
	// Create temporary vector for transform
	let mut temp_employees = vec![];
	
	// 
	for x in 0..in_employees.len() {
		let tuple = (in_employees[x].clone(), in_employees[x].salary);
		temp_employees.push(tuple);
	}
	
	println!("");
	
	temp_employees.sort_by_key(|k| k.1);
	
	let mut out_employees = vec![];
	
	for x in 0..temp_employees.len() {
		out_employees.push(temp_employees[x].0);
	}
	
	return out_employees;
}
