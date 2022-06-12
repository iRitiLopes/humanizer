use crate::centena::Centena;
use crate::number::Numero;

pub struct Milhar {
    value: u32,
    rest: Centena,
}

impl Milhar {
    pub fn from(value: u32) -> Milhar {
        let milhar = value / 1000;
        let rest = value - (milhar * 1000);
        Milhar {
            value: milhar,
            rest: Centena::from(rest),
        }
    }

}

impl Numero for Milhar {
    fn humanize(&self) -> std::string::String {
        let milhar: String;
        let centena = self.rest.humanize();

        match self.value {
            1 => milhar = String::from("mil"),
            2 => milhar = String::from("dois mil"),
            3 => milhar = String::from("três mil"),
            4 => milhar = String::from("quatro mil"),
            5 => milhar = String::from("cinco mil"),
            6 => milhar = String::from("seis mil"),
            7 => milhar = String::from("sete mil"),
            8 => milhar = String::from("oito mil"),
            9 => milhar = String::from("nove mil"),
            _ => milhar = String::from(""),
        };

        if self.value == 0 {
            return centena;
        }

        return format!("{} e {}", milhar, centena);
    }
    fn token(&self) -> u32 {
        self.value
    }
    fn is_zero(&self) -> bool {
        self.value == 0 && self.rest.is_zero()
    }
}
