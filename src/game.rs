struct Game {
    word: String,
    solved: bool,
    lives: u8,
    correct_guesses: usize,
    chars_guessed: Vec<char>,
}

impl Game {
    fn new(word: String) -> Self {
        Game {
            word,
            solved: false,
            lives: 5,
            correct_guesses: 0,
            chars_guessed: Vec::new(),
        }
    }

    fn incr_correct_guesses(&mut self) {
        self.correct_guesses += 1
    }

    fn dec_lives(&mut self) {
        self.lives -= 1
    }
}

pub fn main() {
    let mut game = Game::new("rustman".to_string());

    while game.solved == false && game.lives > 0 {
        draw_game(&game);

        if let Some(i) = get_guess() {
            match test_guess(i, &game) {
                true => {
                    game.incr_correct_guesses();
                }
                false => {
                    game.dec_lives();
                }
            }
            game.chars_guessed.push(i);
        }

        if game.word.chars().count() == game.correct_guesses {
            game.solved = true;
            draw_game(&game);
        }
    }
}

fn format_secret_word(word: &String, guesses: &Vec<char>) -> String {
    let mut formatted_string = String::new();

    for c in word.chars() {
        formatted_string.push(if c == ' ' {
            c
        } else if guesses.contains(&c) {
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

fn test_guess(guess: char, game: &Game) -> bool {
    if game.word.contains(guess) && !game.chars_guessed.contains(&guess) {
        true
    } else {
        false
    }
}

fn draw_game(game: &Game) {
    clear();
    println!("{}", format_secret_word(&game.word, &game.chars_guessed));

    if game.solved {
        println!("Game solved!");
    } else {
        match game.lives {
            0 => {
                println!("Game over!");
            }
            1 => {
                println!("1 life left!");
            }
            2 => {
                println!("2 lives left.");
            }
            3 => {
                println!("3 lives left.");
            }
            4 => {
                println!("4 lives left.");
            }
            _ => {
                println!("{} lives left.", game.lives)
            }
        }
    }
}

fn clear() {
    // Clear terminal screen and place cursor at first row & column
    print!("\x1B[2J\x1B[1;1H");
}
