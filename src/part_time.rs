use crate::employees::Employee;
use crate::employees::HasManager;
use crate::employees::SalaryType;

/// Struct to represent part-time employees
pub struct PartTime {
    name: String,
    hours_worked: u32,
    paid_for_period: bool,
    salary_type: SalaryType,
    total_pay: f64,
}

impl Employee for PartTime {
    fn new(name: &str) -> Self {
        PartTime {
            name: String::from(name),
            hours_worked: 0,
            paid_for_period: false,
            salary_type: SalaryType::Hourly,
            total_pay: 0.0,
        }
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn enter_hours(&mut self, hours: u32) {
        self.hours_worked += hours;
    }

    fn do_work(&self) {
        println!("I work part of the week");
    }
}

impl HasManager for PartTime {
    fn hours_worked(&self) -> u32 {
        self.hours_worked
    }

    fn salary_type(&self) -> SalaryType {
        self.salary_type
    }

    fn receive_payment(&mut self, pay: f64) {
        self.total_pay += pay;
        self.paid_for_period = true;
    }
}
