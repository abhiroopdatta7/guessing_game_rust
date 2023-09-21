mod guessing_game;

fn main() {
    let mut game = crate::guessing_game::Game::init(1, 101);
    game.play();
}
