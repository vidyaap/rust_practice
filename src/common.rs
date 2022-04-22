pub const SALARY: f64 = 80000.00;
pub const HOURLY: f64 = 20.00;

#[derive(Clone, Copy)]
pub enum SalaryType {
    Salaried,
    Hourly,
}

pub trait Employee {
    fn new(name: &str) -> Self
    where
        Self: Sized;

    fn name(&self) -> &str;
    fn enter_hours(&mut self, hours: u32);
    fn do_work(&self);
}

pub trait NonManager: Employee {
    fn calculate_payment(&self) -> f64 {
        match self.salary_type() {
            SalaryType::Salaried => SALARY,
            SalaryType::Hourly => HOURLY * self.hours_worked() as f64,
        }
    }
    fn hours_worked(&self) -> u32;
    fn salary_type(&self) -> SalaryType;
    fn receive_payment(&mut self, pay: f64);
}
