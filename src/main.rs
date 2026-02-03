
use lib::game::Game;

fn main() {
    // let game = Game::from_string(s).unwrap();
    let game = Game::new(3);
    let moves = game.play();
    game.print(&moves.unwrap());
}
