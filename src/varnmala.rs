use crate::Varn;

pub struct Varnmala {
    varns: Vec<Varn>,
}

impl Varnmala {
    pub fn new() -> Self {
        Varnmala { varns: Vec::new() }
    }

    pub fn add_varn(&mut self, new_varn: Varn) {
        self.varns.push(new_varn);
    }

    pub fn initialize_varns(&mut self) {
        for code_point in 0x0900..=0x0954 {
            let varn = Varn::from_scalar(code_point);
            self.add_varn(varn);
        }
    }

    pub fn print_all_varns(&self) {
        for varn in &self.varns {
            varn.print();
        }
    }
}
