mod centena;
mod dezena;
mod unidade;
mod number;
mod cash;
mod milhar;
use crate::cash::Cash;

fn main() {
    let numero = cash::Cash::from(9316.22);
    println!("{}", show(&numero))
}


fn show(number: &Cash) -> String{
    number.humanize()
}