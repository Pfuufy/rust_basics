use std::{
    io,
    collections::HashMap
};

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    // Ask the user if they would like to update information,
    // or fetch information
    loop {
        println!("What would you like to do? Input 'a' or 'b'");
        println!("a. Update personnel");
        println!("b. Fetch personnel data");
    
        let mut action = String::new();
    
        // Get the users input
        io::stdin()
            .read_line(&mut action)
            .expect("Please enter a valid option");

        // Trim the input
        action = String::from(action.trim());

        // Call the correct function based on input
        if action.trim() == "a" {
            employees = update_data(employees);
            // println!("{:?}", employees);
        } else if action == "b" {
            fetch_data(&employees);
            // println!("{:?}", employees);
        } else {
            println!("Unknown option. Try again.");
        }

        println!("");
    }
}

fn update_data(mut employees: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let (verb, employee, department) = get_user_action();

    if verb == "add" || verb == "Add" {
        // This overrides an existing property if the key already exists
        // employees.insert(String::from(employee), department);
        
        employees.entry(department).or_insert(Vec::new()).push(employee);
    } else if verb == "remove" || verb == "Remove" {
        // employees.remove(&employee);
        // employees = employees.into_iter().filter(|(k, _)| *k != employee).collect();

        let new_employees = employees.get(&department);
        let mut updated_employees: Vec<String> = Vec::new();

        match new_employees {
            Some(emp) => {
                for e in emp {
                    if *e != employee {
                        updated_employees.push(e.to_string());
                    }
                }
            },
            None => println!("{} not found in {}", employee, department)
        }

        employees.insert(department, updated_employees);

    } else {
        println!("Unknown command '{}', please try again.", &verb);
    }

    employees
}

fn get_user_action() -> (String, String, String) {
    println!("Enter the action you would like to perform");
    println!("ex. Add Dave to accounting");

    let mut action = String::new();

    io::stdin()
        .read_line(&mut action)
        .expect("Please enter a valid action");

    let action = String::from(action.trim());
    let action: Vec<&str> = action.split(" ").collect();

    // for el in &action {
    //     println!("{}", el);
    // }

    let verb = action[0].to_string();
    let employee = action[1].to_string();
    let department = action[3].to_string();

    (verb, employee, department)
}

fn fetch_data(employees: &HashMap<String, Vec<String>>) {
    println!("Enter the department whose personnel you would like to see:");

    let mut department = String::new();

    io::stdin().read_line(&mut department).expect("Please enter a valid action");

    department = department.trim().to_string();

    // println!("{:?}", employees.get(&department));

    match employees.get(&department) {
        Some(e) => {
            for i in e {
                println!("{}", i);
            }
        },
        None => println!("No employees found for the {} department.", department)
    }
}
