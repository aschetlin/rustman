use super::{Game, HashSet};

pub fn draw_game(game: &Game) {
    clear_screen();

    if game.solved {
        println!("Game solved!");
    } else {
        println!("{}", format_lives(&game));
        println!("{}", format_stats(&game));
    }
}

fn format_lives(game: &Game) -> String {
    match game.lives {
        0 => {
            format!(
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
            )
        }
        1 => {
            format!(
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
            )
        }
        2 => {
            format!(
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
            format!(
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
            format!(
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
            )
        }
        _ => {
            format!(
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
            )
        }
    }
}

fn format_stats(game: &Game) -> String {
    format!(
        "\n{}\n{} lives left.",
        format_secret_word(&game.word, &game.correct_guesses),
        game.lives
    )
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

fn clear_screen() {
    // Clear terminal screen and place cursor at first row & column
    print!("\x1B[2J\x1B[1;1H");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_stats() {
        let mut game = Game::new("test".to_string());
        game.dec_lives();
        let lives_char = format!("{}", game.lives);

        assert!(format_stats(&game).contains(lives_char.as_str()))
    }

    #[test]
    fn test_format_secret_word() {
        let word = String::from("word");
        let guesses: HashSet<char> = ['d'].iter().cloned().collect();
        let expected = String::from("___d");

        assert_eq!(format_secret_word(&word, &guesses), expected)
    }
}
