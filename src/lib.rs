mod turn;
mod player;
mod board;

pub use turn::{Move,Point};
pub use player::Player;

pub struct Config;


pub struct Game {
    player1: Player,
    player2: Player,
    config: Config,
    player_active: u8
}

impl Game {
    pub fn new(player1: Player, player2: Player, config: Config) -> Game {
        Game {player1, player2, config, player_active: 1}
    }

    pub fn run(&mut self) {
        loop {
            match self.player_active{
                1 => {
                    self.player1.make_turn();
                    self.player_active = 2
                }
                2 => {
                    self.player2.make_turn();
                    self.player_active = 1
                }
                _ => {}
            }
        }
    }
}
