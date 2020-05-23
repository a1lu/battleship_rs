use std::io;

fn move_from_stdin() -> battleship::Move {
    println!("Enter you move:");
    let mut mv = String::new();
    io::stdin().read_line(&mut mv).expect("Failed to read line");
    let parts = mv.trim().split_whitespace();

    let mut mv = parts.take(2).filter_map(|i| i.parse().ok());
    battleship::Move::Shot(battleship::Point {
        x: mv.next().unwrap(),
        y: mv.next().unwrap(),
    })
}

fn get_player(name: &str) -> battleship::Player {
    battleship::Player::new(name, |player| {
        println!("Hello, {}!", player.name());
        let mv = move_from_stdin();
        println!{"{:?}", mv};
        mv
    })
}

fn main() {
    println!("Hello, world!");

    let player1 = get_player("P1");
    let player2 = get_player("P2");

    let mut game = battleship::Game::new(player1, player2, battleship::Config{});

    game.run();
}
