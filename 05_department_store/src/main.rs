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
        let department = self
            .people_by_department
            .entry(department)
            .or_insert(Vec::with_capacity(1));
        department.push(employee);
    }

    fn get_employees(&self) -> Vec<&Employee> {
        self.people_by_department
            .values()
            .flat_map(|employees| employees.iter())
            .collect()
    }

    fn get_employees_by_department(&self, department: &str) -> Option<&Vec<Employee>> {
        self.people_by_department.get(department)
    }

    fn remove_by_name(&mut self, name: &str, department: &str) {
        let department = self.people_by_department.get_mut(department);

        if let Some(employees) = department {
            employees.retain(|employee| employee.name != name);
        }
    }
}

fn main() {
    let mut company = Company::new();

    let mut buffer = String::new();
    loop {
        buffer.clear();
        let result = std::io::stdin().read_line(&mut buffer);
        if let Err(_) = result {
            println!("Error trying to get input. Please try again.");
            continue;
        }

        let tokens: Vec<_> = buffer.split_whitespace().collect();

        match tokens[0].to_lowercase().as_str() {
            "exit" => break,
            "add" => {
                let target_employee = Employee {
                    name: String::from(tokens[1]),
                };
                let target_department = String::from(tokens[3]);
                company.add_employee(target_employee, target_department);
            }
            "get" | "show" => {
                show(&company, tokens.get(3));
            }
            "remove" => {
                company.remove_by_name(tokens[1], tokens[3]);
            }
            _ => println!("Unknown input"),
        }
    }
    println!("Goodbye.")
}

fn show(company: &Company, target_department: Option<&&str>) {
    match target_department {
        None => {
            for (index, employee) in company.get_employees().iter().enumerate() {
                println!("{}. {}", index + 1, employee.name);
            }
        }
        Some(department) => {
            let opt_employees = company.get_employees_by_department(&department);
            match opt_employees {
                None => {
                    println!("Department does not exist!");
                }
                Some(employees) => {
                    for (index, employee) in employees.iter().enumerate() {
                        println!("{}. {}", index + 1, employee.name);
                    }
                }
            }
        }
    }
}
