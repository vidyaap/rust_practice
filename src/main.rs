use std::str::FromStr;

use crate::employees::Employee;
use crate::full_time::FullTime;
use crate::manager::Manager;
use crate::part_time::PartTime;

mod employees;
mod full_time;
mod manager;
mod part_time;

#[derive(Debug)]
enum Action {
    AddEmployees,
    EmployeeHours,
    PayEmployees,
    EndDay,
}

impl FromStr for Action {
    type Err = ();
    fn from_str(input: &str) -> Result<Action, Self::Err> {
        match input {
            "AddEmployees" => Ok(Action::AddEmployees),
            "EmployeeHours" => Ok(Action::EmployeeHours),
            "PayEmployees" => Ok(Action::PayEmployees),
            "EndDay" => Ok(Action::EndDay),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum EmployeeType {
    FullTime,
    PartTime,
}

impl FromStr for EmployeeType {
    type Err = ();
    fn from_str(input: &str) -> Result<EmployeeType, Self::Err> {
        match input {
            "Full" => Ok(EmployeeType::FullTime),
            "Part" => Ok(EmployeeType::PartTime),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
enum LoopAction {
    Done,
    Continue,
}

impl FromStr for LoopAction {
    type Err = ();
    fn from_str(input: &str) -> Result<LoopAction, Self::Err> {
        match input {
            "" => Ok(LoopAction::Done),
            "Done" => Ok(LoopAction::Done),
            _ => Ok(LoopAction::Continue),
        }
    }
}

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
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        if hours_vec.len() == manager.num_employees() {
            manager.enter_emp_hours(hours_vec);
            println!("Great! You successfully entered your employees' hours");
            break;
        }
        println!(
            "Sorry, looks like you didn't enter hours for as many employees as you have. \
        Please try again!"
        )
    }
}

fn add_employees(manager: &mut Manager) {
    loop {
        let mut emp = String::new();
        println!(
            "Enter an employee name (enter 'Done' or press Enter if you have no more employees): "
        );
        let _b0 = std::io::stdin().read_line(&mut emp).unwrap();
        let formatted_emp = emp.as_str().trim();
        if formatted_emp.parse() == Ok(LoopAction::Done) {
            break;
        }
        println!(
            "Great! Is {} a full-time worker or a part-time worker?",
            formatted_emp
        );
        'employee_type: loop {
            let mut emp_type = String::new();
            println!("Enter Full for full-time or Part for part-time: ");
            let _b1 = std::io::stdin().read_line(&mut emp_type).unwrap();
            match emp_type.as_str().trim().parse() {
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
                _ => println!("Sorry, that is not a valid employee type, try again!"),
            }
        }
    }
}

fn choose_action(manager: &mut Manager) {
    loop {
        println!("What would you like to do now?");
        println!(
            "Enter the code of your choice (from the parentheses):\n\
    1) Add more employees (AddEmployees)\n\
    2) Enter employee hours (EmployeeHours)\n\
    3) Pay employees (PayEmployees)\n\
    4) Done for the day! (EndDay)"
        );
        let mut choice = String::new();
        let _b = std::io::stdin().read_line(&mut choice).unwrap();
        match choice.as_str().trim().parse() {
            Ok(Action::AddEmployees) => add_employees(manager),
            Ok(Action::EmployeeHours) => add_employee_hours(manager),
            Ok(Action::PayEmployees) => manager.pay_out(),
            Ok(Action::EndDay) => break,
            _ => println!(
                "That is not a valid option, please enter a valid code from the parentheses"
            ),
        }
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
