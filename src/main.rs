use crate::common::Employee;
use crate::full_time::FullTime;
use crate::manager::Manager;
use crate::part_time::PartTime;

mod common;
mod full_time;
mod manager;
mod part_time;

fn activity_options_loop(mut _manager: &mut Manager) {
    loop {
        println!("What would you like to do now?");
        println!(
            "Enter the number of your choice:\n\
    1) Add more employees\n\
    2) Enter employee hours\n\
    3) Pay employees\n\
    4) Done for the day!"
        );
        let mut choice = String::new();
        let _b = std::io::stdin().read_line(&mut choice).unwrap();
        match choice.as_str().trim() {
            "1" => employee_setup_loop(_manager),
            "2" => employee_hours(_manager),
            "3" => _manager.pay_out(),
            "4" => break,
            _ => println!("That is not a valid option, please enter one of 1, 2, 3 or 4"),
        }
    }
}

fn employee_hours(mut _manager: &mut Manager) {
    println!("Here are all your employees: ");
    _manager.display_employees();
    println!("Enter each of their hours in order separated by spaces: ");
    loop {
        let mut hours = String::new();
        let _b = std::io::stdin().read_line(&mut hours).unwrap();
        let hours_vec: Vec<u32> = hours
            .as_str()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        if hours_vec.len() == _manager.get_num_employees() {
            _manager.enter_emp_hours(hours_vec);
            println!("Great! You successfully entered your employees' hours");
            break;
        }
        println!(
            "Sorry, looks like you didn't enter hours for as many employees as you have. \
        Please try again!"
        )
    }
}

fn employee_setup_loop(mut _manager: &mut Manager) {
    loop {
        let mut emp = String::new();
        println!(
            "Enter an employee name (enter 'Done' or press Enter if you have no more employees): "
        );
        let _b0 = std::io::stdin().read_line(&mut emp).unwrap();
        let formatted_emp = emp.as_str().trim();
        if formatted_emp == "Done" || formatted_emp == "" {
            break;
        }
        println!(
            "Great! Is {} a full-time worker or a part-time worker?",
            formatted_emp
        );
        'employee_type: loop {
            let mut emp_type = String::new();
            println!("Enter 0 for full-time or 1 for part-time: ");
            let _b1 = std::io::stdin().read_line(&mut emp_type).unwrap();
            match emp_type.as_str().trim() {
                "0" => {
                    let ft = FullTime::new(formatted_emp);
                    _manager.add_employee(ft);
                    println!(
                        "OK, {} has been added as a full-time employee!",
                        formatted_emp
                    );
                    break 'employee_type;
                }
                "1" => {
                    let pt = PartTime::new(formatted_emp);
                    _manager.add_employee(pt);
                    println!(
                        "OK, {} has been added as a part-time employee!",
                        formatted_emp
                    );
                    break 'employee_type;
                }
                _ => println!("Sorry, that is not a valid employee type, try again!"),
            }
        }
    }
}

fn main() {
    let mut name = String::new();
    println!("Welcome to your new company!");
    println!("You are a manager, please enter your name:");
    let _b = std::io::stdin().read_line(&mut name).unwrap();
    let mut _manager = manager::Manager::new(name.as_str());
    println!("Now you can add some employees!");
    employee_setup_loop(&mut _manager);
    println!("Congrats! Your employees are all in the system.");
    _manager.display_employees();
    activity_options_loop(&mut _manager);
    println!("Bye! See you tomorrow!");
}
