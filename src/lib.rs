// use ndarray::s;
#[macro_use]
extern crate ndarray;
use ndarray::Array2;

use std::collections::HashMap;
use std::fmt;
extern crate rand;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Crossword {
    crossword: Array2<char>,
}
#[derive(Copy, Clone, Debug)]
enum Direction {
    Horizontal,
    Vertical,
}

impl std::ops::Not for Direction {
    type Output = Direction;
    fn not(self) -> Direction {
        match self {
            Direction::Vertical => Direction::Horizontal,
            Direction::Horizontal => Direction::Vertical,
        }
    }
}
#[derive(Copy, Clone, Debug)]
struct WordPos {
    row: usize,
    col: usize,
    dir: Direction,
}
impl Crossword {
    pub fn new(numer_row: usize, number_col: usize) -> Self {
        Crossword {
            crossword: Array2::from_elem((numer_row, number_col), '_'),
        }
    }
    // insert a word in the crossword at the given wordpos
    fn put_word(&mut self, word_position: &WordPos, word: impl AsRef<str>) -> () {
        //may be more efficient
        let word = word.as_ref();
        for (old, new) in match word_position.dir {
            Direction::Horizontal => self.crossword.slice_mut(s![
                word_position.row,
                word_position.col..(word_position.col + word.len())
            ]),
            Direction::Vertical => self.crossword.slice_mut(s![
                word_position.row..(word_position.row + word.len()),
                word_position.col
            ]),
        }
        .iter_mut()
        .zip(word.chars())
        {
            *old = new; // replace the char
        }
    }

    fn get_word(&mut self, word_position: &WordPos, len: usize) -> String {
        // TODO merge get_word and get_max_length for performance reasons?
        let mut final_string = String::new(); //performace tip can preallocate the new string since the lenght in known
        for i in match word_position.dir {
            Direction::Horizontal => self.crossword.slice(s![
                word_position.row,
                word_position.col..(word_position.col + len)
            ]),
            Direction::Vertical => self.crossword.slice(s![
                word_position.row..(word_position.row + len),
                word_position.col
            ]),
        } {
            final_string.push(*i)
        }
        final_string
    }
    //     fn get_max_word_length(&self, word_position: &WordPos) -> usize { //not sure good way to handle 0 len case
    //         match word_position.dir {
    //             Direction::Horizontal => {

    //             //     self.crossword.slice(

    //             //         // .position(|&c|c == '#')

    //             0

    //             } ,
    //             Direction::Vertical => {
    //                 let start_pos = word_position.row * self.n_columns + word_position.column;
    //                 self.crossword.iter()
    //                     .skip(start_pos)
    //                     .step_by(self.n_columns)
    //                    .position(|&c|c == '#')
    //                     .unwrap_or(self.n_rows - word_position.row)
    //             }
    //        }
    //    }
}
impl fmt::Display for Crossword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // removes the square brakets from the default crossword string
        write!(
            f,
            " {}",
            self.crossword
                .to_string()
                .chars()
                .filter(|&c| c != ']' && c != '[')
                .collect::<String>()
        );
        Ok(())
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
            Some(words) => {
                let mut rng = rand::thread_rng();
                words.choose(&mut rng).copied()
            }
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
    #[test]
    fn test_put_word_horizontal() {
        run_test_cross(|mut cross| {
            cross.put_word(
                &WordPos {
                    row: 0,
                    col: 0,
                    dir: Direction::Horizontal,
                },
                "ciao",
            );
            println!("{:?}", cross);
        });
    }
    #[test]
    fn test_word_replace() {
        let mut cross = Crossword::new(10, 10);
        cross.put_word(
            &WordPos {
                row: 0,
                col: 0,
                dir: Direction::Horizontal,
            },
            "ciao",
        );
        cross.put_word(
            &WordPos {
                row: 0,
                col: 3,
                dir: Direction::Horizontal,
            },
            "mondo",
        );
        println!("{:?}", cross);
    }
    #[test]
    fn test_put_word_vertical() {
        let mut cross = Crossword::new(10, 10);
        cross.put_word(
            &WordPos {
                row: 0,
                col: 7,
                dir: Direction::Vertical,
            },
            "verticale",
        );
        println!("{:?}", cross);
        println!("test");
    }
    #[test]
    fn test_get_word() {
        let mut cross = Crossword::new(10, 10);
        cross.put_word(
            &WordPos {
                row: 0,
                col: 0,
                dir: Direction::Horizontal,
            },
            "ciao",
        );
        println!(
            "{}",
            cross.get_word(
                &WordPos {
                    row: 0,
                    col: 0,
                    dir: Direction::Horizontal
                },
                4
            )
        );
        assert_eq!(
            cross.get_word(
                &WordPos {
                    row: 0,
                    col: 0,
                    dir: Direction::Horizontal
                },
                4
            ),
            "ciao"
        );
    }
}
