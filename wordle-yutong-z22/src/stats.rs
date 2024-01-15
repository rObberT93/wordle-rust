use std::collections::HashMap;
use std::collections::btree_map::Range;
use std::{
    io::{self, Write},
};
use colored::Colorize;

use serde::{Deserialize, Serialize};
use serde_json::json;
use super::game::GuessWordStatus;

use std::path::PathBuf;
use std::fs;

type Counter = HashMap<String, usize>;

#[derive(Clone, Serialize, Deserialize)]
struct Game {
    answer: String,
    guesses: Vec<String>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct GameState {
    total_rounds: Option<u32>,
    games: Option<Vec<Game>>,
}

//#[derive(Default, Serialize, Deserialize)]
/// 应该存储所有和多局游戏有关的信息
#[allow(dead_code)]
pub struct Stats {
    wins: i32,
    fails: i32,
    total_tries: i32,
    used_words: Counter, //存储所有猜过的词
    games: Vec<Game>, //存储一次游戏中的所有单局游戏，一局游戏状态确定
    state_path: Option<PathBuf>,//状态文件的路径
}

#[allow(dead_code)]
impl Stats{
    pub fn new() -> Self {
        Stats {
            wins: 0,
            fails: 0,
            total_tries: 0,
            used_words: Counter::new(),
            games: vec![],
            state_path: None,
        }
    }

    pub fn update(&mut self, guesses: &Vec<(String, GuessWordStatus)>, answer: String, is_win: bool) {
        if is_win {
            self.wins += 1;
            self.total_tries += guesses.len() as i32;
        } else {
            self.fails += 1;
        }
        let mut all_guess_words: Vec<String> = vec![];
        for (word, _) in guesses {
            self.count(word.to_owned());
            all_guess_words.push(word.to_string());
        }
        self.games.push(Game{
            answer: answer.to_string(),
            guesses: all_guess_words,
        })
    }

    fn count(&mut self, word: String) {
        let entry = self.used_words.entry(word).or_insert(0);
        *entry += 1;
    } // 存储所有猜过的词

    fn success_rate(&self) -> f32{
        if self.wins == 0 {
            0.0
        }else {
            let tries = self.wins + self.fails;
            self.wins as f32 / tries as f32
        }
    }

    pub fn print_stats(&self) {
        // println!("You winned {} games, lost {} games!", self.wins.to_string().blue(), self.fails.to_string().blue());
        // println!("Your chance of winning is {:.2}", self.success_rate().to_string().blue());
        // println!("The words that you use most frequently are:");

        // let mut words: Vec<(&String, &usize)> = self.used_words.iter().collect();
        // words.sort_by(|a: &(&String, &usize), b: &(&String, &usize)| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
        // for (_, (word, count)) in words.iter().take(5).enumerate() {
        //     print!("{} {} ", word.to_string().green(), count.to_string().blue());
        // }
        // println!();

        //test
        print!("{} {} {:.2}", self.wins, self.fails, self.get_average_tries());
        println!();
        let mut words: Vec<(&String, &usize)> = self.used_words.iter().collect();
        words.sort_by(|a: &(&String, &usize), b: &(&String, &usize)| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
        
        for (i, &(word, count)) in words.iter().enumerate().take(5) {
            print!("{} {}", word, count);
            if i < 4 && i < words.len() - 1 {
                print!(" ");
            }
        }
        
        println!();
    }

    pub fn get_average_tries(&self) -> f64{
        if self.wins == 0 {
            return 0.0;
        } 
        self.total_tries as f64 / self.wins as f64
    }
    //是否需要加载之前的游戏状态
    //如果没有路径 直接创建新状态
    //如果该路径下没有这个文件 创建文件
    //如果存在json文件 加载
    //如果加载不合法 返回none
    //加载合法 返回状态
    pub fn load(state_path: &Option<PathBuf>) -> Option<Self> {
        if state_path.is_some() { 
            let mut stats = Self::new();
            stats.state_path = state_path.clone();

            if PathBuf::from(state_path.as_ref().unwrap()).exists() {
                if let Ok(state) = serde_json::from_str::<GameState>(
                    fs::read_to_string(state_path.as_ref().unwrap())
                        .unwrap()
                        .as_str(),
                ) {
                    if let Some(games) = state.games {
                        for game in games {
                            stats.games.push(game.clone());
                            if game.guesses.last()? == &game.answer {
                                stats.wins += 1;
                                stats.total_tries += game.guesses.len() as i32;
                            } else {
                                stats.fails += 1;
                            }
                            for word in game.guesses {
                                Self::count(&mut stats, word);
                            }
                        }
                    }
                    Some(stats)
                } else {
                    None
                }
            } else {
                //文件不存在，创建文件
                Some(stats)
            }
        } else {
            Some(Self::new())
        }
    }

    pub fn save(&mut self) {
        let state = GameState {
            total_rounds: Some((self.wins + self.fails) as u32),
            games: Some(self.games.clone()),
        };
        if let Some(path) = &self.state_path {
            fs::write(path, serde_json::to_string_pretty(&state).unwrap()).unwrap();
        }
    }
}