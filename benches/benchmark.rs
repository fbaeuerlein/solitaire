use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use lib::game::Game;
use lib::board::Board;


    fn play()
    {
        let s =  "X X X
                        X X X
                    X X X X X X X 
                    X X X O X X X
                    X X X X X X X
                        X X X
                        X X X";
    //     let s = 
    //     "X X
    //    X X X X
    //    X O X X
    //      X X";

        let g = Game::from_string(s).unwrap();

        let m = g.play();

        // g.print(&m.unwrap());
    }

// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("play", |b| b.iter(|| play()));
// }

    fn get_possible_moves_01()
    {
        let s =  "X X X
                        X X X
                    X X X X X X X 
                    X X X O X X X
                    X X X X X X X
                        X X X
                        X X X";
        let moves = Board::from_string(s).unwrap().get_possible_moves(&(2, 4));
    }

fn get_possible_moves(c: &mut Criterion) {
    c.bench_function("get_possible_moves", |b| b.iter(|| play()));
}

criterion_group!(benches, get_possible_moves);
criterion_main!(benches);
