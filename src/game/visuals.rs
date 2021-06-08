use super::{Game, HashSet};

pub fn draw_game(game: &Game) {
    clear();

    if game.solved {
        println!("Game solved!");
    } else {
        match game.lives {
            0 => {
                println!(
                    "┌───────────────
│           ▼
│         ┌───┐
│         │x x│
│         └─┬─┘
│           │
│     ┌─────┼─────┐
│     │     │     │
│           │
│           │
│           │
│           │
│        ┌──┴──┐
│        │     │
│        │     │
│        │     │
│        │     │
│        │     │
│       ─┘     └─
│"
                );
            }
            1 => {
                println!(
                    "┌───────────────
│           ▼
│         ┌───┐
│         │   │
│         └─┬─┘
│           │
│     ┌─────┼─────┐
│     │     │     │
│           │
│           │
│           │
│           │
│        ┌──┴──┐
│        │     │
│        │     │
│        │     │
│        │     │
│        │     │
│       ─┘     └─
│"
                );
            }
            2 => {
                println!(
                    "┌───────────────
│           ▼
│         ┌───┐
│         │   │
│         └─┬─┘
│           │
│           │
│           │
│           │
│           │
│           │
│           │
│        ┌──┴──┐
│        │     │
│        │     │
│        │     │
│        │     │
│        │     │
│       ─┘     └─
│"
                )
            }
            3 => {
                println!(
                    "┌───────────────
│           ▼
│         ┌───┐
│         │   │
│         └─┬─┘
│           │
│           │
│           │
│           │
│           │
│           │
│           │
│
│
│
│
│
│
│
│"
                )
            }
            4 => {
                println!(
                    "┌───────────────
│           ▼
│         ┌───┐
│         │   │
│         └───┘
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│"
                );
            }
            _ => {
                println!(
                    "┌───────────────
│           ▼
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│"
                );
            }
        }
        println!(
            "\n{}",
            format_secret_word(&game.word, &game.correct_guesses)
        );
        println!("\n{} lives left.", game.lives)
    }
}

fn format_secret_word(word: &String, guesses: &HashSet<char>) -> String {
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

fn clear() {
    // Clear terminal screen and place cursor at first row & column
    print!("\x1B[2J\x1B[1;1H");
}
