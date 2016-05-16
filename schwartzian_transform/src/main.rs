// Define struct for employees
struct Employee {
	name: &'static str,
	age: u32,
} 

fn main() {

	// Create vector
	let mut employees = vec![];
	  
	// Create employees and push them on the vector  
	employees.push(Employee {name: "Meier", age: 38});
	employees.push(Employee {name: "Peters", age: 98});
	employees.push(Employee {name: "Baumeister", age: 43});
	employees.push(Employee {name: "Glass", age: 21});
	employees.push(Employee {name: "Stade", age: 46});
	employees.push(Employee {name: "Klein", age: 52});
	employees.push(Employee {name: "Golle", age: 26});
	employees.push(Employee {name: "Helling", age: 61});
	employees.push(Employee {name: "Fischer", age: 32});
	employees.push(Employee {name: "Maug", age: 26});

	/// Sort vector by key
	/// |k| k.name --> sort by name
	/// |k| k.age --> sort by age
	employees.sort_by_key(|k| k.name);
	
	// Print vector of employees
	for x in 0..10 { 
		println!("{0},{1}", employees[x].name, employees[x].age);
	}
}
