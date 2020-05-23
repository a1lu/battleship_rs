use crate::turn;

pub struct Player {
    name: String,
    turn: fn(&Player) -> turn::Move,
}

impl Player {
    pub fn new(name: &str, turn: fn(name: &Player) -> turn::Move) -> Player {
        Player {
            name: name.to_string(),
            turn,
        }
    }

    pub fn make_turn(&self) {
        (self.turn)(&self);
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
