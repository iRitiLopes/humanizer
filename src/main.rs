mod centena;
mod dezena;
mod unidade;
mod number;
mod cash;
use crate::cash::Cash;

fn main() {
    let numero = cash::Cash::from(506.03);
    println!("{}", show(&numero))
}


fn show(number: &Cash) -> String{
    number.humanize()
}