
use lib::game::Game;

fn main() {
    let s =  "O O
                    O O X X
                    O O O X
                    O X";

    let game = Game::from_string(s).unwrap();
    game.play();
}
