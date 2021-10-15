mod game;
mod visuals;

use game::{game_events, Game};
use std::collections::HashSet;
use visuals::draw_game;

pub fn main() {
    let mut game = Game::new("rest in piece".to_string());

    while !game.solved && game.lives > 0 {
        draw_game(&game);
        game_events(&mut game);
    }
    draw_game(&game);
}
