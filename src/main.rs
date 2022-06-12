mod cash;

#[path = "decimal/centena.rs"]
mod centena;

#[path = "decimal/dezena.rs"]
mod dezena;

#[path = "decimal/milhar.rs"]
mod milhar;

#[path = "decimal/number.rs"]
mod number;

#[path = "decimal/unidade.rs"]
mod unidade;

use crate::cash::Cash;

fn main() {
    let numero = cash::Cash::from(50316.22);
    println!("{}", show(&numero))
}

fn show(number: &Cash) -> String {
    number.humanize()
}
