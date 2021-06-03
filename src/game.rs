struct Game {
    word: String,
    solved: bool,
    lives: u8,
    correct_guesses: usize,
    chars_guessed: Vec<char>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            word: String::from("rustman"),
            solved: false,
            lives: 5,
            correct_guesses: 0,
            chars_guessed: Vec::new(),
        }
    }
}

pub fn main() {
    let mut game = Game::default();

    while game.solved == false && game.lives > 0 {
        println!("{}", format_secret_word(&game.word, &game.chars_guessed));

        if let Some(i) = get_guess() {
            game.chars_guessed.push(i);

            match test_guess(i, &game.word) {
                true => {
                    game.correct_guesses += 1;
                    println!("{} is a correct guess!", i);
                    println!("Correct guesses: {}", game.correct_guesses);
                }
                false => {
                    game.lives -= 1;
                    println!("{} is not a correct guess.", i);
                    println!("{} lives remaining.", game.lives);
                }
            }
        }

        if game.word.chars().count() == game.correct_guesses {
            game.solved = true;
            println!("Game solved.");
        }
    }
}

fn format_secret_word(word: &String, mask: &Vec<char>) -> String {
    let mut formatted_string = String::new();

    for c in word.chars() {
        formatted_string.push(if c == ' ' {
            c
        } else if mask.contains(&c) {
            c
        } else {
            '_'
        })
    }

    formatted_string
}

fn get_guess() -> Option<char> {
    let mut input = String::new();

    println!("Enter a guess.");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    input.trim().chars().nth(0)
}

fn test_guess(guess: char, word: &String) -> bool {
    if word.contains(guess) {
        true
    } else {
        false
    }
}
