use crate::dezena::Dezena;
use crate::milhar::Milhar;
use crate::number::Numero;

pub struct Cash {
    upper: Milhar,
    lower: Dezena,
}

impl Cash {
    pub fn from(value: f32) -> Cash {
        Cash {
            upper: Milhar::from(value.trunc() as u32),
            lower: Dezena::from((value.fract() * 100.00).round() as u32),
        }
    }

    pub fn humanize(&self) -> String{
        format!("{} reais e {} centavos", self.upper.humanize(), self.lower.humanize())
    }
}