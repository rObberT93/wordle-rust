use fltk::{app, button::Button, frame::{*}, prelude::*, window::*};
use fltk::enums::FrameType;
use fltk::enums::Color;
use std::{cell::RefCell, process::exit};
use std::rc::Rc;
use crate::game::{Game, GuessWordStatus, LetterStatus};
use fltk::dialog::alert;
use std::collections::HashMap;

// a round of game(max attempts = 6)
pub fn run_gui(name: String,  answer_list: Vec<String>, seed: u64, day: usize, difficult_mode: bool, acceptable_list: Vec<String>) -> bool{
    let letter_to_number: HashMap<char, (usize, usize)> = create_letter_to_number_map();
    let app = app::App::default();
    let wind: Rc<RefCell<DoubleWindow>> = Rc::new(RefCell::new(DoubleWindow::new(0, 0, 800, 800, "")));
    
    wind.borrow_mut().set_pos(400, 0);
    let wind_clone: Rc<RefCell<DoubleWindow>> = Rc::clone(&wind);
    wind_clone.borrow_mut().set_color(Color::White);
    wind_clone.borrow_mut().set_label(&format!("Wordle in Rust - Hello, {}!", name));  

    let mut exit_button = Button::new(600, 410, 80, 40, "exit");
    exit_button.set_color(Color::rgb_color(106, 170, 100)); 
    exit_button.set_frame(FrameType::FlatBox); 
    exit_button.set_label_size(18); 
    exit_button.set_label_color(Color::White); 

    
    exit_button.set_callback(move |_| {
        exit(0);
    });

    let secret_word: String;
    secret_word = Game::get_seed_random_word(&answer_list, seed, day);

    let mut game: Game = Game::new(secret_word.to_string(), difficult_mode, acceptable_list.clone(), acceptable_list.clone());
    
    let attempts: Rc<RefCell<usize>> = Rc::new(RefCell::new(game.get_tries()));        
    println!("Secret word: {}", secret_word);
    println!("attempts: {}", attempts.borrow());

    let mut x: i32;
    let mut y = 50;

    let mut vec_frames: Vec<Vec<Frame>> = Vec::new();
    for _ in 0..6 {
        x = 230;
        let mut frames: Vec<Frame> = Vec::new();
        for _ in 0..5 {
            let mut frame = Frame::new(x, y, 50, 50, "");
            frame.set_frame(FrameType::UpBox);
            frame.set_label_size(20);
            frame.set_label_color(Color::Black);
            frame.set_color(Color::White); 
            frames.push(frame);
            x += 60;
        }
        vec_frames.push(frames);
        y += 60;

    }

    let selected_letters = Rc::new(RefCell::new(String::new()));

    let mut vec_buttons: Vec<Vec<Button>> = Vec::new();
    
    let mut buttons: Vec<Button> = Vec::new(); 
    let mut x = 50;
    let mut y = 550;

    for c in ['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'].iter() {
        let mut button = Button::new(x, y, 50, 50, c.to_string().as_str());
        button.set_frame(FrameType::RFlatBox);
        button.set_label_size(20);
        button.set_label_color(Color::Black);
        button.set_color(Color::from_rgb(211, 214, 218));

        let selected_letters_clone: Rc<RefCell<String>> = Rc::clone(&selected_letters);
        let vec_frames_clone: Rc<RefCell<Vec<Vec<Frame>>>> = Rc::clone(&Rc::new(RefCell::new(vec_frames.clone())));
        let attempts_clone: Rc<RefCell<usize>> = Rc::clone(&attempts);
        button.set_callback(move |_| {
            let mut selected_letters: std::cell::RefMut<'_, String> = selected_letters_clone.borrow_mut();
            if selected_letters.len() < 5 {
                selected_letters.push(*c);
                println!("{}", selected_letters.clone());
                let mut frames: std::cell::RefMut<'_, Vec<Vec<Frame>>> = vec_frames_clone.borrow_mut();
                let attempts_clone: std::cell::Ref<'_, usize> = attempts_clone.borrow();
                frames[*attempts_clone][selected_letters.len() - 1].set_label(c.to_string().as_str());
            }
        });
        buttons.push(button);
        x += 70; 
    }
    vec_buttons.push(buttons);
    y += 70; 
    x = 90;

    buttons = Vec::new();
    for c in ['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'].iter() {
        let mut button = Button::new(x, y, 50, 50, c.to_string().as_str());
        button.set_frame(FrameType::RFlatBox);
        button.set_label_size(20);
        button.set_label_color(Color::Black);
        button.set_color(Color::from_rgb(211, 214, 218));
        let selected_letters_clone = Rc::clone(&selected_letters);
        let vec_frames_clone: Rc<RefCell<Vec<Vec<Frame>>>> = Rc::clone(&Rc::new(RefCell::new(vec_frames.clone())));
        let attempts_clone: Rc<RefCell<usize>> = Rc::clone(&attempts);
        button.set_callback(move |_| {
            let mut selected_letters = selected_letters_clone.borrow_mut();
            if selected_letters.len() < 5 {
                selected_letters.push(*c);
                println!("{}", selected_letters.clone());
                let mut frames: std::cell::RefMut<'_, Vec<Vec<Frame>>> = vec_frames_clone.borrow_mut();
                let attempts_clone: std::cell::Ref<'_, usize> = attempts_clone.borrow();
                frames[*attempts_clone][selected_letters.len() - 1].set_label(c.to_string().as_str());
            }
        });
        buttons.push(button);
        x += 70; 
    }

    vec_buttons.push(buttons);  
    y += 70;

    let mut buttons: Vec<Button> = Vec::new();
    let mut x = 160;
    for c in ['Z', 'X', 'C', 'V', 'B', 'N', 'M'].iter() {
        let mut button = Button::new(x, y, 50, 50, c.to_string().as_str());
        button.set_frame(FrameType::RFlatBox);
        button.set_label_size(20);
        button.set_label_color(Color::Black);
        button.set_color(Color::from_rgb(211, 214, 218));
        let selected_letters_clone = Rc::clone(&selected_letters);
        let vec_frames_clone = Rc::clone(&Rc::new(RefCell::new(vec_frames.clone())));
        let attempts_clone = Rc::clone(&attempts);
        button.set_callback(move |_| {
            let mut selected_letters = selected_letters_clone.borrow_mut();
            if selected_letters.len() < 5 {
                selected_letters.push(*c);
                println!("{}", selected_letters.clone());
                let mut frames = vec_frames_clone.borrow_mut();
                let attempts_clone = attempts_clone.borrow();
                frames[*attempts_clone][selected_letters.len() - 1].set_label(c.to_string().as_str());
            }
        });
        buttons.push(button);
        x += 70; 
    }
    vec_buttons.push(buttons);

    let mut enter_button: Button = Button::new(60, y, 80, 50, "ENTER");
    enter_button.set_frame(FrameType::RFlatBox);
    enter_button.set_label_size(20);
    enter_button.set_label_color(Color::Black);
    enter_button.set_color(Color::from_rgb(211, 214, 218));
    let selected_letters_clone: Rc<RefCell<String>> = Rc::clone(&selected_letters);
    let attempts_clone: Rc<RefCell<usize>> = Rc::clone(&attempts);
    let vec_frames_clone: Rc<RefCell<Vec<Vec<Frame>>>> = Rc::clone(&Rc::new(RefCell::new(vec_frames.clone())));
    let vec_buttons_clone: Rc<RefCell<Vec<Vec<Button>>>> = Rc::clone(&Rc::new(RefCell::new(vec_buttons.clone())));
    
    let wind_clone_for_restart: Rc<RefCell<DoubleWindow>> = Rc::clone(&wind);

    enter_button.set_callback(move |_| {
        let mut selected_letters = selected_letters_clone.borrow_mut();
        if selected_letters.len() == 5 {
            if !Game::input_valid_check(&selected_letters.to_uppercase(), &acceptable_list.clone()) {
                alert(550, 200, "Invalid");
            } 
            
            else { //valid word
                let result: GuessWordStatus = game.play(&selected_letters.to_uppercase());
                let mut attempts_clone: std::cell::RefMut<'_, usize> = attempts_clone.borrow_mut(); 
                *attempts_clone += 1;
                if *attempts_clone == 6 || selected_letters.to_uppercase() == secret_word.clone() {
                    let mut message: String = " ".to_string();

                    if selected_letters.to_uppercase() == secret_word.clone() {
                        message = "You win!".to_string();
                    }
                    else if *attempts_clone == 6 && selected_letters.to_uppercase() != secret_word.clone(){
                        message = "You lose!".to_string();
                    }
                    let wind_: Rc<RefCell<DoubleWindow>> = Rc::clone(&wind);                    
                    let mut wind_: std::cell::RefMut<'_, DoubleWindow> = wind_.borrow_mut();
            
                    wind_.set_label(&message); 
                    wind_.clear();
                    wind_.redraw();
                    let mut resart_button = Button::new(100, 150, 200, 100, "RESTART");
                    resart_button.set_frame(FrameType::RFlatBox);
                    resart_button.set_label_size(20);
                    resart_button.set_label_color(Color::White);
                    resart_button.set_color(Color::rgb_color(156, 34, 24));
    
                    let wind_clone_for_restart_inner = Rc::clone(&wind_clone_for_restart);
                    resart_button.set_callback(move |_| {
                        wind_clone_for_restart_inner.borrow_mut().hide();
                        true;
                    });
                    let mut close_button = Button::new(350, 150, 200, 100, "CLOSE");
                    close_button.set_frame(FrameType::RFlatBox);
                    close_button.set_label_size(20);
                    close_button.set_label_color(Color::White);
                    close_button.set_color(Color::rgb_color(106, 170, 100));
    
                    close_button.set_callback(move |_| {
                        exit(0);
                    });
                    wind_.add(&resart_button);
                    wind_.add(&close_button);
                }
                else {
                    let mut frames = vec_frames_clone.borrow_mut();
                    let mut buttons_clone = vec_buttons_clone.borrow_mut();

                    for i in 0..5 {
                        match result[i] {
                            LetterStatus::Green => {
                                frames[*attempts_clone - 1][i].set_color(Color::rgb_color(106, 170, 100));
                                let letter = selected_letters.chars().nth(i).unwrap();
                                if let Some(&(row, column)) = letter_to_number.get(&letter) {
                                    buttons_clone[row][column].set_color(Color::rgb_color(106, 170, 100));
                                    buttons_clone[row][column].set_label_color(Color::White);
                                    buttons_clone[row][column].set_label(&selected_letters.chars().nth(i).unwrap().to_string());
                                } else {}
                            },
                            LetterStatus::Yellow => {
                                frames[*attempts_clone - 1][i].set_color(Color::rgb_color(201, 180, 88));
                                let letter = selected_letters.chars().nth(i).unwrap();
                                if let Some(&(row, column)) = letter_to_number.get(&letter) {
                                    buttons_clone[row][column].set_color(Color::rgb_color(201, 180, 88));
                                    buttons_clone[row][column].set_label_color(Color::White);
                                    buttons_clone[row][column].set_label(&selected_letters.chars().nth(i).unwrap().to_string());
                                } else {}
                            },
                            LetterStatus::Red => {
                                frames[*attempts_clone - 1][i].set_color(Color::rgb_color(156, 34, 24));
                                let letter = selected_letters.chars().nth(i).unwrap();
                                if let Some(&(row, column)) = letter_to_number.get(&letter) {
                                    buttons_clone[row][column].set_color(Color::rgb_color(156, 34, 24));
                                    buttons_clone[row][column].set_label_color(Color::White);
                                    buttons_clone[row][column].set_label(&selected_letters.chars().nth(i).unwrap().to_string());
                                } else {}
                            },
                            _ => {} 
                        }
                        frames[*attempts_clone - 1][i].set_label_color(Color::White);
                        frames[*attempts_clone - 1][i].set_label(&selected_letters.chars().nth(i).unwrap().to_string());
                    }
                }
                selected_letters.clear();
            }                
            println!("{}", selected_letters.clone());
        }
    });            

    let mut delete_button = Button::new(650, y, 80, 50, "DELETE");
    delete_button.set_frame(FrameType::RFlatBox);
    delete_button.set_label_size(20);
    delete_button.set_label_color(Color::Black);
    delete_button.set_color(Color::from_rgb(211, 214, 218));

    let selected_letters_clone = Rc::clone(&selected_letters);
    let vec_frames_clone = Rc::clone(&Rc::new(RefCell::new(vec_frames.clone())));
    let attempts_clone = Rc::clone(&attempts);
    delete_button.set_callback(move |_| {
        let mut selected_letters = selected_letters_clone.borrow_mut();
        if !selected_letters.is_empty() {
            let mut frames = vec_frames_clone.borrow_mut();
            let attempts_clone = attempts_clone.borrow();
            frames[*attempts_clone][selected_letters.len() - 1].set_label(' '.to_string().as_str());
    
            selected_letters.pop();
            println!("{}", selected_letters.clone());
        }
    });

    wind_clone.borrow_mut().end();
    wind_clone.borrow_mut().show();
    app.run().unwrap();
    true
}

fn create_letter_to_number_map() -> HashMap<char, (usize, usize)> {
    let mut letter_to_number = HashMap::new();
    letter_to_number.insert('Q', (0, 0));
    letter_to_number.insert('W', (0, 1));
    letter_to_number.insert('E', (0, 2));
    letter_to_number.insert('R', (0, 3));
    letter_to_number.insert('T', (0, 4));
    letter_to_number.insert('Y', (0, 5));
    letter_to_number.insert('U', (0, 6));
    letter_to_number.insert('I', (0, 7));
    letter_to_number.insert('O', (0, 8));
    letter_to_number.insert('P', (0, 9));   
    letter_to_number.insert('A', (1, 0));
    letter_to_number.insert('S', (1, 1));
    letter_to_number.insert('D', (1, 2));
    letter_to_number.insert('F', (1, 3));
    letter_to_number.insert('G', (1, 4));
    letter_to_number.insert('H', (1, 5));
    letter_to_number.insert('J', (1, 6));
    letter_to_number.insert('K', (1, 7));
    letter_to_number.insert('L', (1, 8));
    letter_to_number.insert('Z', (2, 0));
    letter_to_number.insert('X', (2, 1));
    letter_to_number.insert('C', (2, 2));
    letter_to_number.insert('V', (2, 3));
    letter_to_number.insert('B', (2, 4));
    letter_to_number.insert('N', (2, 5));
    letter_to_number.insert('M', (2, 6));
    letter_to_number
}