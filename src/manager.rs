use crate::common::Employee;
use crate::common::NonManager;

pub struct Manager {
    name: String,
    employees: Vec<Box<dyn NonManager>>,
    hours_worked: u32,
}

impl Manager {
    pub fn add_employee<T: 'static + NonManager>(&mut self, emp: T) {
        self.employees.push(Box::new(emp));
    }

    pub fn display_employees(&self) {
        println!("{}", "----------".repeat(self.employees.len()));
        for emp in self.employees.iter() {
            print!("{:^9}|", emp.name());
        }
        println!("\n{}", "----------".repeat(self.employees.len()));
    }

    pub fn enter_emp_hours(&mut self, hours_by_emp: Vec<u32>) {
        assert_eq!(hours_by_emp.len(), self.employees.len());
        let it = hours_by_emp.iter().zip(self.employees.iter_mut());
        for (_, (hours, emp)) in it.enumerate() {
            emp.enter_hours(*hours);
        }
    }

    pub fn num_employees(&self) -> usize {
        self.employees.len()
    }

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
