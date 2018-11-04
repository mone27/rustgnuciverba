extern crate ndarray;
use ndarray::Array2;
use std::collections::HashMap;
use std::fmt;
extern crate rand;
use rand::Rng;

#[derive(Debug)]
pub struct Crossword {
    crossword: Array2<char>,
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
impl Crossword {
    fn new(numer_row: usize, number_col: usize) -> Self {
        Crossword {
            crossword: Array2::from_elem((numer_row, number_col), '_'),
        }
    }
    fn put_word(&mut self, word_position: &WordPos, word: impl AsRef<str>) -> (){
        let word = word.as_ref();
        for (c, pos) in word.chars().zip(0..word.len()) {
            self.crossword[match word_position.dir{
                Direction::Horizontal => word_position.row * self.n_columns + word_position.column + pos -1, //just a quick awful no sense quick fix
                Direction::Vertical =>
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

pub struct Dict {
    dict: HashMap<usize, Vec<String>>,
}

impl Dict {
    pub fn new(list_word: &str) -> Self {
        Dict {
            dict: {
                let mut map: HashMap<usize, Vec<String>> = HashMap::new();
                for line in list_word.lines() {
                    map.entry(line.len())
                        .or_insert(vec![])
                        .push(line.to_string());
                }
                map
            },
        }
    }
    pub fn find_random_word(&self, query_word: &str) -> Option<&str> {
        match self.find_words(query_word) {
            Some(words) => Some(rand::thread_rng().choose(&words).unwrap()),
            None => None,
        }
    }
    fn check_word_ok(word: &str, query_word: &str) -> bool {
        // warning! does not treat '#' char properly, input must be checked with Crossword::get_max_word_length
        for (c_new, c_old) in word.chars().zip(query_word.chars()) {
            if !(c_old == c_new || c_old == '_') {
                return false;
            }
        }
        true
    }
    fn find_words(&self, query_word: &str) -> Option<Vec<&str>> {
        match self.dict.get(&query_word.len()) {
            //NEED TO OPTIMISE exploiting the fact that the dict is alphabetically sorted
            Some(all_words) => {
                let list_words: Vec<&str> = all_words
                    .iter()
                    .filter(|word| Self::check_word_ok(word, query_word))
                    .map(AsRef::as_ref)
                    .collect();
                if list_words.len() == 0 {
                    None
                } else {
                    Some(list_words)
                }
            }
            None => None,
        }
    }
}
impl fmt::Display for Dict {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (len, vec) in &self.dict {
            writeln!(f, "\n\nlen: {} \n ", len)?;
            for w in vec {
                writeln!(f, "  {}", w)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn run_test_dict<T>(test: T) -> ()
    where
        T: FnOnce(Dict) -> (),
    {
        let dict = Dict::new("ciao\nprova\npippo\nprovando\ntutto\nprove\nabc\n");
        test(dict);
    }

    #[test]
    fn test_check_word_ok() {
        assert!(Dict::check_word_ok("ciao", "ciao"));
        assert!(Dict::check_word_ok("ciao", "c___"));
        assert_ne!(Dict::check_word_ok("pluto", "pippo"), true);
        assert!(!Dict::check_word_ok("ciao", "pippo"));
        assert!(Dict::check_word_ok("c", "_"));
        assert!(!Dict::check_word_ok("ciao", "___d"))
    }
    #[test]
    fn test_find_word() {
        run_test_dict(|dict| {
            assert_eq!(dict.find_words("ciao"), Some(vec!["ciao"]));
            assert_eq!(dict.find_words("prov_"), Some(vec!["prova", "prove"]));
            assert_eq!(dict.find_words("___"), Some(vec!["abc"]));
            assert_eq!(dict.find_words("world"), None);
        });
    }
    // Crossword sect
    fn run_test_cross<T>(test: T) -> ()
    where
        T: FnOnce(Crossword) -> (),
    {
        let cross = Crossword::new(10, 10);
        test(cross);
    }
    #[test]
    fn test_crossword_creation() {
        run_test_cross(|cross| println!("{:?}", cross))
    }
}
