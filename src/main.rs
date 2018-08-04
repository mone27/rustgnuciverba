use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fmt;


fn main() {

    let mut crossword = Gnuciverba::new(
        15, 15,
        "/home/simone/PycharmProjects/gnuciverba/1000_parole_italiane_comuni.txt")
        .unwrap();
    //crossword.print_dict();
    crossword.generate();
    println!("{}", crossword);


}

#[derive(Debug)]
struct Gnuciverba{
    crossword: Vec<Vec<char>>,
    dict: HashMap<usize, Vec<String>>
}

enum Direction{
    horizontal,
    vertical
}
struct  Position{
    row: usize,
    column: usize
}
impl Gnuciverba{
    pub fn generate(&mut self) -> (){
        self.write_word_horizontal(String::from("ciao"), Position{row: 2, column: 2});
        self.write_word_vertical(String::from("mondo"), Position{row: 5, column: 1});
        println!("{}", self.get_word(Position{row: 2, column:2}, 4, Direction::horizontal));
        println!("{}", self.get_word(Position{row: 2, column:2}, 4, Direction::vertical));

    }





    pub fn new(number_row: usize, number_column: usize, dict_path: &str) -> Result<Self, std::io::Error>{

        let mut file = File::open(dict_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(Gnuciverba {
            crossword: vec![vec!['_'; number_row]; number_column ],
            dict: {
                Gnuciverba::string_to_classified_bcolumn_length(contents)
            }
        })

    }

    fn write_word_vertical(&mut self, word: String, position: Position) -> () {
        /// it panics if gives out of bond
        word.chars().zip(0..word.len())
            .map(|(c, pos)| {
                self.crossword[position.row][position.column+ pos]= c;
            })
            .for_each(drop)
    }

    fn write_word_horizontal(&mut self, word: String, position: Position) -> (){
        word.chars().zip(0..word.len())
            .map(|(c, pos)| {
                self.crossword[position.row + pos][position.column] = c;
            })
            .for_each(drop)
    }

    fn check_if_can_insert(&self, word: String, position: Position, dir: Direction) -> bool{
        let old_word = self.get_word(position, word.len(), dir);
        word.chars().zip(old_word.chars()).any(|(c_new, c_old)|
            c_old != '_' && c_old != c_new )

    }

    fn get_word(&self, pos: Position, len: usize, dir: Direction) -> String{
        match dir {
            Direction::horizontal => {
                self.crossword[pos.row][pos.column..(pos.column + len)].iter().collect()
            }
            Direction::vertical => {
                self.crossword[pos.row..(pos.row+len)].iter()
                    .map(|vec|vec[pos.column] ).collect()
            }
        }

    }
    fn string_to_classified_bcolumn_length(string: String) -> HashMap<usize, Vec<String>> {
        let mut map: HashMap<usize, Vec<String>> = HashMap::new();
        for line in string.lines() {
            if let Some(mut inner_vector) = map.get_mut(&(line.len() )) {
                inner_vector.push(line.to_string());
            }
            if let None = map.get(&(line.len()))  {
                    map.insert(line.len(), vec![line.to_string()]);
            }
        }
        map
    }
    pub fn print_dict(&self){
        for (len, vec) in &self.dict{
            println!("\n\nlen: {} \n ", len);
            for w in vec{
                println!("  {}",w)
            }
        }
    }

}
impl fmt::Display for Gnuciverba{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "    ")?;
        for n in 0..self.crossword.len(){
                write!(f, "{} ", n)?
            };
        write!(f, "\n")?;
        let mut counter = 0;
        for row in &self.crossword{
            write!(f, "{:2}   ", counter)?;
            for c in row{
                write!(f, "{} ", c)?;
            }
            write!(f, "\n")?;
            counter += 1;
        };
        Ok(())
    }


}
