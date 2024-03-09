use core::panic;
use std::{
    env, io::{self, Write}, process::{self}
};
use text_io::read;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use console;
use rayon::prelude::*;

mod game;
mod builtin_words;
mod args;
mod stats;
mod tui_mode;
mod gui;

use game::{Game, GuessWordStatus};
use stats::Stats;
use tui_mode::App;

use fltk::{app, button::Button, prelude::*, window::*};
use fltk::enums::FrameType;
use fltk::{enums::Color, input::Input};
use std::cell::RefCell;
use std::rc::Rc;
use std::process::exit;
use chrono::Local;
use std::thread;
use fltk::frame::Frame;
use chrono::Timelike;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();

    let mut word_processor: args::WordProcessor = args::WordProcessor::new();
    word_processor.process_args(&args);

    let random_mode: bool = word_processor.random_mode;
    let meet_word_argument: bool = word_processor.meet_word_argument;
    let mut day: usize = word_processor.day_argument.unwrap();
    let seed: u64 = word_processor.seed_argument.unwrap();
    let difficult_mode: bool = word_processor.difficult_mode;
    let word_argument: String = word_processor.word_argument.into_iter().collect();
    let stats_mode: bool = word_processor.stats_mode;
    let answer_list: Vec<String> = word_processor.final_set;
    let acceptable_list: Vec<String> = word_processor.acceptable_set;
    let state: Option<std::path::PathBuf> = word_processor.state_path;
    let state_mode: bool = word_processor.state_mode;
    let tui_mode: bool = word_processor.tui_mode;
    let gui_mode: bool = word_processor.gui_mode;
    let hint_mode: bool = word_processor.hint_mode;
    let test_mode: bool = word_processor.test_mode;

    let mut stats: Stats = if let Some(stats) = Stats::load(&state) {
        stats
    } else {
        panic!("Failed to load stats in json");
    };

    if !test_mode{
        if gui_mode {
            let app = app::App::default();
            let wind: Rc<RefCell<DoubleWindow>> = Rc::new(RefCell::new(DoubleWindow::new(0, 0, 800, 800, "Home Page")));
            wind.borrow_mut().set_pos(400, 0);
            wind.borrow_mut().set_color(Color::White);
            let input = Input::new(160, 220, 120, 30, "Name");
            
            let mut button = Button::new(160, 300, 120, 30, "Start");
            button.set_color(Color::rgb_color(156, 34, 24));
            button.set_frame(FrameType::FlatBox);
            button.set_label_size(20);
            button.set_label_color(Color::White);

            let mut frame = Frame::new(120, 420, 400, 30, "");
            frame.set_frame(FrameType::FlatBox);
            frame.set_color(Color::White);
            frame.set_label_size(24);
            frame.set_label_color(Color::Black);
            
            let mut frame_time = Frame::new(140, 480, 160, 50, "");
            frame_time.set_frame(FrameType::FlatBox);
            frame_time.set_color(Color::White);
            frame_time.set_label_size(24);
            frame_time.set_label_color(Color::Black);
            
            thread::spawn(move || {
                loop {
                    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();   
                    let current_hour = Local::now().hour();   
                    if (0..8).contains(&current_hour) || (23..25).contains(&current_hour) {
                        frame.set_label("Are you having trouble falling asleep? :/");
                    } else if (7 .. 9).contains(&current_hour) {
                        frame.set_label("You should have breakfast first :P");
                    } else if (9 .. 13).contains(&current_hour) {
                        frame.set_label("It's time to work :)");
                    } else if (13 .. 14).contains(&current_hour) {
                        frame.set_label("You should have lunch first :P");
                    } else if (14 .. 18).contains(&current_hour) {
                        frame.set_label("It's time to work :)");
                    } else if (18 .. 20).contains(&current_hour) {
                        frame.set_label("You should have dinner first :p");
                    } else {
                        frame.set_label("Take a break and go to bed early -_-zZ");
                    }
                    frame_time.set_label(&current_time);
                    thread::sleep(std::time::Duration::from_secs(1));
                }
            });
            let wind_clone = Rc::clone(&wind);
            button.set_callback(move |_| {
                let mut wind_clone = wind_clone.borrow_mut();
                wind_clone.hide();
            });
            let mut exit_button = Button::new(160, 360, 120, 30, "exit");
            exit_button.set_color(Color::rgb_color(106, 170, 100));
            exit_button.set_frame(FrameType::FlatBox);
            exit_button.set_label_size(20);
            exit_button.set_label_color(Color::White);
            exit_button.set_callback(move |_| {
                exit(0);
            });

            wind.borrow_mut().end();
            wind.borrow_mut().show();
            app.run().unwrap();
                
            let name: Rc<RefCell<Input>> = Rc::from(RefCell::from(input));
            let name: String = name.borrow().value();

            loop {
                let res: bool = gui::run_gui(name.clone(), answer_list.clone(), seed.clone(), day.clone(), difficult_mode.clone(), acceptable_list.clone());
                if !res {
                    break;
                }
                day = day + 1;            
            }
        }
        else if tui_mode {
            enable_raw_mode()?;
            let mut stdout = io::stdout();
            execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        
            let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(stdout);
            let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;

            let mut secret_word: String = String::new();

            let mut temp_app =  App::default(secret_word, difficult_mode);
            if !random_mode {
                if meet_word_argument {
                    secret_word = word_argument.to_uppercase();
                } else {
                    if let Ok(Some(word)) = App::input_answer(&mut terminal, &mut temp_app, &answer_list.clone()) {
                        secret_word = word;
                    } else {
                        panic!("Unexpected error");
                    }
                }
            } else {
                secret_word = Game::get_seed_random_word(&answer_list, seed, day);
            }
            let mut _game: Game = Game::new(secret_word.to_string(), difficult_mode, acceptable_list.clone(), acceptable_list.clone());
    
            // initialize
            let mut _app: App = App::default(secret_word, difficult_mode);
            
            // Muti game
            loop {
                // a round of game
                loop {
                    // one single guess
                    let mut _valid_word: Option<String> = None;
                    if let Ok(Some(word)) = App::run_app(&mut terminal, &mut _app, &acceptable_list, &mut _game, 0, false, false) {
                        _valid_word = Some(word.to_uppercase());
                    } else { 
                        panic!("Unexpected errors");
                    }
                    // game over
                    if let Some(word) = _valid_word {
                        // if game over
                        if _game.is_game_over(&word, false, false) {
                            stats.update(&_game.guesses, _game.answer.to_string(), _game.is_win);
                            // save game status in json
                            if state_mode {
                                stats.save();
                            }        
                            let frequent_words: Vec<(&String, &usize)> = stats.get_frequent_words();
                            let mut words: Vec<(String, usize)> = Vec::new();
                            for (_, (word, count)) in frequent_words.iter().take(5).enumerate() {
                                words.push((word.to_string(), **count));
                            }
                
                            _app.load_stats_message(stats.get_wins(), stats.get_fails(), stats.get_success_rate(), words);
                            if _game.is_win {
                                // if win
                                if let Ok(Some(_word)) = App::run_app(&mut terminal, &mut _app, &acceptable_list, &mut _game, 1, false, false) {
                                    // show statistic
                                    let _ = App::run_app(&mut terminal, &mut _app, &acceptable_list, &mut _game, 0, false, true);
                                    break;
                                } else {
                                    panic!("Unexpected errors");
                                }    
                            } else {
                                // if lose
                                if let Ok(Some(_word)) = App::run_app(&mut terminal, &mut _app, &acceptable_list, &mut _game, 2, false, false) {
                                    // show statistic
                                    let _ = App::run_app(&mut terminal, &mut _app, &acceptable_list, &mut _game, 0, false, true);
                                    break;
                                } else {
                                    panic!("Unexpected errors");
                                }
                            }
                        } else {
                            // keep guessing 
                            continue;
                        }
                    } else {
                        panic!("Unexpected errors");
                    }
                }
                // start next round
                if let Ok(Some(_word)) = App::run_app(&mut terminal, &mut _app, &acceptable_list, &mut _game, 0, true, false) {
                    if !random_mode && meet_word_argument {
                        break;
                    }
                    if _app.continue_to_play == true {
                        // continue to play
                        if !random_mode {
                            let _temp_secret_answer: String = String::new();
                            let mut temp_app = App::default(_temp_secret_answer, difficult_mode);
                            if let Ok(Some(word)) = App::input_answer(&mut terminal, &mut temp_app, &answer_list.clone()) {
                                secret_word = word;
                            } else {
                                panic!("Unexpected error");
                            }
                        } else {
                            day += 1;
                            secret_word = Game::get_seed_random_word(&answer_list, seed, day);
                        }
                        _game = Game::new(secret_word.clone(), difficult_mode, acceptable_list.clone(), acceptable_list.clone());
                        _app = App::default(secret_word, difficult_mode);
                        terminal.draw(|f|App::ui(f, &mut _app))?;
                        continue;

                    } else {
                        // exit the game
                        break;
                    }

                } else {
                    panic!("Unexpected errors");
                }
            }

            // restore terminal
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;
        }

        else {
            // automatically determine if it is in interactive mode
            let is_tty = atty::is(atty::Stream::Stdout);
            if is_tty {
                print!("{}", console::style("Your name: ").bold().red());
                io::stdout().flush().unwrap();
                let mut line = String::new();
                io::stdin().read_line(&mut line)?;
                println!("Welcome to wordle, {}!", line.trim());
            }

            let mut secret_word: String;

            // word pattern
            if random_mode == false {
                if meet_word_argument {
                    secret_word = word_argument.to_uppercase();
                } else {
                    if is_tty {
                        println!("Please type in the answer to start the game: ");
                    }
                    secret_word = input_ans(&answer_list, is_tty); 
                }
            } else { // random pattern
                secret_word = Game::get_seed_random_word(&answer_list, seed, day);
            }

            let mut game: Game = Game::new(secret_word.to_string(), difficult_mode, acceptable_list.clone(), acceptable_list.clone());

            if is_tty {
                println!("You have 6 chances to guess the word!");
                println!();
            }

            loop {
                let attempts = game.get_tries() + 1;
                if is_tty {
                    println!("ROUND{}:", attempts);
                    println!("Enter your guess: ");
                }

                let word: String = match game.ask_for_guess(&acceptable_list) {
                    Ok(word) => word,
                    Err(error) => {
                        println!("{}", error.print_error(is_tty));
                        continue;
                    }
                };

                let result: GuessWordStatus = game.play(&word);

                if is_tty {
                    game.print_guess_history();
                    if hint_mode {
                        if !game.is_game_over(&word, is_tty, false) {
                            let current_hint_list = game.get_hint(&word, result);
                            println!("Here are all possible words");
                            for hint in current_hint_list.clone(){
                                println!("{}", hint);
                            }
                            println!();
                            println!("Do you need recommendations for the most likely words?");
                            println!("<y> for yes, <n> for No");
                            let request: char = read!();
                            if request == 'y' || request == 'Y' {
                                let recommend_list: Vec<String> = game.get_recommend_words(&current_hint_list);
                                for (index, recommend_word) in recommend_list.iter().enumerate() {
                                    println!("{}: {}", index + 1, recommend_word);
                                }
                            } else {

                            }
                            
                            game.hint_list = current_hint_list;
                        }
                    }
                } else {
                    game.print_status_word(&word, &result);
                    game.print_status_alphabet();
                }

                if game.is_game_over(& word, is_tty, true) {
                    // update the game status so far
                    stats.update(&game.guesses, game.answer.to_string(), game.is_win);
                    if stats_mode {
                        stats.print_stats(is_tty);
                    }
                    if state_mode {
                        stats.save();
                    }
                    // continue in non specified answer mode
                    if !meet_word_argument {
                        if game.continue_to_play(is_tty) {
                            if random_mode {
                                loop {
                                    day += 1;
                                    secret_word = Game::get_seed_random_word(&answer_list, seed, day);
                                    game = Game::new(secret_word, difficult_mode, acceptable_list.clone(), acceptable_list.clone());
                                    break;
                                }
                            } else {
                                if is_tty{
                                    println!("Please type in the answer to start the game: ");
                                }
                                secret_word = input_ans(&acceptable_list, is_tty);
                                game = Game::new(secret_word, difficult_mode, acceptable_list.clone(), acceptable_list.clone());
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
        }
    } 
    // test mode: 
    // caculate all attempts for each answer in default final list and each guess word in available list
    else {
        let mut attempt_sequence: Vec<Vec<u64>> = Vec::new();
        for ans_word in answer_list {
            let attempts_for_one_answer: Vec<u64> = acceptable_list.clone().par_iter().map(|guess_word| {
                let mut game = Game::new(ans_word.clone(), false, acceptable_list.clone(), acceptable_list.clone());
                game.over_all_game_numbers(guess_word)
            }).collect();

        attempt_sequence.push(attempts_for_one_answer);
        }
        print_attempts_result(&attempt_sequence);
    }

    Ok(())
}

fn input_ans(word_list:&Vec<String>, is_tty: bool) -> String {
    loop {
        match Game::get_secret_word(&word_list) {
            Ok(word) => {
                return word;
            }
            Err(error) => {
                println!("{}", error.print_error(is_tty));
                continue;
            }
        } 
    }
}

// test mode printer
fn print_attempts_result(attemp_sequence: &Vec<Vec<u64>>) {
    let summed_attempt_sequence: Vec<u64> = attemp_sequence
        .par_iter() // par_iter()
        .map(|inner_vec| inner_vec.par_iter().sum())
        .collect();


    for (i, inner_vec) in attemp_sequence.iter().enumerate() {
        println!("\n--- word: {} ---", i + 1);
        for (index, element) in inner_vec.iter().enumerate() {
            if index % 80 == 0 && index != 0 {
                println!();
            }
            print!("{} ", element);
        }
        println!();
        let average: f64 = summed_attempt_sequence[i] as f64/ attemp_sequence[0].len() as f64;
        println!("average attempts: {}", average);        
    }
    println!();
    let sum: u64 = summed_attempt_sequence.par_iter().sum();

    let total_average: f64 = sum as f64 / (summed_attempt_sequence.len() as f64 * attemp_sequence[0].len() as f64); // calculate average

    println!("Total Average: {} {}",sum, total_average);
}
