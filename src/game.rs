use colored::Colorize;
use std::io::{self, Write};
use rand::prelude::*;
use rayon::prelude::*;


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LetterStatus { 
    Unknown,
    Red, 
    Yellow, 
    Green 
}

#[derive(Debug)]
pub enum ErrorType {
    WrongLength,
    GuessNotInList,
    AnsNotInList,
    HintUnused,
}

impl ErrorType {
    pub fn print_error(&self, is_tty: bool) -> String {
        if is_tty {
            match self {
                Self::WrongLength => format!("{}{}{}", "The length of a word should be ".red(), WORD_LENGTH.to_string().green(), ", please try another word!".red()),
                Self::GuessNotInList => format!("{}", "Not in the dictionary! Please try another word!".red()),
                Self::AnsNotInList => format!("{}", "Not in the dictionary! Please enter another answer!".red()),
                Self::HintUnused => String::from("You must use the hint in difficult mode."),
            }
        } else {
            match self {
                Self::WrongLength => String::from("INVALID"),
                Self::GuessNotInList => String::from("INVALID"),
                Self::AnsNotInList => String::from("INVALID"),
                Self::HintUnused => String::from("INVALID"),
            }
        }
    }
}

const WORD_LENGTH: usize = 5;
const ALPHABET_LENGTH: usize = 26;
const TRY_CASES: usize = 6;
const RECOMMEND_NUMBER: usize = 5;
pub type GuessWordStatus = [LetterStatus; WORD_LENGTH];

fn sanitize_word(word: &str) -> String {
    word.trim().to_uppercase().chars().filter(|c| c.is_ascii_alphabetic()).collect()
}

pub struct Game {
    difficult: bool,
    pub answer: String,
    pub alphabet: [LetterStatus; ALPHABET_LENGTH],
    pub guesses: Vec<(String, GuessWordStatus)>, // guessing history of all words in each game
    pub is_win: bool,
    pub hint_list: Vec<String>,
    pub test_list: Vec<String>,
}

impl Game {
    pub fn new(answer: String, difficult: bool, hint_list: Vec<String>, test_list: Vec<String>) -> Game {
        Game {
            difficult,
            answer,
            alphabet: [LetterStatus::Unknown; ALPHABET_LENGTH],
            guesses: Vec::new(),
            is_win: false,
            hint_list,
            test_list,
        }
    }

    pub fn get_guess_word_status(&self, word: &str) -> GuessWordStatus {
        let mut ans_counter: [i32; ALPHABET_LENGTH]= [0; ALPHABET_LENGTH];
        let mut corrected: [bool; WORD_LENGTH] = [false; WORD_LENGTH];
        let mut result: [LetterStatus; WORD_LENGTH] = [LetterStatus::Unknown; WORD_LENGTH];
        
        // letter number in answer
        for c in self.answer.chars() {
            let index: usize = (c as u8 - b'A') as usize;
            ans_counter[index] += 1;
        }

        // mark the correct letters in guessed word
        for (i, c) in word.chars().enumerate() {
            let answer_char: char = self.answer.chars().nth(i).unwrap();
            if answer_char == c {
                corrected[i] = true;
                let index: usize = (c as u8 - b'A') as usize;
                ans_counter[index] -= 1;
            }
        }

        // determine the state
        word.chars().enumerate().for_each(|(i, c)| {
            let index: usize = (c as u8 - b'A') as usize;
            result[i] = if corrected[i] {
                LetterStatus::Green
            } else if ans_counter[index] != 0 {
                ans_counter[index] -= 1;
                LetterStatus::Yellow
            } else {
                LetterStatus::Red
            };
        });

        result
    }

    // update alphabet and return word status
    pub fn play(&mut self, word: &str) -> GuessWordStatus {
        let status: GuessWordStatus = self.get_guess_word_status(word);
        for (i, character) in word.chars().enumerate() {
            let index: usize = (character as u8 - b'A') as usize;
            let updated_status: LetterStatus = status[i];

            self.alphabet[index] = self.alphabet[index].max(updated_status); //更新字母表，取最好
        }
        self.guesses.push((word.to_string().clone(), status));

        status
    }

    pub fn print_colored_alphabet(&self) {
        let result: String = self
            .alphabet
            .iter()
            .enumerate()
            .map(|(i, status)| {
                let c: char = (i as u8 + b'A') as char;
                match status {
                    &LetterStatus::Green => c.to_string().green().to_string(),
                    &LetterStatus::Red => c.to_string().red().to_string(),
                    &LetterStatus::Yellow => c.to_string().yellow().to_string(),
                    _ => c.to_string(),
                }
            })
            .collect();

        println!("{}", result);
    }

    pub fn print_status_alphabet(&self) {
        let result: String = self
            .alphabet
            .iter()
            .map(|status: &LetterStatus| match status {
                &LetterStatus::Green => "G".to_string(),
                &LetterStatus::Red => "R".to_string(),
                &LetterStatus::Yellow => "Y".to_string(),
                _ => "X".to_string(),
            })
            .collect();

        println!("{}", result);
    } 
    
    pub fn print_colored_word(&self, word: &str, result: &GuessWordStatus ) {
        let colored_word: String = word
            .chars()
            .enumerate()
            .map(|(i, c)| match result[i] {
                LetterStatus::Green => c.to_string().green().to_string(),
                LetterStatus::Red => c.to_string().red().to_string(),
                LetterStatus::Yellow => c.to_string().yellow().to_string(),
                _ => c.to_string(),
            })
            .collect();
        
        print!("{} ", colored_word);
    }

    pub fn print_status_word(&self, word: &str, result: &GuessWordStatus ) {
        let colored_word: String = word
            .chars()
            .enumerate()
            .map(|(i, _c)| match result[i] {
                LetterStatus::Green => "G",
                LetterStatus::Red =>"R",
                LetterStatus::Yellow => "Y",
                _ => "X",
            })
            .collect();

        print!("{} ", colored_word);
    }

    pub fn get_secret_word(word_list: &Vec<String>) -> Result<String, ErrorType> {
        loop {
            let mut secret_word: String = String::new();
            io::stdin().read_line(&mut secret_word).unwrap();
            secret_word = sanitize_word(&secret_word);
            if secret_word.len() != WORD_LENGTH {
                return Err(ErrorType::WrongLength);
            } else if !word_list.iter().any(|word| word == &secret_word) {
                return Err(ErrorType::AnsNotInList);
            } else {
                return Ok(secret_word);
            }
        }
    }

    pub fn ask_for_guess(&mut self, word_list: &Vec<String>) -> Result<String, ErrorType> {
        loop {
            let mut guess: String = String::new();
            io::stdin().read_line(&mut guess).unwrap();
            guess = sanitize_word(&guess);
            if guess.len() != WORD_LENGTH {
                return Err(ErrorType::WrongLength);
            }
            if !word_list.iter().any(|word| word == &guess) {
                return Err(ErrorType::GuessNotInList);
            } 
            if self.difficult {
                if !self.check_difficult_mode(&guess) {
                    return Err(ErrorType::HintUnused);
                }
            }
            return Ok(guess);
        }
    }
    
    // check if the input guess word is VALID
    pub fn input_valid_check(guess: &String, word_list: &Vec<String>) -> bool {
        if !word_list.iter().any(|word| &word == &guess) {
            return false;
        } 
        true
    }

    pub fn get_tries(&self) -> usize {
        self.guesses.len()
    }

    pub fn is_game_over(&mut self, word: &str, is_tty: bool, need_output: bool) -> bool { 
        let tries_number: usize = self.guesses.len();
        if word == self.answer {
            if need_output {
                if is_tty {
                    println!("You used {} chances and get the answer!", tries_number.to_string().green());
                } else {
                    println!("CORRECT {}", tries_number);   
                }  
            }
            self.is_win = true;
            true
        } else if tries_number >= TRY_CASES {
            if need_output {
                if is_tty {
                    println!("{} {} {}", "FAILED!".red(), "The answer is:" ,self.answer.green());
                } else {
                    println!("FAILED {}", self.answer);
                }   
            }
            true
        } else {
            false
        }
    }

    pub fn print_guess_history(&self) {
        for (word, result) in &self.guesses {
            self.print_colored_word(word, result);
            self.print_colored_alphabet();
        }
        println!();
    }

    // pseudo random word
    pub fn get_seed_random_word(word_list: &Vec<String>, seed: u64, day: usize) -> String {
        let mut rng: StdRng = StdRng::seed_from_u64(seed);
        let mut shuffled_word_list: Vec<&String> = word_list.iter().collect();

        shuffled_word_list.shuffle(&mut rng);

        let index: usize = day - 1;

        shuffled_word_list.get(index).unwrap().to_string()
    }

    pub fn check_difficult_mode(&self, word: &str) -> bool {
        if self.guesses.len() >= 1 {
            let last_guess: &(String, [LetterStatus; WORD_LENGTH]) = &self.guesses[self.guesses.len() - 1];
            let last_status: &[LetterStatus; WORD_LENGTH] = &last_guess.1;
    
            // green: can not change
            for (i, c) in word.chars().enumerate() {                
                let last_char_status = last_status[i];
                if last_char_status == LetterStatus::Green && last_guess.0.chars().nth(i).unwrap() != c {
                    return false
                }
            }
            // yellow: should contain
            for i in 0..ALPHABET_LENGTH {
                if self.alphabet[i] == LetterStatus::Yellow {
                    let c: char = (i as u8 + b'A') as char;
                    if !word.chars().any(|x| x== c) {
                        return false;
                    }
                }
            }
        }
    
        true
    }
    
    pub fn continue_to_play(&self, is_tty: bool) -> bool{
        loop {
            if is_tty {
                print!(
                    "Would you like to start a new game? {} ",
                    console::style("[Y/N]").bold().blue()
                );
            }
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let trimmed_input = input.trim().to_lowercase();
    
            match trimmed_input.as_str() {
                "y" => {
                    return true;
                }
                "n" => {
                    return false;
                }
                _ => {
                    continue;
                }
            }
        }
    }

    // give int
    pub fn get_hint(&mut self, last_guess_word: &str, last_guess_status: GuessWordStatus) -> Vec<String>{
        let mut temp_hint_list = self.hint_list.clone();
        let mut alphabet_counter: [u8; ALPHABET_LENGTH] = [0; ALPHABET_LENGTH]; // count green and yellow letter numbers
        let mut limited_number: [bool; ALPHABET_LENGTH] = [false; ALPHABET_LENGTH]; // if red letter appears

        // green: filter out words with the same letters in corresponding positions
        for (index, letter_status) in last_guess_status.iter().enumerate() { //index 0..5
            if letter_status == &LetterStatus::Green {
                let letter = last_guess_word.chars().nth(index).unwrap();
                let word_index: usize = (letter as u8 - b'A') as usize; //0..26
                alphabet_counter[word_index] += 1;
                temp_hint_list = temp_hint_list
                    .iter()
                    .filter(|word| word.chars().nth(index).unwrap() == letter)
                    .cloned()
                    .collect();
            }
            // initial screening for yellow and red charcter
            // check position
            if letter_status == &LetterStatus::Yellow {
                let letter = last_guess_word.chars().nth(index).unwrap();
                let word_index: usize = (letter as u8 - b'A') as usize;
                alphabet_counter[word_index] += 1;
                temp_hint_list = temp_hint_list
                    .iter()
                    .filter(|word| word.chars().nth(index).unwrap() != letter && word.contains(letter))
                    .cloned()
                    .collect();
            }
            if letter_status == &LetterStatus::Red {
                let letter = last_guess_word.chars().nth(index).unwrap();
                let word_index: usize = (letter as u8 - b'A') as usize;
                limited_number[word_index] = true;
                temp_hint_list = temp_hint_list
                    .iter()
                    .filter(|word| word.chars().nth(index).unwrap() != letter)
                    .cloned()
                    .collect();
            }
        }
        
        // check number
        temp_hint_list = temp_hint_list
            .iter()
            .filter(|word| self.hint_helper(word, limited_number, alphabet_counter))
            .cloned()
            .collect();

        temp_hint_list

    }

    // help check number
    fn hint_helper(&self, word: &str, limited_number: [bool; ALPHABET_LENGTH], alphabet_counter: [u8; ALPHABET_LENGTH]) -> bool{
        let mut word_counter: [u8; ALPHABET_LENGTH] = [0; ALPHABET_LENGTH];

        for c in word.chars() {
            let word_index: usize = (c as u8 - b'A') as usize;
            word_counter[word_index] += 1;
        }

        for c in word.chars() {
            let word_index: usize = (c as u8 - b'A') as usize;
            if limited_number[word_index] {
                if word_counter[word_index] == alphabet_counter[word_index] {
                    continue;
                } else {
                    return false;
                }
            } else {
                if word_counter[word_index] >= alphabet_counter[word_index] {
                    continue;
                }
                return false;

            }
        }
        true
    }

    fn compute_letter_weight(&self) -> Vec<(f64, f64, f64, f64, f64)>{
        // for all remaining words, count a-z's weighs in each positions 
        let mut count_helper: [[f64; ALPHABET_LENGTH]; WORD_LENGTH] = [[0.0; ALPHABET_LENGTH]; WORD_LENGTH];
        let mut count: Vec<(f64, f64, f64, f64, f64)> = Vec::new();
        let mut pos_sum: [u64; WORD_LENGTH] = [0; WORD_LENGTH];

        for word in self.hint_list.iter() {
            for (index, ch) in word.chars().into_iter().enumerate() {
                let word_index = (ch as u8 - b'A') as usize;
                count_helper[index][word_index] += 1 as f64;
                pos_sum[index] += 1;
            }
        }
        // normalization
        for i in 0..WORD_LENGTH{
            for j in 0..ALPHABET_LENGTH {
                if pos_sum[i] == 0 {
                    count_helper[i][j] = 0.0;
                } else {
                    count_helper[i][j] = count_helper[i][j] / pos_sum[i] as f64;
                }
            }
        }
        for i in 0..ALPHABET_LENGTH {
            count.push((count_helper[0][i], count_helper[1][i], count_helper[2][i], count_helper[3][i], count_helper[4][i]));
        }
        count

    }

    // give each remaining word in the list a score
    fn compute_next_guess_grade(&mut self, next_guess_word: &str) -> f64 {
        let count: Vec<(f64, f64, f64, f64, f64)> = self.compute_letter_weight();
        let mut grade: f64 = 0.0;

        let c0: char = next_guess_word.chars().nth(0).unwrap();
        let word_index: usize = (c0 as u8 - b'A') as usize;
        grade += count[word_index].0;

        let c1: char = next_guess_word.chars().nth(1).unwrap();
        let word_index: usize = (c1 as u8 - b'A') as usize;
        grade += count[word_index].1;

        let c2: char = next_guess_word.chars().nth(2).unwrap();
        let word_index: usize = (c2 as u8 - b'A') as usize;
        grade += count[word_index].2;

        let c3: char = next_guess_word.chars().nth(3).unwrap();
        let word_index: usize = (c3 as u8 - b'A') as usize;
        grade += count[word_index].3;

        let c4: char = next_guess_word.chars().nth(4).unwrap();
        let word_index: usize = (c4 as u8 - b'A') as usize;
        grade += count[word_index].4;

        grade
    }

    // get no more than 5 recommend words
    pub fn get_recommend_words(&mut self, hint_list: &Vec<String>) -> Vec<String> {
        let next_guess_grades: Vec<(String, f64)> = hint_list
            .iter()
            .map(|word: &String| (word.clone(), self.compute_next_guess_grade(word)))
            .collect();
    
        let mut sorted_grades = next_guess_grades.clone();
        sorted_grades.par_sort_by(|(_, grade1), (_, grade2)| grade2.partial_cmp(grade1).unwrap());
    
        let recommend_words = if self.hint_list.len() <= RECOMMEND_NUMBER {
            sorted_grades.iter().map(|(word, _)| word.clone()).collect()
        } else {
            sorted_grades
                .par_iter()
                .take(RECOMMEND_NUMBER)
                .map(|(word, _)| word.clone())
                .collect()
        };
    
        recommend_words
    }
    
    pub fn over_all_game_numbers(&mut self, word: &str) -> u64 {
        let mut num: u64 = 0;
        let status: [LetterStatus; WORD_LENGTH] = self.play(word);
        self.test_list = self.get_hint(word, status);
        num += 1;

        if word == self.answer {
            return num;
        }

        num += 1;

        while &self.get_recommend_words(&self.test_list.clone())[0] != &self.answer {
            let recommend_word = &self.get_recommend_words(&self.test_list.clone())[0];
            let status: [LetterStatus; WORD_LENGTH] = self.play(&recommend_word);
            self.test_list = self.get_hint(&recommend_word, status);
            self.hint_list = self.test_list.clone();
            num += 1;    
        }
        num
    }
    // is it related to information entropy? but it is an absolutely wrong idea
    // fn compute_next_guess_grade(&mut self, next_guess_word: &str) -> f64 {
    //     let next_status = self.get_guess_word_status(next_guess_word);
    //     let next_list: Vec<String> = self.get_hint(next_guess_word, next_status).clone();
    //     next_list.len() as f64 / self.hint_list.len() as f64
    // }
}
