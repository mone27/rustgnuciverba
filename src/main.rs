
extern crate rustgnuciverba;
use rustgnuciverba::{Crossword, Dict, generate_crossword};
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let mut content = String::new();
    File::open("/home/simone/PycharmProjects/gnuciverba/1000_parole_italiane_comuni.txt").unwrap().read_to_string(&mut content).expect("could not read file");
    let mut crossword = Crossword::new(12, 25);
    generate_crossword(&mut crossword,Dict::new(&content));
    println!("{}", crossword);

}

