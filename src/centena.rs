use crate::dezena::Dezena;
use crate::number::Numero;

pub struct Centena {
    value: u32,
    rest: Dezena,
}

impl Centena {
    pub fn from(value: u32) -> Centena {
        let centena = value / 100;
        let rest = value - (centena * 100);
        Centena {
            value: centena,
            rest: Dezena::from(rest),
        }
    }

    pub fn humanize_cem(&self) -> String {
        if !self.rest.is_zero() {
            return String::from("cento");
        } else {
            return String::from("cem");
        }
    }
}

impl Numero for Centena {
    fn humanize(&self) -> std::string::String {
        let centena: String;
        let dezena = self.rest.humanize();

        match self.value {
            1 => centena = self.humanize_cem(),
            2 => centena = String::from("duzentos"),
            3 => centena = String::from("trezentos"),
            4 => centena = String::from("quatrocentos"),
            5 => centena = String::from("quinhentos"),
            6 => centena = String::from("seissentos"),
            7 => centena = String::from("setessentos"),
            8 => centena = String::from("oitossentos"),
            9 => centena = String::from("novessentos"),
            _ => centena = String::from(""),
        };

        if self.value == 0 {
            return dezena;
        }

        if self.rest.is_zero() || self.value == 1 {
            return centena;
        }

        return format!("{} e {}", centena, dezena);
    }
    fn token(&self) -> u32 {
        self.value
    }
    fn is_zero(&self) -> bool {
        self.value == 0
    }
}
