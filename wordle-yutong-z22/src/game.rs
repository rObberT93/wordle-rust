use colored::Colorize;
use std::{
    io::{self, BufRead, Write}, collections::HashSet,
};
use rand::prelude::*;


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
// impl ErrorType {
//     pub fn print_error(&self) -> String{
//         match self {
//             Self::WrongLength => format!("{}{}{}", "The length of a word should be ".red(), WORD_LENGTH.to_string().green(), ", please try another word!".red()),
//             Self::GuessNotInList => format!("{}", "Not in the dictionary! Please try another word!".red()),
//             Self::AnsNotInList => format!("{}", "Not in the dictionary! Please enter another answer!".red()),
//             Self::HintUnused => String::from("You must use the hint in difficult mode."),
//         }
//     }
// }
impl ErrorType {
    pub fn print_error(&self) -> String {
        match self {
            Self::WrongLength => String::from("INVALID"),
            Self::GuessNotInList => String::from("INVALID"),
            Self::AnsNotInList => String::from("INVALID"),
            Self::HintUnused => String::from("INVALID"),
        }
    }
}

const WORD_LENGTH: usize = 5;
const ALPHABET_LENGTH: usize = 26;
const TRY_CASES: usize = 6;
pub type GuessWordStatus = [LetterStatus; WORD_LENGTH];

fn sanitize_word(word: &str) -> String {
    word.trim().to_uppercase().chars().filter(|c| c.is_ascii_alphabetic()).collect()
}

pub struct Game {
    difficult: bool,
    pub answer: String,
    alphabet: [LetterStatus; ALPHABET_LENGTH],
    pub guesses: Vec<(String, GuessWordStatus)>, //存储每一局所有单词的猜测历史记录
    pub is_win: bool,
}

impl Game {
    pub fn new(answer: String, difficult: bool) -> Game {
        Game {
            difficult,
            answer,
            alphabet: [LetterStatus::Unknown; ALPHABET_LENGTH],
            guesses: Vec::new(),
            is_win: false,
        }
    }

    fn get_guess_word_status(&self, word: &str) -> GuessWordStatus {
        let mut ans_counter: [i32; ALPHABET_LENGTH]= [0; ALPHABET_LENGTH];
        let mut corrected: [bool; WORD_LENGTH] = [false; WORD_LENGTH];
        let mut result: [LetterStatus; WORD_LENGTH] = [LetterStatus::Unknown; WORD_LENGTH];
        
        // 计算答案中每个字母的数量
        for c in self.answer.chars() {
            let index: usize = (c as u8 - b'A') as usize;
            ans_counter[index] += 1;
        }

        // 检查猜测的单词，标记正确的字母，并更新答案中字母的数量
        for (i, c) in word.chars().enumerate() {
            let answer_char: char = self.answer.chars().nth(i).unwrap();
            if answer_char == c {
                corrected[i] = true;
                let index: usize = (c as u8 - b'A') as usize;
                ans_counter[index] -= 1;
            }
        }

        // 根据猜测的单词的每个字母，确定字母的状态
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

    // 一次猜测更新一次字母表
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
        //println!("Enter your guess: ");
        loop {
            let mut guess: String = String::new();
            io::stdin().read_line(&mut guess).unwrap();
            guess = sanitize_word(&guess);
            if guess.len() != WORD_LENGTH {
                return Err(ErrorType::WrongLength);
            } else if !word_list.iter().any(|word| word == &guess) {
                return Err(ErrorType::GuessNotInList);
            } else if self.difficult {
                // if let Err(error) = self.check_difficult_mode(&guess) {
                //     return Err(error);
                // }
                if !self.check_difficult_mode(&guess) {
                    return Err(ErrorType::HintUnused);
                }
            }
            return Ok(guess);
        }
    }    

    pub fn get_tries(&self) -> usize {
        self.guesses.len()
    }

    pub fn is_game_over(&mut self, word: &str) -> bool { 
        let tries_number: usize = self.guesses.len();
        if word == self.answer {
            // println!("You used {} chances and get the answer!", tries_number.to_string().green());
            println!("CORRECT {}", tries_number);
            self.is_win = true;
            true
        } else if tries_number >= TRY_CASES {
            // println!("{} {} {}", "FAILED!".red(), "The answer is:" ,self.answer.green());
            println!("FAILED {}", self.answer);
            true
        } else {
            false
        }
    }

    pub fn print_guess_history(&self) {
        for (word, result) in &self.guesses {
            self.print_colored_word(word, result);
            print!(" ");
            self.print_colored_alphabet();
        }
        println!();
    }

    pub fn get_seed_random_word(word_list: &Vec<String>, seed: u64, day: usize) -> String {
        let mut rng: StdRng = StdRng::seed_from_u64(seed);
        let mut shuffled_word_list: Vec<&String> = word_list.iter().collect();

        shuffled_word_list.shuffle(&mut rng);

        let index = day - 1;
        
        shuffled_word_list.get(index).unwrap().to_string()
    }

    pub fn check_difficult_mode(&self, word: &str) -> bool {
        if self.guesses.len() >= 1 {
            let last_guess: &(String, [LetterStatus; 5]) = &self.guesses[self.guesses.len() - 1];
            let last_status: &[LetterStatus; 5] = &last_guess.1;
    
            for (i, c) in word.chars().enumerate() {                
                let last_char_status = last_status[i];
                if last_char_status == LetterStatus::Green && last_guess.0.chars().nth(i).unwrap() != c {
                    return false
                }
            }
            for i in 0..26 {
                if self.alphabet[i] == LetterStatus::Yellow {
                    let c: char = (i as u8 + b'A') as char;
                    if !word.chars().any(|x| x== c) {
                        return false;
                    }
                }
            }
        }
    
        return true;
    }
    
    pub fn continue_to_play(&self) -> bool{
        loop {
            // print!(
            //     "Would you like to start a new game? {} ",
            //     console::style("[Y/N]").bold().blue()
            // );
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
}
