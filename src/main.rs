use std::str::FromStr;

use crate::employees::Employee;
use crate::full_time::FullTime;
use crate::manager::Manager;
use crate::part_time::PartTime;

mod employees;
mod full_time;
mod manager;
mod part_time;

/// Enum to represent the possible actions to take as a manager
#[derive(Debug)]
enum Action {
    AddEmployees,
    EmployeeHours,
    PayEmployees,
    EndDay,
}

impl FromStr for Action {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Action, Self::Err> {
        match input {
            "1" | "add" | "AddEmployees" => Ok(Action::AddEmployees),
            "2" | "hours" | "EmployeeHours" => Ok(Action::EmployeeHours),
            "3" | "pay" | "PayEmployees" => Ok(Action::PayEmployees),
            "4" | "end" | "EndDay" => Ok(Action::EndDay),
            _ => Err("Invalid action type"),
        }
    }
}

/// Enum to represent the types of employees allowed as input
#[derive(Debug)]
enum EmployeeType {
    FullTime,
    PartTime,
}

impl FromStr for EmployeeType {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<EmployeeType, Self::Err> {
        match input {
            "Full" => Ok(EmployeeType::FullTime),
            "Part" => Ok(EmployeeType::PartTime),
            _ => Err("Invalid employee type"),
        }
    }
}

/// Enum to represent user options in an infinite loop
#[derive(Debug, PartialEq)]
enum LoopAction {
    Done,
    Continue,
}

impl FromStr for LoopAction {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<LoopAction, Self::Err> {
        match input {
            "" => Ok(LoopAction::Done),
            "Done" => Ok(LoopAction::Done),
            _ => Ok(LoopAction::Continue),
        }
    }
}

/// Function to enter hours for all employees in manager's system
///
/// Loops until number and type of inputs are correct
fn add_employee_hours(manager: &mut Manager) {
    println!("Here are all your employees: ");
    manager.display_employees();
    println!("Enter each of their hours in order separated by spaces: ");
    loop {
        let mut hours = String::new();
        let _b = std::io::stdin().read_line(&mut hours).unwrap();
        let hours_vec: Vec<u32> = hours
            .as_str()
            .split_whitespace()
            .filter_map(|w| w.parse::<u32>().ok())
            .collect();
        if hours_vec.len() == manager.num_employees() {
            manager.enter_emp_hours(hours_vec);
            println!("Great! You successfully entered your employees' hours");
            break;
        }
        println!(
            "Sorry, looks like you didn't enter hours for as many employees as you have.\n \
        Please try again, and make sure you only enter numeric values!"
        )
    }
}

/// Function to add employees by name and type of employee (provided as stdin)
///
/// Loops until user stops (i.e. doesn't have more employees to add)
fn add_employees(manager: &mut Manager) {
    loop {
        println!(
            "Enter an employee name (enter 'Done' or press Enter if you have no more employees): "
        );
        let (emp, str_emp) = read_stdin();
        if emp == Ok(LoopAction::Done) {
            break;
        }
        let formatted_emp = str_emp.as_str().trim();
        println!(
            "Great! Is {} a full-time worker or a part-time worker?",
            formatted_emp
        );
        'employee_type: loop {
            println!("Enter Full for full-time or Part for part-time: ");
            let (emp_type, _) = read_stdin();
            match emp_type {
                Ok(EmployeeType::FullTime) => {
                    let ft = FullTime::new(formatted_emp);
                    manager.add_employee(ft);
                    println!(
                        "OK, {} has been added as a full-time employee!",
                        formatted_emp
                    );
                    break 'employee_type;
                }
                Ok(EmployeeType::PartTime) => {
                    let pt = PartTime::new(formatted_emp);
                    manager.add_employee(pt);
                    println!(
                        "OK, {} has been added as a part-time employee!",
                        formatted_emp
                    );
                    break 'employee_type;
                }
                Err(_) => println!("Sorry, that is not a valid employee type, try again!"),
            }
        }
    }
}

/// Function responsible for control flow of program
///
/// Presents activity options to user and calls appropriate functions for each action
fn choose_action(manager: &mut Manager) {
    loop {
        println!("What would you like to do now?");
        println!(
            "Enter the number, shorthand, or code of your choice (from the parentheses):\n\
    1) Add more employees (add or AddEmployees)\n\
    2) Enter employee hours (hours or EmployeeHours)\n\
    3) Pay employees (pay or PayEmployees)\n\
    4) Done for the day! (end or EndDay)"
        );
        let (choice, _) = read_stdin();
        match choice {
            Ok(Action::AddEmployees) => add_employees(manager),
            Ok(Action::EmployeeHours) => add_employee_hours(manager),
            Ok(Action::PayEmployees) => manager.pay_out(),
            Ok(Action::EndDay) => break,
            Err(_) => println!(
                "That is not a valid option, please enter a valid number or shorthand/code from the parentheses"
            ),
        }
    }
}

/// Helper to read from stdin and parse into any type that has implemented FromStr
fn read_stdin<T: FromStr<Err = &'static str>>() -> (Result<T, &'static str>, String) {
    let mut line = String::new();
    let _b = std::io::stdin().read_line(&mut line).unwrap();
    let parsed = line.as_str().trim().parse();
    match parsed {
        Ok(_) => (parsed, line),
        Err(_) => (Err("Unable to parse line from stdin"), line),
    }
}

fn main() {
    let mut name = String::new();
    println!("Welcome to your new company!");
    println!("You are a manager, please enter your name:");
    let _b = std::io::stdin().read_line(&mut name).unwrap();
    let mut manager = manager::Manager::new(name.as_str());
    println!("Now you can add some employees!");
    add_employees(&mut manager);
    println!("Congrats! Your employees are all in the system.");
    manager.display_employees();
    choose_action(&mut manager);
    println!("Bye! See you tomorrow!");
}
