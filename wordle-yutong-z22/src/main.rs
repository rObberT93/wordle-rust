use std::{
    io::{self, Write},
    process,
    env,
};

mod game;
mod builtin_words;
mod args;
mod stats;

use game::{Game, GuessWordStatus};
use stats::Stats;
//use std::collections::HashSet;
use console;


fn main() -> Result<(), Box<dyn std::error::Error>> {


    let args: Vec<String> = env::args().collect();

    let mut word_processor = args::WordProcessor::new();
    word_processor.process_args(&args);

    let random_mode = word_processor.random_mode;
    let meet_word_argument = word_processor.meet_word_argument;
    // let mut day: usize = 1;
    // let mut seed: u64 = 101;
    let mut day = word_processor.day_argument.unwrap();
    let seed = word_processor.seed_argument.unwrap();
    let difficult_mode: bool = word_processor.difficult_mode;
    let word_argument: String = word_processor.word_argument.into_iter().collect();
    let stats_mode: bool = word_processor.stats_mode;
    let answer_list: Vec<String> = word_processor.final_set;
    let acceptable_list: Vec<String> = word_processor.acceptable_set;
    let state = word_processor.state_path;
    let state_mode = word_processor.state_mode;

    // if is_tty {
    //     print!("{}", console::style("Your name: ").bold().red());
    //     io::stdout().flush().unwrap();
    // }

    // let mut line = String::new();
    // io::stdin().read_line(&mut line)?;
    //println!("Welcome to wordle, {}!", line.trim());

    let mut secret_word: String;

    if random_mode == false {
        if meet_word_argument {
            secret_word = word_argument.to_uppercase();
        } else {
            //println!("Please type in the answer to start the game: ");    
            secret_word = input_ans(&answer_list); 
        }
    } else {
        secret_word = Game::get_seed_random_word(&answer_list, seed, day);
    }

    //println!("{} {} {} {:?} {:?}", random_mode, seed, day, word_processor.final_set_file, word_processor.acceptable_set_file);


    let mut game: Game = Game::new(secret_word.to_string(), difficult_mode);

    let mut stats = if let Some(stats) = Stats::load(&state) {
        stats
    } else {
        panic!("Failed to load stats: 'state.json' broken\nYou should consider delete it.");
    };

    // println!("You have 6 chances to guess the word!");
    // println!();

    loop {
        //let attempts = game.get_tries() + 1;
        //println!("ROUND{}:", attempts);

        let word: String = match game.ask_for_guess(&acceptable_list) {
            Ok(word) => word,
            Err(error) => {
                println!("{}", error.print_error());
                continue; // 继续循环，等待重新输入单词
            }
        };

        let result: GuessWordStatus = game.play(&word);

        //game.print_colored_word(&word, &result);
        //game.print_colored_alphabet();
        game.print_status_word(&word, &result);
        game.print_status_alphabet();
        //game.print_guess_history();

        if game.is_game_over(& word) {
            //一次游戏结束更新一次到现在为止的游戏状态
            stats.update(&game.guesses, game.answer.to_string(), game.is_win);
            if stats_mode {
                stats.print_stats();
            }
            if state_mode {
                stats.save();
            }
            if !meet_word_argument { //没有指定答案
                if game.continue_to_play() {
                    if random_mode {
                        loop {
                            day += 1;
                            secret_word = Game::get_seed_random_word(&answer_list, seed, day);
                            game = Game::new(secret_word, difficult_mode);
                            break;
                        }
                    } else {
                        //println!("Please type in the answer to start the game: ");
                        secret_word = input_ans(&acceptable_list);
                        game = Game::new(secret_word, difficult_mode);
                    }
                } else {
                    process::exit(0);
                }
            }
            else {
                break;
            }
        }
    }

    Ok(())
}

fn input_ans(word_list:&Vec<String>) -> String {
    loop {
        match Game::get_secret_word(&word_list) {
            Ok(word) => {
                return word;
            }
            Err(error) => {
                println!("{}", error.print_error());
                continue;
            }
        } 
    }
}