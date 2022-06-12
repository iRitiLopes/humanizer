
use crate::number::Numero;

pub struct Unidade {
    value: u32,
}

impl Unidade {
    pub fn from(value: u32) -> Unidade {
        match value {
            _ => Unidade { value: value },
        }
    }
}

impl Numero for Unidade {
    fn humanize(&self) -> std::string::String {
        match self.value {
            1 => String::from("um"),
            2 => String::from("dois"),
            3 => String::from("tres"),
            4 => String::from("quatro"),
            5 => String::from("cinco"),
            6 => String::from("seis"),
            7 => String::from("sete"),
            8 => String::from("oito"),
            9 => String::from("nove"),
            _ => String::from(""),
        }
    }

    fn token(&self) -> u32 {
        self.value
    }
    fn is_zero(&self) -> bool {
        self.value == 0
    }
}