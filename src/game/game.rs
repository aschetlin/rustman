use super::{Game, HashSet};

pub fn get_guess() -> Option<char> {
    let mut input = String::new();

    println!("Enter a guess.");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    input.trim().chars().nth(0)
}

pub fn test_guess(guess: char, game: &Game) -> bool {
    if game.word.contains(guess) && !game.incorrect_guesses.contains(&guess) {
        true
    } else {
        false
    }
}

pub fn get_unique_chars(word: &String) -> HashSet<char> {
    let mut result: HashSet<char> = vec![].into_iter().collect();
    let split_word: String = word.to_lowercase().split_whitespace().collect();

    for char in split_word.chars() {
        result.insert(char);
    }

    result
}
