use super::HashSet;

pub struct Game {
    pub word: String,
    word_chars: HashSet<char>,
    pub solved: bool,
    pub lives: u8,
    pub correct_guesses: HashSet<char>,
    incorrect_guesses: HashSet<char>,
}

impl Game {
    pub fn new(word: String) -> Self {
        let unique_chars = get_unique_chars(&word);

        Game {
            word,
            word_chars: unique_chars,
            solved: false,
            lives: 5,
            correct_guesses: HashSet::new(),
            incorrect_guesses: HashSet::new(),
        }
    }

    pub fn dec_lives(&mut self) {
        self.lives -= 1
    }
}

pub fn game_events(mut game: &mut Game) {
    if let Some(i) = get_guess() {
        handle_guess(i, &mut game);
    }

    if game.word_chars == game.correct_guesses {
        game.solved = true;
    }
}

fn get_guess() -> Option<char> {
    let mut input = String::new();

    println!("Enter a guess.");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    input.trim().chars().nth(0)
}

fn handle_guess(guess: char, game: &mut Game) {
    match test_guess(guess, game) {
        true => {
            game.correct_guesses.insert(guess);
        }
        false => {
            game.dec_lives();
            game.incorrect_guesses.insert(guess);
        }
    }
}

fn get_unique_chars(word: &String) -> HashSet<char> {
    let mut result: HashSet<char> = HashSet::new();
    let split_word: String = word.to_lowercase().split_whitespace().collect();

    for char in split_word.chars() {
        result.insert(char);
    }

    result
}

fn test_guess(guess: char, game: &Game) -> bool {
    if game.word.contains(guess) && !game.incorrect_guesses.contains(&guess) {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_correct_guess() {
        let mut game = Game::new("test".to_string());
        let guess: char = 't';
        handle_guess(guess, &mut game);

        assert!(game.correct_guesses.contains(&guess));
    }

    #[test]
    fn test_handle_incorrect_guess() {
        let mut game = Game::new("test".to_string());
        let guess: char = 'x';
        let previous_lives = game.lives;
        handle_guess(guess, &mut game);

        assert!(game.incorrect_guesses.contains(&guess));
        assert!(game.lives < previous_lives);
    }

    #[test]
    fn test_get_unique_chars() {
        let word = String::from("test words");
        let desired_output: HashSet<char> = ['t', 'e', 's', 'w', 'o', 'r', 'd']
            .iter()
            .cloned()
            .collect();

        assert_eq!(get_unique_chars(&word), desired_output)
    }

    #[test]
    fn test_test_guess() {
        let game = Game::new("test".to_string());
        assert!(test_guess('t', &game))
    }
}
