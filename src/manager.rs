use crate::common;

pub struct Manager {
    name: String,
    employees: Vec<Box<dyn common::NonManager>>,
    hours_worked: u32,
}

impl Manager {
    pub fn add_employee<T: 'static + common::NonManager>(&mut self, emp: T) {
        self.employees.push(Box::new(emp));
    }
    pub fn display_employees(&mut self) {
        for _ in self.employees.iter_mut() {
            print!("----------");
        }
        println!();
        for emp in self.employees.iter_mut() {
            print!("{:^9}|", emp.name());
        }
        println!();
        for _ in self.employees.iter_mut() {
            print!("----------");
        }
        println!();
    }
    pub fn enter_emp_hours(&mut self, hours_by_emp: Vec<u32>) {
        assert_eq!(hours_by_emp.len(), self.employees.len());
        for (i, emp) in self.employees.iter_mut().enumerate() {
            emp.enter_hours(hours_by_emp[i]);
        }
    }
    pub fn get_num_employees(&self) -> usize {
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

impl common::Employee for Manager {
    fn new(name: &str) -> Self {
        Manager {
            name: String::from(name),
            employees: vec![],
            hours_worked: 0,
        }
        // Manager { name, full_time_employees: vec![], part_time_employees: vec![], hours_worked: 0 }
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
