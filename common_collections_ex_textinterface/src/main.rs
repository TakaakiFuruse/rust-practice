use std::io;

struct Employee {
    name: String,
    department: String,
}

fn main() {
    let mut employees: Vec<Employee> = vec![];

    loop {
        println!("L to list employees, N to add new employee, Q to quit");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("no error");

        match input.trim() {
            "L" => match &employees.is_empty() {
                true => println!("No Employees to list!"),
                false => {
                    for employee in &employees {
                        println!("name: {}\n dpt:  {}", employee.name, employee.department);
                    }
                }
            },
            "N" => {
                println!("NAME?");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("no error");
                println!("DEPARTMENT?");
                let mut department = String::new();
                io::stdin().read_line(&mut department).expect("no error");
                employees.push(Employee {
                    name: name,
                    department: department,
                })
            }
            "Q" => break,
            _ => println!("non"),
        }
    }
}
