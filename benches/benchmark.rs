use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use lib::game::Game;
use lib::board::Board;
use lib::board_manager::BoardManager;

fn play(game : &Game)
{
    black_box(game.play());
}

fn game_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("game");
    group.significance_level(0.1).sample_size(10);
    group.bench_function("play (n = 3)", |b| b.iter(|| play()));
    group.finish();
}

fn get_possible_moves(board: &Board)
{
    black_box(board.get_possible_moves(&(2, 4)));
}

fn get_all_possible_moves(board: &Board)
{
    black_box(board.get_all_possible_moves());
}

fn get_index(mgr: &BoardManager)
{
    black_box(mgr.get_index(&(0, 0)));
    black_box(mgr.get_index(&(2, 0)));
    black_box(mgr.get_index(&(3, 3)));
    black_box(mgr.get_index(&(2, 6)));
    black_box(mgr.get_index(&(6, 6)));
}

fn get_coordinates(mgr: &BoardManager)
{
    black_box(mgr.get_coordinate(17));
    black_box(mgr.get_coordinate(0));
    black_box(mgr.get_coordinate(32));
}



fn board_manager_benches(c: &mut Criterion)
{
    let mut group = c.benchmark_group("board_manager");
    let mgr = BoardManager::new(3);
    group.significance_level(0.1).sample_size(100);
    group.bench_with_input("get_index", &mgr.clone(), |b, mgr| b.iter(|| get_index(mgr)));
    group.bench_with_input("get_coordinates", &mgr.clone(), |b, mgr| b.iter(|| get_coordinates(mgr)));
    group.finish();
}

fn board_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("board");
    let board = Board::default(3);
    group.significance_level(0.1).sample_size(100);
    group.bench_with_input("get_possible_moves", &board.clone(), |b, board| b.iter(|| get_possible_moves(board)));
    group.bench_with_input("get_all_possible_moves", &board.clone(), |b, board| b.iter(|| get_all_possible_moves(board)));
    group.finish();
}

criterion_group!(benches, board_benches, board_manager_benches);
criterion_main!(benches);
