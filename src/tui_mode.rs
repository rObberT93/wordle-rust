use crossterm::event::{self, Event, KeyCode};
use std::{io, time::Duration, vec};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use crate::game::Game;

use super::game::LetterStatus;

const WORD_LENGTH: usize = 5;
const ALPHABET_LENGTH: usize = 26;
pub struct App {
    pub answer: String,
    pub input: String,
    pub tries: u32,
    pub guesses: Vec<Option<String>>,
    pub alphabet: Vec<u8>,
    pub word_status: Vec<Vec<u8>>,
    pub message: String,
    pub continue_to_play: bool,
    pub difficult_mode: bool,
    pub last_input: String,
    pub wins: i32,
    pub fails: i32,
    pub success_rate: f32,
    frequent_words: Vec<(String, usize)>,
    printed_1: bool, // output helper
    printed_2: bool,
    printed_3: bool,
    print_complish: bool,
}

impl App{
    pub fn default(answer: String, difficult_mode: bool) -> Self {
        App {
            answer,
            input: String::new(),
            tries: 0,
            guesses: Vec::new(),
            alphabet: vec![0; ALPHABET_LENGTH],
            word_status: Vec::new(),
            message: "Welcome to wordle!".to_string(),
            continue_to_play: false,
            difficult_mode,
            last_input: String::new(),
            wins: 0,
            fails: 0,
            success_rate: 0.0,
            frequent_words: Vec::new(),
            printed_1: false,
            printed_2: false,
            printed_3: false,
            print_complish: false,
        }
    }

    // win_state 0: the game is not over 1: game over and win 2: game over and fail
    pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App, word_list: &Vec<String>, game: &mut Game, win_state: u8, ask_continue_to_play: bool, to_print_state: bool) -> io::Result<Option<String>> {
        let mut _is_input_completed: bool = false;
        // typing characters
        loop {
            terminal.draw(|f| Self::ui(f, app))?;

            if win_state == 1 {
                app.message = format!("You win! Use {} chances. Press <enter> to continue...", app.guesses.len());
                terminal.draw(|f| Self::ui(f, app))?;
                _is_input_completed = true;
            } else if win_state == 2 {
                app.message = format!("Failed! The answer is {}. Press <enter> to continue...", app.answer);
                terminal.draw(|f| Self::ui(f, app))?;
                _is_input_completed = true;  
            }
            if app.print_complish && to_print_state {
                _is_input_completed = true;
                app.print_complish = false;
                app.printed_1 = false;
                app.printed_2 = false;
                app.printed_3 = false;
                break;
                // finish printing stats
            }
            if !app.printed_1 && !app.printed_2 && !app.print_complish && !app.printed_3 && to_print_state {
                app.message = format!("Enter <s> to show statistical information");
                terminal.draw(|f| Self::ui(f, app))?;
                app.printed_1 = true;
            }
            
            if ask_continue_to_play {
                app.message = format!("Want more games? Enter <y> to continue, <n> to exit");
                terminal.draw(|f| Self::ui(f, app))?;
            }

            // 处理按键事件
            if event::poll(Duration::from_secs(1))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(ch) => {
                            if to_print_state {
                                if app.printed_1 && !app.printed_2 && !app.print_complish && (ch == 's' || ch == 'S') {
                                    app.message = format!("Wins: {}; Fails: {}, press <enter> to continue...", app.wins, app.fails);
                                    terminal.draw(|f| Self::ui(f, app))?;
                                    app.printed_2 = true;
                                    continue;
                                }
                            } else {
                                if ask_continue_to_play {
                                    if ch == 'y' || ch == 'Y' {
                                        _is_input_completed = true;
                                        app.continue_to_play = true;
                                        break;
                                    } else if ch == 'n' || ch == 'N' {
                                        _is_input_completed = true;
                                        app.continue_to_play = false;
                                        break;
                                    } else {
                                        continue;
                                    }
                                } else {
                                    app.input.push(ch);                                        
                                }
                            }
                        }
                        KeyCode::Esc => {
                            _is_input_completed = true;
                            app.continue_to_play = false;
                            break;
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Enter => {
                            // print frequent words
                            if to_print_state && app.printed_1 && app.printed_2 && !app.printed_3 && !app.print_complish {
                                let message: String = app.frequent_words
                                    .iter()
                                    .map(|(word, count)| format!("{}({})", word, count))
                                    .collect::<Vec<String>>()
                                    .join(", ");
                                app.message = message;
                                terminal.draw(|f| Self::ui(f, app))?;
                                app.printed_3 = true;
                                continue;
                            }
                            if to_print_state && app.printed_1 && app.printed_2 && app.printed_3 && !app.print_complish {
                                app.print_complish = true;
                            }
                            if win_state != 0 {
                                break;
                            }
                            if app.input.len() != WORD_LENGTH {
                                app.message = format!("Each word should be 5 in length!");
                                terminal.draw(|f| Self::ui(f, app))?;
                                continue;
                            } else {
                                // INVALID input
                                if !Game::input_valid_check(&app.input.to_uppercase(), word_list) {
                                    app.message = format!("Not in word list!");
                                    terminal.draw(|f| Self::ui(f, app))?;
                                    app.input.clear();
                                    continue;
                                } else if app.difficult_mode { // check diffcult mode
                                    if !game.check_difficult_mode(app.input.to_uppercase().as_str()) {
                                        app.message = format!("You must use the hint in difficult mode.");
                                        terminal.draw(|f| Self::ui(f, app))?;
                                        continue;
                                    }
                                }
                                // VALID 
                                app.tries += 1;
                                app.guesses.push(Some(app.input.to_uppercase().clone()));

                                // print colored characters
                                let mut temp_word_status: [u8; WORD_LENGTH] = [0; WORD_LENGTH];
                                let word_status: [LetterStatus; WORD_LENGTH] = game.play(app.input.clone().to_uppercase().as_str());
                                for (index, letter_status) in word_status.iter().enumerate() {
                                    let number: u8 = match letter_status {
                                        LetterStatus::Unknown => 0,
                                        LetterStatus::Red => 1,
                                        LetterStatus::Yellow => 2,
                                        LetterStatus::Green => 3,
                                    }; 
                                    temp_word_status[index] = number;
                                }
                                app.word_status.push(temp_word_status.to_vec());
                                let alphabet_status: [LetterStatus; ALPHABET_LENGTH] = game.alphabet;
                                for (index, letter_status) in alphabet_status.iter().enumerate() {
                                    let number: u8 = match letter_status {
                                        LetterStatus::Unknown => 0,
                                        LetterStatus::Red => 1,
                                        LetterStatus::Yellow => 2,
                                        LetterStatus::Green => 3,
                                    };
                                    app.alphabet[index] = number;
                                }

                                // save for later use
                                app.last_input = app.input.to_uppercase().clone();
                                app.input.clear();
                                _is_input_completed = true;
                                terminal.draw(|f| Self::ui(f, app))?;
                                // break the loop + return input word + return to main fn
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        if _is_input_completed {
            return Ok(Some(app.last_input.clone()));
        } else {
            return Ok(None);
        }
    }
    pub fn input_answer<B: Backend>(terminal: &mut Terminal<B>, app: &mut App, answer_list: &Vec<String> ) -> io::Result<Option<String>> {
        let answer: String;
        app.message = format!("please input your answer to start the game");
        loop {
            terminal.draw(|f| Self::ui(f, app))?;
            if event::poll(Duration::from_secs(1))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(ch) => {
                            app.input.push(ch);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Enter => {
                            if app.input.len() != WORD_LENGTH {
                                app.message = format!("The length of the answer should be 5!");
                                app.input.clear();
                                terminal.draw(|f| Self::ui(f, app))?;
                            }
                            if !answer_list.iter().any(|word| word == &app.input.to_uppercase().clone()) {
                                app.message = format!("INVALID input");
                                app.input.clear();
                                terminal.draw(|f| Self::ui(f, app))?; 
                            } else {
                                answer = app.input.to_uppercase().clone();
                                app.input.clear();
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
        
        }
        return Ok(Some(answer.clone()));
    }

    // draw ui
    pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        // structure
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(9), 
                        Constraint::Percentage(37), 
                        Constraint::Percentage(9), 
                        Constraint::Percentage(9), 
                        Constraint::Percentage(9), 
                        Constraint::Percentage(9), 
                        Constraint::Percentage(9), 
                        Constraint::Percentage(9), 
                        ].as_ref())
            .direction(Direction::Vertical)
            .split(f.size());
        // two text boxes
        let input_text_layout = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .direction(Direction::Horizontal)
            .split(chunks[0]);
        // input box
        let input_paragraph = Paragraph::new(Spans::from(vec![
            Span::raw("Input: "),
            Span::styled(
                &app.input,
                Style::default().add_modifier(Modifier::BOLD).fg(tui::style::Color::Gray),
            ),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Keyboard Input"))
        .alignment(Alignment::Left);
        
        f.render_widget(input_paragraph, input_text_layout[0]);
        // prompt box
        let text_box_paragraph = Paragraph::new(Spans::from(vec![
            Span::raw("Message: "),
            Span::styled(
                &app.message,
                Style::default().add_modifier(Modifier::BOLD).fg(tui::style::Color::Red),
            ),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Interactive info: "))
        .alignment(Alignment::Left);
        
        f.render_widget(text_box_paragraph, input_text_layout[1]);
        // draw the keyboard
        let keyboard_layout = Layout::default()
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ]
            .as_ref(),
        )
        .direction(Direction::Vertical)
        .split(chunks[1]);
    
        let keyboard_row_1 = vec![
            "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P",
        ];
    
        let keyboard_row_2 = vec![
            "A", "S", "D", "F", "G", "H", "J", "K", "L",
        ];
    
        let keyboard_row_3 = vec![
            "Enter", "Z", "X", "C", "V", "B", "N", "M", "Delete",
        ];
    
        let keyboard_rows = vec![keyboard_row_1, keyboard_row_2, keyboard_row_3];
    
        for (i, row) in keyboard_rows.iter().enumerate() {
            let row_layout = Layout::default()
                // .constraints::<&Constraint>(
                .constraints::<&[Constraint]>(
                    row.iter()
                        .map(|_| Constraint::Percentage(10))
                        .collect::<Vec<_>>()
                        .as_ref(),
                )
                .direction(Direction::Horizontal)
                .split(keyboard_layout[i]);
    
            for (j, key) in row.iter().enumerate() {
                // special for ENTER and DELETE
                let key_widget = if (i, j) == (2, 0) || (i, j) == (2, 8) {
                    Paragraph::new(Spans::from(*key))
                        .block(Block::default().borders(Borders::ALL))
                        .alignment(Alignment::Center)
                } else { 
                    Paragraph::new(Spans::from(vec![Span::styled(
                        *key,
                        update(app.alphabet[key.chars().next().unwrap() as usize - b'A' as usize]),
                    )]))
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(Alignment::Center)
                };
                f.render_widget(key_widget, row_layout[j]);
            }
        }
        // While seemingly silly, I lack good ideas... 
        // The drawn window will disappear after leaving the buffer, so I don't know how to use a loop
        let empty_box_1 =  Paragraph::new("").block(Block::default().borders(Borders::ALL).title(format!("Guess: {}", 1))).alignment(Alignment::Left);
        let empty_box_2 =  Paragraph::new("").block(Block::default().borders(Borders::ALL).title(format!("Guess: {}", 2))).alignment(Alignment::Left);
        let empty_box_3 =  Paragraph::new("").block(Block::default().borders(Borders::ALL).title(format!("Guess: {}", 3))).alignment(Alignment::Left);
        let empty_box_4 =  Paragraph::new("").block(Block::default().borders(Borders::ALL).title(format!("Guess: {}", 4))).alignment(Alignment::Left);
        let empty_box_5 =  Paragraph::new("").block(Block::default().borders(Borders::ALL).title(format!("Guess: {}", 5))).alignment(Alignment::Left);
        let empty_box_6 =  Paragraph::new("").block(Block::default().borders(Borders::ALL).title(format!("Guess: {}", 6))).alignment(Alignment::Left);

        f.render_widget(empty_box_1, chunks[2]);
        f.render_widget(empty_box_2, chunks[3]);
        f.render_widget(empty_box_3, chunks[4]);
        f.render_widget(empty_box_4, chunks[5]);
        f.render_widget(empty_box_5, chunks[6]);
        f.render_widget(empty_box_6, chunks[7]);

        let mut w1: String = String::new();
        let mut w2: String = String::new();
        let mut w3: String = String::new();
        let mut w4: String = String::new();
        let mut w5: String = String::new();
        let mut w6: String = String::new();
        
        if app.guesses.len() >= 1 {
            if let Some(guess_word) = app.guesses[0].clone() {
                w1 = guess_word;
            }    
        }
        if app.guesses.len() >= 2 {
            if let Some(guess_word) = app.guesses[1].clone() {
                w2 = guess_word;
            }    
        }
        if app.guesses.len() >= 3 {
            if let Some(guess_word) = app.guesses[2].clone() {
                w3 = guess_word;
            }    
        }
        if app.guesses.len() >= 4 {
            if let Some(guess_word) = app.guesses[3].clone() {
                w4 = guess_word;
            }    
        }
        if app.guesses.len() >= 5 {
            if let Some(guess_word) = app.guesses[4].clone() {
                w5 = guess_word;
            }    
        }
        if app.guesses.len() >= 6 {
            if let Some(guess_word) = app.guesses[5].clone() {
                w6 = guess_word;
            }
        }
        // update word to text box
        // guess word 1
        if app.guesses.len() >= 1 {
            let guess_word_spans: Vec<Span> = w1.chars()
            .enumerate()
            .map(|(i, c)| {
                let color = update(app.word_status[0][i]); 
                Span::styled(c.to_string(), color)
            })
            .collect();    
            let guess_word_paragraph = Paragraph::new(Spans::from(guess_word_spans))
                .block(Block::default().borders(Borders::ALL).title(format!("Guess: {}", 1)))
                .alignment(Alignment::Left);
            f.render_widget(guess_word_paragraph, chunks[2]);  
        }
        // guess word 2
        if app.guesses.len() >= 2 {
            let guess_word_spans: Vec<Span> = w2.chars()
            .enumerate()
            .map(|(i, c)| {
                let color = update(app.word_status[1][i]); 
                Span::styled(c.to_string(), color)
            })
            .collect();    
            let guess_word_paragraph = Paragraph::new(Spans::from(guess_word_spans))
                .block(Block::default().borders(Borders::ALL).title(format!("Guess: {}", 2)))
                .alignment(Alignment::Left);
            f.render_widget(guess_word_paragraph, chunks[3]);  
        }
        // guess word 3
        if app.guesses.len() >= 3 {
            let guess_word_spans: Vec<Span> = w3.chars()
            .enumerate()
            .map(|(i, c)| {
                let color = update(app.word_status[2][i]); 
                Span::styled(c.to_string(), color)
            })
            .collect();    
            let guess_word_paragraph = Paragraph::new(Spans::from(guess_word_spans))
                .block(Block::default().borders(Borders::ALL).title(format!("Guess: {}", 3)))
                .alignment(Alignment::Left);
            f.render_widget(guess_word_paragraph, chunks[4]);
        }
        // guess word 4
        if app.guesses.len() >= 4 {
            let guess_word_spans: Vec<Span> = w4.chars()
            .enumerate()
            .map(|(i, c)| {
                let color = update(app.word_status[3][i]); 
                Span::styled(c.to_string(), color)
            })
            .collect();    
            let guess_word_paragraph = Paragraph::new(Spans::from(guess_word_spans))
                .block(Block::default().borders(Borders::ALL).title(format!("Guess: {}", 4)))
                .alignment(Alignment::Left);
            f.render_widget(guess_word_paragraph, chunks[5]);  
        }
        // guess word 5
        if app.guesses.len() >= 5 {
            let guess_word_spans: Vec<Span> = w5.chars()
            .enumerate()
            .map(|(i, c)| {
                let color = update(app.word_status[4][i]); 
                Span::styled(c.to_string(), color)
            })
            .collect();    
            let guess_word_paragraph = Paragraph::new(Spans::from(guess_word_spans))
                .block(Block::default().borders(Borders::ALL).title(format!("Guess: {}", 5)))
                .alignment(Alignment::Left);
            f.render_widget(guess_word_paragraph, chunks[6]);  
        }
        // guess word 6
        if app.guesses.len() >= 6 {
            let guess_word_spans: Vec<Span> = w6.chars()
            .enumerate()
            .map(|(i, c)| {
                let color = update(app.word_status[5][i]); 
                Span::styled(c.to_string(), color)
            })
            .collect();    
            let guess_word_paragraph = Paragraph::new(Spans::from(guess_word_spans))
                .block(Block::default().borders(Borders::ALL).title(format!("Guess: {}", 6)))
                .alignment(Alignment::Left);
            f.render_widget(guess_word_paragraph, chunks[7]);  
        }
    }

    pub fn load_stats_message(&mut self, wins: i32, fails: i32, success_rate: f32, frequent_words: Vec<(String, usize)>) {
        self.wins = wins;
        self.fails = fails;
        self.success_rate = success_rate;
        self.frequent_words = frequent_words;
    }
}

fn update(status: u8) -> Style {
    if status == 0 {
        Style::default().add_modifier(Modifier::BOLD).fg(tui::style::Color::Gray)
    } else if status == 1 {
        Style::default().add_modifier(Modifier::BOLD).fg(tui::style::Color::Red)
    } else if status == 2 {
        Style::default().add_modifier(Modifier::BOLD).fg(tui::style::Color::Yellow)     
    } else {
        Style::default().add_modifier(Modifier::BOLD).fg(tui::style::Color::Green)
    }
}
