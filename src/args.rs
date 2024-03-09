use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    vec,
    fs,
};
use serde::Deserialize;

use super::builtin_words;

const WORD_LENGTH: usize = 5;
#[derive(Deserialize)]
pub struct WordProcessor {
    pub random_mode: bool, // random pattern
    word_mode: bool, // specify words
    pub meet_word_argument: bool, // if cmd determines word
    pub difficult_mode: bool, // difficult pattern
    pub stats_mode: bool, // output status after game ends
    pub word_argument: Option<String>, 
    seed_mode: bool, // if assigned seed
    pub seed_argument: Option<u64>, 
    day_mode: bool, // if assigned day
    pub day_argument: Option<usize>, 
    pub final_set_file: Option<String>, //final vocabulary file name
    pub acceptable_set_file: Option<String>, // acceptable vocabulary file name
    pub final_set: Vec<String>, // final vocabulary
    pub acceptable_set: Vec<String>, // acceptable vocabulary
    pub state_mode: bool, // save status
    pub state_path: Option<PathBuf>, // json path
    pub config_mode: bool, // load argument
    pub config_path: Option<PathBuf>, // json path
    pub tui_mode: bool, // start TUI
    pub gui_mode: bool, // start GUI
    pub hint_mode: bool, // need hint
    pub test_mode: bool, // test attempts numbers
}

#[derive(Deserialize)]
struct Config {
    random: bool, 
    difficult: bool,
    stats: bool,
    day: Option<usize>,
    seed: Option<u64>,
    final_set: Option<String>,
    acceptable_set: Option<String>,
    state: Option<PathBuf>,
    word: Option<String>,
}

impl WordProcessor {
    pub fn new() -> Self {
        WordProcessor {
            random_mode: false,
            word_mode: false,
            meet_word_argument: false,
            difficult_mode: false,
            stats_mode: false,
            word_argument: None,
            seed_mode: false,
            seed_argument: None,
            day_mode: false,
            day_argument: None,
            final_set_file: Some("builtin_word.rs".to_string()), // default
            acceptable_set_file: Some("builtin_word.rs".to_string()), //default
            final_set: vec![],
            acceptable_set: vec![],
            state_mode: false,
            state_path: None,
            config_mode: false,
            config_path: None,
            tui_mode: false,
            gui_mode: false,
            hint_mode: false,
            test_mode: false,
        }
    }


    pub fn process_args(&mut self, args: &[String]) {
        self.final_set = get_default_answers_list();
        self.acceptable_set = get_default_accept_list();
        self.seed_argument = Some(101);
        self.day_argument = Some(1);

        if args.iter().any(|arg| arg == "-c" || arg == "--config") {
            self.config_mode = true;
        }

        if let Some(index) = args.iter().position(|arg| arg == "-c" || arg == "--config") {
            if index + 1 < args.len() {
                let next_argument = &args[index + 1];
                if let Ok(path) = next_argument.parse::<PathBuf>() {
                    self.load_config_from_file(&Some(path.clone()));

                    self.config_path = Some(path);
                }
            }
        }

        if args.iter().any(|arg| arg == "-w" || arg == "--word") {
            self.random_mode = false;
            self.word_mode = true;

            if let Some(index) = args.iter().position(|arg| arg == "-w" || arg == "--word") {
                if index + 1 < args.len() {
                    let next_argument = &args[index + 1];
                    if next_argument.chars().next().unwrap().is_alphabetic() {
                        self.word_argument = Some(next_argument.to_string());
                        self.meet_word_argument = true;
                    }
                }
            }
        }

        if args.iter().any(|arg| arg == "-r" || arg == "--random") {
            self.random_mode = true;
        }

        if args.iter().any(|arg| arg == "-D" || arg == "--difficult") {
            self.difficult_mode = true;
        }

        if args.iter().any(|arg| arg == "-t" || arg == "--stats") {
            if self.meet_word_argument {
                panic!("Tht -t/--stats option is not allowed when a word has been designated!");
            } else {
                self.stats_mode = true;
            }
        }

        if let Some(index) = args.iter().position(|arg| arg == "-f" || arg == "--final-set") {
            if index + 1 < args.len() {
                let file_name = &args[index + 1];
                if file_name.contains('.') {
                    self.load_answer_list(file_name);

                    self.final_set_file = Some(args[index + 1].to_string());
                } else {
                    println!("Ignoring invalid file name: {}, use default answer list!", file_name)
                }
            }
        }    
        
        if let Some(index) = args.iter().position(|arg| arg == "-a" || arg == "--acceptable-set") {
            if index + 1 < args.len() {
                let file_name = &args[index + 1];
                if file_name.contains('.') {
                    self.load_accept_list(file_name);
                    self.acceptable_set_file = Some(args[index + 1].to_string());
                } else {
                    println!("Ignoring invalid file name: {}, use default answer list!", file_name)
                }
            }
        }

        self.check_sets(&self.final_set, &self.acceptable_set); // final set must be strictly a subset of the acceptable list

        if let Some(index) = args.iter().position(|arg| arg == "-s" || arg == "--seed") {
            self.seed_mode = true;
            if index + 1 < args.len() {
                let next_argument = &args[index + 1];
                if let Ok(seed) = next_argument.parse::<u64>() {
                    self.seed_argument = Some(seed);
                }
            }
        }

        if let Some(index) = args.iter().position(|arg| arg == "-d" || arg == "--day") {
            self.day_mode = true;
            if index + 1 < args.len() {
                let next_argument = &args[index + 1];
                if next_argument.contains("-") {
                    panic!("Invalid value for -d/--day option!");
                }
                if let Ok(day) = next_argument.parse::<usize>() {
                    self.day_argument = Some(day);
                    if !self.validate_day() {
                        panic!("Invalid value for -d/--day option!");
                    }
                }
            }
        }

        if self.random_mode && self.word_mode {
            panic!("The -w/--word option is not allowed in random mode!");
        }

        if !self.random_mode && (self.day_mode|| self.seed_mode) {
            panic!("Please use -d/--day or -s/--seed options in random mode!");
        }
        
        if args.iter().any(|arg| arg == "-S" || arg == "--state") {
            self.state_mode = true;
        }

        if let Some(index) = args.iter().position(|arg| arg == "-S" || arg == "--state") {
            if index + 1 < args.len() {
                let next_argument = &args[index + 1];
                if let Ok(state) = next_argument.parse::<PathBuf>() {
                    self.state_path = Some(state);
                }
            }
        }
        if args.iter().any(|arg| arg == "-T" || arg == "--tui") {
            self.tui_mode = true;
        }
        
        if args.iter().any(|arg| arg == "-G" || arg == "--gui") {
            self.gui_mode = true;
        }

        if args.iter().any(|arg| arg == "-H" || arg == "--hint") {
            self.hint_mode = true;
        }

        if args.iter().any(|arg| arg == "--test") {
            self.test_mode = true;
        }
    }

    pub fn load_answer_list(&mut self, file_name: &str) {
        if let Ok(file) = File::open(file_name) { // open file
            self.final_set.clear(); 
            let reader: BufReader<File> = BufReader::new(file); // read line by line
            for line in reader.lines() {
                if let Ok(word) = line {
                    // check if there are multiple words in a line
                    let words_in_a_line = word.split_whitespace().collect::<Vec<&str>>();
                    if words_in_a_line.len() > 1 {
                        panic!("There can be only one word in a line!");
                    } else if word.len() != WORD_LENGTH {
                        panic!("Each word should be 5 in length!");
                    } else if self.final_set.contains(&word.trim().to_owned().to_uppercase()) {
                        panic!("Duplicated!");
                    }
                    self.final_set.push(word.trim().to_owned().to_uppercase());
                }
            }
        } else {
            panic!("Failed to open file: {}", file_name);
        }
    }

    pub fn load_accept_list(&mut self, file_name: &str) {
        if let Ok(file) = File::open(file_name) {
            self.acceptable_set.clear();
            let reader: BufReader<File> = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(word) = line {
                    let words_in_a_line = word.split_whitespace().collect::<Vec<&str>>();
                    // check multi words in a line
                    if words_in_a_line.len() > 1 {
                        panic!("There can be only one word in a line!");
                    } else if word.len() != WORD_LENGTH{
                        panic!("Each word should be 5 in length!");
                    } else if self.acceptable_set.contains(&word.trim().to_owned().to_uppercase()) {
                        panic!("Duplicated!");
                    }
                    self.acceptable_set.push(word.trim().to_owned().to_uppercase());
                }
            }
            self.acceptable_set.sort();
        } else {
            panic!("Failed to open file: {}", file_name);
        }
    }

    pub fn check_sets(&self, final_set: &Vec<String>, acceptable_set: &Vec<String>) {
        for word in final_set {
            if !acceptable_set.contains(word) {
                panic!("acceptable_set does not include the word: {}!", word);
            }
        }
    }

    fn validate_day(&self) -> bool {
        if self.day_argument.unwrap() == 0 {
            return false;
        } else if self.day_argument.unwrap() > self.final_set.len(){
            return false;
        }
        return true;
    }

    fn load_config_from_file(&mut self, config_path: &Option<PathBuf>) {
        if PathBuf::from(config_path.as_ref().unwrap()).exists() {
            if let Ok(config) = serde_json::from_str::<Config>(
                fs::read_to_string(config_path.as_ref().unwrap())
                    .unwrap()
                    .as_str()
            ){
                self.random_mode = config.random;
                self.difficult_mode = config.difficult;
                self.stats_mode = config.stats;
                self.day_argument = config.day;
                self.seed_argument = config.seed;
                self.final_set_file = config.final_set;
                self.acceptable_set_file = config.acceptable_set;
                self.state_path = config.state;
                self.word_argument = config.word;
                match self.word_argument {
                    Some(_) => {
                        self.random_mode = false;
                    },
                    None => {
                        self.random_mode = true;
                    }
                }
            } else {
                Self::new();
            }
        } else {
            panic!("Config file doesn't exist!");
        }
    } 
    
}    

fn get_default_answers_list() -> Vec<String> {
    let answer_list: Vec<String> = {
        builtin_words::FINAL
            .iter()
            .map(|s| s.to_uppercase())
            .collect()
    };

    answer_list
}

fn get_default_accept_list() -> Vec<String> {
    let accept_list: Vec<String> = {
        builtin_words::ACCEPTABLE
            .iter()
            .map(|s| s.to_uppercase())
            .collect()
    };

    accept_list
}