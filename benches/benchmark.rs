use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use lib::game::Game;

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

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("play", |b| b.iter(|| play()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
