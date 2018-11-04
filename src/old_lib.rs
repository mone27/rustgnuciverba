
use std::collections::HashMap;
use std::fmt;

extern crate rand;

use rand::Rng;
extern crate ndarray;
use ndarray::Array2;

pub fn generate_crossword(crossword: &mut Crossword, dict: Dict) -> (){

    crossword.put_word(&WordPos{dir: Direction::Vertical, row: 2, column: 2}, "ciao") ;
    crossword.put_word(&WordPos{dir: Direction::Horizontal, row: 5, column: 1}, "mondo");
    if let Some(w) = dict.find_random_word(&crossword.get_word(&WordPos{dir: Direction::Horizontal, row: 2,column: 2}, 5)){
       if w.len() >= 2 {
           crossword.put_word(&WordPos{dir: Direction::Horizontal, row: 2,column: 2}, w);

       }
    };
    let mut inserted_word = Vec::<(&str, WordPos)>::new();
    let mut dir = Direction::Horizontal;
    for row in 0..crossword.n_rows{
        for column in 0..crossword.n_columns{
            for _ in 0..2 {
                dir = !dir;
                let pos = WordPos { dir: dir, row: row, column: column };
                if let Some(w) = dict.find_random_word(&crossword.get_word(&pos,
                                                               crossword.get_max_word_length(&pos))) {
                    if w.len() >= 2 {
                        crossword.put_word(&pos, w);
                        inserted_word.push((w, pos));
                    }
                };
            }
        }
    }
    // !!!!! bug in column! every column is shifthed by one! must fix it
   println!("inserted word: {:?}", inserted_word);

}


//
//#[derive(Clone, Debug)]
//pub struct Crossword{
//    crossword: Array2::<char>,
//    n_rows: usize,
//    n_columns: usize,
//}
pub struct Crossword{

}
#[derive(Copy, Clone, Debug)]
enum Direction{
    Horizontal,
    Vertical
}

impl std::ops::Not for Direction{
    type Output = Direction;
    fn not(self) -> Direction {
        match self {
            Direction::Vertical => Direction::Horizontal,
            Direction::Horizontal => Direction::Vertical
        }
    }
}
#[derive(Copy, Clone, Debug)]
struct WordPos{
    row: usize,
    column: usize,
    dir: Direction
}

impl Crossword{
    pub fn new(number_row: usize, number_column: usize) -> Self{

        Crossword {
            crossword: vec!['_' ; number_column * number_row ],
            n_rows: number_row,
            n_columns: number_column,
        }

    }


    fn put_word(&mut self, word_position: &WordPos, word: impl AsRef<str>) -> (){
        let word = word.as_ref();
        for (c, pos) in word.chars().zip(0..word.len()) {
            self.crossword[match word_position.dir{
                Direction::Horizontal => word_position.row * self.n_columns + word_position.column + pos -1, //just a quick awful no sense quick fix
                Direction::Vertical => (word_position.row + pos) * self.n_columns + word_position.column
            }] = c;
        }

    }

    fn get_word(&self, word_position: &WordPos, len: usize) -> String{
        // TODO merge get_word and get_max_length for performance reasons?
        match word_position.dir {
            Direction::Horizontal => {
                let start_pos = word_position.row * self.n_columns + word_position.column;
                self.crossword[start_pos..(start_pos + len)].iter().collect()
            }
            Direction::Vertical => {
                let mut final_word = String::new();
                let start_pos = word_position.row * self.n_columns + word_position.column;
                for c in (start_pos..(start_pos + self.n_columns*len)).step_by(self.n_columns){
                    final_word.push(self.crossword[c])
                };
                final_word
            }
        }
    }
    fn get_max_word_length(&self, word_position: &WordPos) -> usize { //not sure good way to handle 0 len case
        match word_position.dir {
            Direction::Horizontal => {
                let start_pos = word_position.row * self.n_columns + word_position.column;
                self.crossword.iter()
                    .skip(start_pos)
                    .position(|&c|c == '#')
                    .unwrap_or((word_position.row+1)*self.n_columns) - start_pos
            } ,
            Direction::Vertical => {
                let start_pos = word_position.row * self.n_columns + word_position.column;
                self.crossword.iter()
                    .skip(start_pos)
                    .step_by(self.n_columns)
                    .position(|&c|c == '#')
                    .unwrap_or(self.n_rows - word_position.row)
            }
        }
    }

}
impl fmt::Display for Crossword{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "    ")?;
        for n in 0..self.n_columns{
            write!(f, "{:2} ", n)?
        };
        write!(f, "\n")?;
        let mut counter = 0;
        for c in &self.crossword{
            if counter % self.n_columns == 0 {
                write!(f, "\n")?;
                write!(f, "{:2}   ", counter/self.n_columns)?;
                write!(f, "{}  ", c)?;
            }
            write!(f, "{}  ", c)?;
            counter += 1;
        };
        Ok(())
    }


}


pub struct Dict{
    dict: HashMap<usize, Vec<String>>
}

impl Dict{
    pub fn new(list_word: &str) -> Self{
        Dict{
            dict: {
                let mut map: HashMap<usize, Vec<String>> = HashMap::new();
                for line in list_word.lines() {
                    map.entry(line.len()).or_insert(vec![]).push(line.to_string());
                }
                map
            }
        }
    }
    pub fn find_random_word(&self, query_word: &str) -> Option<&str>{
         match self.find_words(query_word){
            Some(words) => Some(rand::thread_rng().choose(&words).unwrap()),
            None => None
        }
    }
    fn check_word_ok (word: &str, query_word: &str) -> bool {
        // warning! does not treat '#' char properly, input must be checked with Crossword::get_max_word_length
        for (c_new, c_old) in word.chars().zip(query_word.chars()){
            if !(c_old == c_new || c_old == '_') {
                return false
            }
        }
        true
    }
    fn find_words(&self, query_word: &str) -> Option<Vec<& str>>{
        match self.dict.get(&query_word.len()){   //NEED TO OPTIMISE exploiting the fact that the dict is alphabetically sorted
            Some(all_words) => {
                let list_words: Vec<&str> = all_words.iter().filter(|word|
                    Self::check_word_ok(word, query_word)).map(AsRef::as_ref).collect();
                if list_words.len()== 0{
                    None
                }
                    else {
                        Some(list_words)
                    }
            },
            None => None
        }
    }
}
impl fmt::Display for Dict{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        for (len, vec) in &self.dict{
            writeln!(f, "\n\nlen: {} \n ", len)?;
            for w in vec{
                writeln!(f, "  {}",w)?;
            }
        };
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_column_insert(){
        let mut cross = Crossword::new(15,15);
    }
     #[test]
    fn test_check_word_ok(){
        assert!(Dict::check_word_ok("ciao", "ciao"));
        assert!(Dict::check_word_ok("ciao", "c___"));
        assert_ne!(Dict::check_word_ok("pluto", "pippo"), true);
        assert!(!Dict::check_word_ok("ciao", "pippo"));
        assert!(Dict::check_word_ok("c", "_"));
        assert!(!Dict::check_word_ok("ciao", "___d"))
    }
    #[test]
    fn test_find_word(){
        let dict = Dict::new(
                      "ciao\nprova\npippo\nprovando\ntutto\nprove\nabc\n"
        );
        assert_eq!(dict.find_words("ciao"), Some(vec!["ciao"]));
        assert_eq!(dict.find_words("prov_"),Some(vec!["prova","prove"]));
        assert_eq!(dict.find_words("___"), Some(vec!["abc"]));
        assert_eq!(dict.find_words("world"), None);
    }
    #[test]
    fn test_write_word(){

    }
    #[test]
    fn test_get_word(){

    }
    #[test]
    fn test_find_random_word(){

    }
    #[test]
    fn test_get_max_word_length(){
        let mut cross = Crossword::new(15, 15);
        //Horizontal
        cross.put_word(&WordPos{dir: Direction::Horizontal, row: 0, column: 4}, "#");
        assert_eq!(cross.get_max_word_length(&WordPos{dir: Direction::Horizontal, row: 0, column: 0}), 4);
        assert_eq!(cross.get_max_word_length(&WordPos{dir: Direction::Horizontal, row:5, column: 0}), 15);
        //Vertical
        cross.put_word(&WordPos{dir: Direction::Vertical, row: 6, column: 2}, "#");
        assert_eq!(cross.get_max_word_length(&WordPos{dir: Direction::Vertical, row: 0, column: 2}), 6);
        assert_eq!(cross.get_max_word_length(&WordPos{dir: Direction::Vertical, row: 0, column: 7}),15);
    }
}