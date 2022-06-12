use crate::dezena::Dezena;
use crate::centena::Centena;
use crate::number::Numero;

pub struct Cash {
    upper: Centena,
    lower: Dezena,
}

impl Cash {
    pub fn from(value: f32) -> Cash {
        Cash {
            upper: Centena::from(value.trunc() as u32),
            lower: Dezena::from((value.fract() * 100.00).round() as u32),
        }
    }

    pub fn humanize(&self) -> String{
        format!("{} reais e {} centavos", self.upper.humanize(), self.lower.humanize())
    }
}