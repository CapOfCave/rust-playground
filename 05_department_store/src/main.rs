use std::collections::HashMap;

struct Employee {
    name: String,
}

struct Company {
    people_by_department: HashMap<String, Vec<Employee>>,
}

impl Company {
    fn new() -> Self {
        Company {
            people_by_department: HashMap::new(),
        }
    }

    fn add_employee(&mut self, employee: Employee, department: String) {
        let department = self.people_by_department.entry(department).or_insert(Vec::with_capacity(1));
        department.push(employee);
    }

    fn get_employees(&self) -> Vec<&Employee> {
        self.people_by_department.values().flat_map(|employees| {employees.iter()}).collect()
    }
    
    fn remove_by_name(&mut self, name: &str, department: &str) {
        let department = self.people_by_department.get_mut(department);
        
        if let Some(employees) = department {
            employees.retain(|employee| {employee.name != name});
        }
    }
}

fn main() {
    let mut company = Company::new();

    loop {
        let mut input = String::new();
        let result = std::io::stdin().read_line(&mut input);
        if let Err(_) = result {
            println!("Error trying to get input. Please try again.");
            continue;
        }

        let tokens: Vec<_> = input.split_whitespace().collect();        

        match tokens[0].to_lowercase().as_str() {
            "exit" => break,
            "add" => {
                let target_employee = Employee {
                    name: String::from(tokens[1]),
                };
                let target_department = String::from(tokens[3]);
                company.add_employee(target_employee, target_department);
            },
            "get" | "show" => {
                for (index, employee) in company.get_employees().iter().enumerate() {
                    println!("{}. {}", index, employee.name)
                }
            },
            "remove" => {
                company.remove_by_name(tokens[1], tokens[3])

            }
            _ => println!("Unknown input")
        }
        
    }
    println!("Goodbye.")
}