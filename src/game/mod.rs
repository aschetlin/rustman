mod game;
mod visuals;

use game::{get_guess, handle_guess, get_unique_chars};
use std::collections::HashSet;
use visuals::draw_game;

pub struct Game {
    word: String,
    solved: bool,
    lives: u8,
    correct_guesses: HashSet<char>,
    incorrect_guesses: HashSet<char>,
}

impl Game {
    fn new(word: String) -> Self {
        Game {
            word,
            solved: false,
            lives: 5,
            correct_guesses: vec![].into_iter().collect(),
            incorrect_guesses: vec![].into_iter().collect(),
        }
    }

    fn dec_lives(&mut self) {
        self.lives -= 1
    }
}

pub fn main() {
    let mut game = Game::new("large boulder the size of a small boulder".to_string());
    let word_chars = get_unique_chars(&game.word);

    while game.solved == false && game.lives > 0 {
        draw_game(&game);

        if let Some(i) = get_guess() {
            handle_guess(i, &mut game);
        }

        if word_chars == game.correct_guesses {
            game.solved = true;
        }
    }

    draw_game(&game);
}
