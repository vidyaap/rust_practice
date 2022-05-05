/// Constant to store salary value for full-time employees
pub const SALARY: f64 = 80000.00;
/// Constant to store hourly pay for part-time employees
pub const HOURLY: f64 = 20.00;

/// Enum to represent salary types, choice of Salaried or Hourly
#[derive(Clone, Copy)]
pub enum SalaryType {
    Salaried,
    Hourly,
}

/// Trait to represent all employees
pub trait Employee {
    /// Constructor for new employees
    fn new(name: &str) -> Self
    where
        Self: Sized;

    /// Function to get an employee's name
    fn name(&self) -> &str;

    /// Function to add hours to an employee's total hours worked
    fn enter_hours(&mut self, hours: u32);

    /// Function to represent the work that each type of employee could do
    fn do_work(&self);
}

/// Trait to represent any employee that has a manager
pub trait HasManager: Employee {
    /// Function to calculate an employee's pay based on their salary type and number of hours worked
    fn calculate_payment(&self) -> f64 {
        match self.salary_type() {
            SalaryType::Salaried => SALARY,
            SalaryType::Hourly => HOURLY * self.hours_worked() as f64,
        }
    }

    /// Function to get the number of hours an employee worked
    fn hours_worked(&self) -> u32;

    /// Function to get an employee's salary type
    fn salary_type(&self) -> SalaryType;

    /// Function to add received payment to an employee's total pay for a pay period
    fn receive_payment(&mut self, pay: f64);
}
