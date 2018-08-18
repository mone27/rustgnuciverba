
use std::collections::HashMap;
use std::fmt;

extern crate rand;

use rand::Rng;

pub fn generate_crossword(crossword: &mut Crossword, dict: Dict) -> (){

    crossword.write_word(WordPos{dir: Direction::Vertical, row: 2, column: 2}, "ciao") ;
    crossword.write_word(WordPos{dir: Direction::Horizontal, row: 5, column: 1}, "mondo");
    if let Some(w) = dict.find_random_word(&crossword.get_word(WordPos{dir: Direction::Horizontal, row: 2,column: 2}, 3)){
       crossword.write_word(WordPos{dir: Direction::Horizontal, row: 2,column: 2}, w)
    };
}











#[derive(Clone, Debug)]
pub struct Crossword{
    crossword: Vec<char>,
    n_rows: usize,
    n_columns: usize,
}
#[derive(Copy, Clone, Debug)]
enum Direction{
    Horizontal,
    Vertical
}
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


    fn write_word(&mut self, word_position: WordPos,  word: impl AsRef<str>) -> (){
        let word = word.as_ref();
        for (c, pos) in word.chars().zip(0..word.len()) {
            self.crossword[match word_position.dir{
                Direction::Horizontal => word_position.row * self.n_columns + word_position.column + pos,
                Direction::Vertical => (word_position.row + pos) * self.n_columns + word_position.column
            }] = c;
        }

    }

    fn get_word(&self, word_position: WordPos, len: usize) -> String{
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
//impl  Gnuciverba{
//
//    pub fn generate(&mut self) -> (){
//        self.write_word(Direction::Vertical, Position{row: 2, column: 2}, "ciao") ;
//        self.write_word(Direction::Horizontal, Position{row: 5, column: 1}, "mondo");
//        if let Some(w) = self.find_random_word(&self.get_word(Direction::Horizontal, Position{row: 2,column: 2}, 3)){
//            self.write_word(Direction::Horizontal, Position{row: 2,column: 2}, w)
//        };
//
//
//    }
//    pub fn new(number_row: usize, number_column: usize, list_word: &str) -> Self{
//
//        Gnuciverba {
//            crossword: vec!['_' ; number_column * number_row ],
//            dict: {
//                let mut map: HashMap<usize, Vec<String>> = HashMap::new();
//                for line in list_word.lines() {
//                    map.entry(line.len()).or_insert(vec![]).push(line.to_string());
//                }
//                map
//            },
//            n_rows: number_row,
//            n_columns: number_column,
//        }
//
//    }
//
//
//    fn write_word(&mut self,dir: Direction, position: Position,  word: impl AsRef<str>) -> (){
//        let word = word.as_ref();
//        for (c, pos) in word.chars().zip(0..word.len()) {
//            self.crossword[match dir{
//                Direction::Horizontal => position.row * self.n_columns + position.column + pos,
//                Direction::Vertical => (position.row + pos) * self.n_columns + position.column
//            }] = c;
//        }
//
//    }
//
//    fn get_word(&self, dir: Direction, pos: Position, len: usize) -> String{
//        match dir {
//            Direction::Horizontal => {
//                let start_pos = pos.row * self.n_columns + pos.column;
//                self.crossword[start_pos..(start_pos + len)].iter().collect()
//            }
//            Direction::Vertical => {
//                let mut return_string = String::new();
//                let start_pos = pos.row * self.n_columns + pos.column;
//                for c in (start_pos..(start_pos + self.n_columns*len)).step_by(self.n_columns){
//                    return_string.push(self.crossword[c])
//                };
//                return_string
//            }
//        }
//    }
//
//
//    fn find_words(&self, query_word: &str) -> Option<Vec<& str>>{
//        // must not copy here!!!
//        match self.dict.get(&query_word.len()){   //NEED TO OPTIMISE exploiting the fact that the dict is alphabetically sorted
//            Some(all_words) => {
//                let list_words: Vec<&str> = all_words.iter().filter(|word|
//                    Self::check_word_ok(word, query_word)).map(AsRef::as_ref).collect();
//                if list_words.len()== 0{
//                    None
//                }
//                else {
//                    Some(list_words)
//                }
//                },
//            None => None
//        }
//    }
//
//    fn find_random_word(&self, query_word: &str) -> Option<String>{
//        // returning a String it copies data without reason, to avoid ownership problem it could be an idea to split crosswor struct in crossword and dict
//        match self.find_words(query_word){
//            Some(words) => Some(rand::thread_rng().choose(&words).unwrap().to_string()), // optimize: remove copy of data
//            None => None
//        }
//    }
//    fn check_word_ok (word: &str, query_word: &str) -> bool {
//         for (c_new, c_old) in word.chars().zip(query_word.chars()){
//             if !(c_old == c_new || c_old == '_') {
//                 return false
//             }
//         }
//         true
//    }
////    fn string_to_classified_column_length(string: &str) -> HashMap<usize, Vec<&str>> {
////        let mut map: HashMap<usize, Vec<&str>> = HashMap::new();
////        for line in string.lines() {
////            let line = line.to_string();
////            map.entry(line.len()).or_insert(vec![]).push(&line);
////        }
////        map
////    }
//    pub fn print_dict(&self){
//        for (len, vec) in &self.dict{
//            println!("\n\nlen: {} \n ", len);
//            for w in vec{
//                println!("  {}",w)
//            }
//        }
//    }
//
//}
//impl fmt::Display for Gnuciverba{
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
//        write!(f, "    ")?;
//        for n in 0..self.n_columns{
//            write!(f, "{:2} ", n)?
//        };
//        write!(f, "\n")?;
//        let mut counter = 0;
//        for c in &self.crossword{
//            if counter % self.n_columns == 0 {
//                write!(f, "\n")?;
//                write!(f, "{:2}   ", counter/self.n_columns)?;
//                write!(f, "{}  ", c)?;
//            }
//            write!(f, "{}  ", c)?;
//            counter += 1;
//        };
//        Ok(())
//    }
//
//
//}


pub fn test() {

}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn first_test(){
        assert_eq!(test(), ())
    }
     #[test]
    fn test_check_word_ok(){
         let mut crossword = Gnuciverba::new(
             15, 15,
             "ciao\nprova\npippo\nprovando\ntutto\nprove\nabc\n"
         );
        assert!(Gnuciverba::check_word_ok("ciao", "ciao"));
        assert!(Gnuciverba::check_word_ok("ciao", "c___"));
        assert_ne!(Gnuciverba::check_word_ok("pluto", "pippo"), true);
        assert!(!Gnuciverba::check_word_ok("ciao", "pippo"));
        assert!(Gnuciverba::check_word_ok("c", "_"));
        assert!(!Gnuciverba::check_word_ok("ciao", "___d"))
    }
    #[test]
    fn test_find_word(){
        let mut crossword = Gnuciverba::new(
            15, 15,
            &"ciao\nprova\npippo\nprovando\ntutto\nprove\nabc\n"
        );
        assert_eq!(crossword.find_words("ciao"), Some(vec!["ciao"]));
        assert_eq!(crossword.find_words("prov_"),Some(vec!["prova","prove"]));
        assert_eq!(crossword.find_words("___"), Some(vec!["abc"]));
        assert_eq!(crossword.find_words("world"), None);
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
}