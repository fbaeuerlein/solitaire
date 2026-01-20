
use lib::game::Game;

fn main() {
    let s =  "X X X
                    X X X
                X X X X X X X 
                X X X O X X X
                X X X X X X X
                    X X X
                    X X X";

    let game = Game::from_string(s).unwrap();
    let moves = game.play();
    game.print(&moves.unwrap());
}
