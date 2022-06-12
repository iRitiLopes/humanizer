
use crate::number::Numero;
use crate::unidade::Unidade;

pub struct Dezena {
    value: u32,
    rest: Unidade,
}

impl Dezena {
    pub fn from(value: u32) -> Dezena {
        let dezena = value / 10;
        let unidade = value % 10;
        match dezena {
            _ => Dezena {
                value: dezena,
                rest: Unidade::from(unidade),
            },
        }
    }

    pub fn humanize_dez(&self) -> String {
        if !self.rest.is_zero() {
            return match self.rest.token() {
                1 => String::from("onze"),
                2 => String::from("doze"),
                3 => String::from("treze"),
                4 => String::from("catorze"),
                5 => String::from("quinze"),
                6 => String::from("dezesseis"),
                7 => String::from("dezessete"),
                8 => String::from("dezoito"),
                9 => String::from("dezenove"),
                _ => String::from(""),
            };
        } else {
            return String::from("dez");
        }
    }
}

impl Numero for Dezena {
    fn humanize(&self) -> std::string::String {
        let ten: String;
        let oneness = self.rest.humanize();

        match self.value {
            1 => ten = self.humanize_dez(),
            2 => ten = String::from("vinte"),
            3 => ten = String::from("trinta"),
            4 => ten = String::from("quarenta"),
            5 => ten = String::from("cinquenta"),
            6 => ten = String::from("sessenta"),
            7 => ten = String::from("setenta"),
            8 => ten = String::from("oitenta"),
            9 => ten = String::from("noventa"),
            _ => ten = String::from(""),
        };

        if self.value == 0 {
            return oneness;
        }

        if self.rest.is_zero() || self.value == 1 {
            return ten;
        }

        return format!("{} e {}", ten, oneness);
    }
    fn token(&self) -> u32 {
        self.value
    }
    fn is_zero(&self) -> bool {
        self.value == 0 && self.rest.is_zero()
    }
}