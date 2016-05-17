// Define struct for employees
struct Employee {
	name: &'static str,
	age: u32,
}

fn main() {

	// Create vector
	let mut in_employees = vec![];

	// Create employees and push them on the vector
	in_employees.push(Employee {name: "Meier", age: 38});
	in_employees.push(Employee {name: "Peters", age: 98});
	in_employees.push(Employee {name: "Baumeister", age: 43});
	in_employees.push(Employee {name: "Glass", age: 21});
	in_employees.push(Employee {name: "Stade", age: 46});
	in_employees.push(Employee {name: "Klein", age: 52});
	in_employees.push(Employee {name: "Golle", age: 26});
	in_employees.push(Employee {name: "Helling", age: 61});
	in_employees.push(Employee {name: "Fischer", age: 32});
	in_employees.push(Employee {name: "Maug", age: 26});

	// Sort vector by key
	// |k| k.name --> sort by name
	// |k| k.age --> sort by age
	//in_employees.sort_by_key(|k| k.age);

	println!("Schwartzian Transformation");
	println!("");

	// Print vector of employees
	for x in 0..in_employees.len() {
		println!("{0},{1}", in_employees[x].name, in_employees[x].age);
	}
	
	// Create temporary vector for transform
	let mut temp_employees = vec![];
	
	// 
	for x in 0..in_employees.len() {
		let tuple = (&in_employees[x], in_employees[x].age);
		temp_employees.push(tuple);
	}
	
	println!("");
	
	temp_employees.sort_by_key(|k| k.1);
	
	let mut out_employees = vec![];
	
	for x in 0..temp_employees.len() {
		out_employees.push(temp_employees[x].0);
		println!("{0}, {1}", out_employees[x].name, out_employees[x].age);
	}
	
	
}
