
pub trait Numero {
    fn humanize(&self) -> String;
    fn token(&self) -> u32;
    fn is_zero(&self) -> bool;
}