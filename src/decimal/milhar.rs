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

        milhar = format!("{} mil", Centena::from(self.value).humanize());

        if self.value == 0 {
            return centena;
        }

        return format!("{} {}", milhar, centena);
    }
    fn token(&self) -> u32 {
        self.value
    }
    fn is_zero(&self) -> bool {
        self.value == 0 && self.rest.is_zero()
    }
}
