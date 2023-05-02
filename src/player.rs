use crate::{OptPos, RelPos, Map};
use crate::api::Direction;

#[derive(Clone, Debug, Default)]
pub struct Player {
    pub pos : OptPos,
    pub delta_pos : RelPos,

    dir : Option<Direction>
}

impl Player {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset_pos(&mut self) { 
        self.pos = OptPos::default();
        self.delta_pos = RelPos::default();
    }

    #[inline]
    pub fn set_dir(&mut self, dir : Option<Direction>) {
        self.dir = dir;
    }

    #[inline]
    pub fn get_dir(&self) -> Option<Direction> {
        self.dir
    }

    pub fn reg_move(&mut self, dir : Direction) {
        match dir {
            Direction::North => {
                self.delta_pos.1 += 1; 
                if let Some(y) = self.pos.1 {
                    self.pos.1 = Some(y + 1);
                }
            }, 
            Direction::East => {
                self.delta_pos.0 += 1; 
                if let Some(x) = self.pos.0 {
                    self.pos.0 = Some(x + 1);
                }
            },
            Direction::South => {
                self.delta_pos.1 -= 1; 
                if let Some(y) = self.pos.1 {
                    self.pos.1 = Some(y - 1);
                }
            },
            Direction::West => {
                self.delta_pos.0 -= 1; 
                if let Some(x) = self.pos.0 {
                    self.pos.0 = Some(x - 1);
                }
            }
        }
    }

    pub fn map_wall(&mut self, dir : Direction, map : &Map) {
        match dir {
            Direction::North => {
                self.pos.1 = Some(map.size.1);
            }, 
            Direction::East => {
                self.pos.0 = Some(map.size.0);
            },
            Direction::South => {
                self.pos.1 = Some(0);
            },
            Direction::West => {
                self.pos.0 = Some(0);
            }
        }
    }

    pub fn is_pos_safe(&self, pos : RelPos, map : &Map) -> bool {
        if pos.0 != 0 {
            if let Some(x) = self.pos.0 {
                let abs_x = x + pos.0;
                if (abs_x >= map.size.0) | (abs_x < 0) {
                    return false;
                }
            } else {
                if self.delta_pos.0.abs() < (self.delta_pos.0 + pos.0).abs() {
                    return false;
                }
            }

            if let Some(y) = self.pos.1 {
                let abs_y = y + pos.1;
                if (abs_y >= map.size.1) | (abs_y < 0) {
                    return false;
                }
            } else {
                if self.delta_pos.1.abs() < (self.delta_pos.1 + pos.1).abs() {
                    return false;
                }
            }
        }

        true
    }
}