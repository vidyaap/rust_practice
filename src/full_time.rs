use crate::common;
use crate::common::SalaryType;

pub struct FullTime {
    name: String,
    hours_worked: u32,
    paid_for_period: bool,
    salary_type: common::SalaryType,
    total_pay: f64,
}

impl common::Employee for FullTime {
    fn new(name: &str) -> Self {
        FullTime {
            name: String::from(name),
            hours_worked: 0,
            paid_for_period: false,
            salary_type: common::SalaryType::Salaried,
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
        println!("I work all week");
    }
}

impl common::NonManager for FullTime {
    // fn calculate_payment(&self) -> f64 {
    //     common::SALARY
    // }
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
