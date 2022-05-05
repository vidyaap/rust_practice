use crate::employees::Employee;
use crate::employees::HasManager;

/// Struct to represent managers with a number of employees
pub struct Manager {
    name: String,
    employees: Vec<Box<dyn HasManager>>,
    hours_worked: u32,
}

impl Manager {
    /// Function to add an employee to a manager's list
    pub fn add_employee<T: 'static + HasManager>(&mut self, emp: T) {
        self.employees.push(Box::new(emp));
    }

    /// Function to print a string representation of a manager's employee list to the console
    pub fn display_employees(&self) {
        println!("{}", "----------".repeat(self.employees.len()));
        for emp in self.employees.iter() {
            print!("{:^9}|", emp.name());
        }
        println!("\n{}", "----------".repeat(self.employees.len()));
    }

    /// Function to enter hours to all of a manager's employees at once
    ///
    /// Order and length of hours vector must match those of employee list
    pub fn enter_emp_hours(&mut self, hours_by_emp: Vec<u32>) {
        assert_eq!(hours_by_emp.len(), self.employees.len());
        hours_by_emp.iter()
            .zip(self.employees.iter_mut())
            .for_each(|(hours, emp)| emp.enter_hours(*hours));
    }

    /// Function to get the length of a manager's employee list
    pub fn num_employees(&self) -> usize {
        self.employees.len()
    }

    /// Function to pay a manager's employees based on their salary types and the hours they worked
    pub fn pay_out(&mut self) {
        for emp in self.employees.iter_mut() {
            let amount = emp.calculate_payment();
            println!("Paid {} ${} for the pay period", emp.name(), amount);
            emp.receive_payment(amount);
        }
    }
}

impl Employee for Manager {
    fn new(name: &str) -> Self {
        Manager {
            name: String::from(name),
            employees: vec![],
            hours_worked: 0,
        }
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn enter_hours(&mut self, hours: u32) {
        self.hours_worked += hours;
    }

    fn do_work(&self) {
        println!("Taking a meeting");
    }
}
