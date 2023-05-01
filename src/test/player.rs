use crate::{RelPos, OptPos};
use crate::game::Map;
use crate::player::Player;

#[test]
fn is_pos_safe() {
    let mut pl = Player::new();
    let map = Map::DEFAULT_LEVEL;

    // Since the start, the player has moved 4 blocks to right and 2 to the left
    pl.delta_pos = RelPos(4, -2);   

    assert!(pl.is_pos_safe(RelPos(-2, 1), &map));       // Safe
    assert!(!pl.is_pos_safe(RelPos(2, 1), &map));       // Not safe in X (out of box)
    assert!(!pl.is_pos_safe(RelPos(-2, -1), &map));     // Not safe in Y (out of box)

    pl.pos = OptPos(Some(5), Some(5));
    
    assert!(pl.is_pos_safe(RelPos(2, 2), &map));        // Safe  
    assert!(!pl.is_pos_safe(RelPos(-12, 2), &map));     // Not safe in X (out of map)
    assert!(!pl.is_pos_safe(RelPos(2, -12), &map));     // Not safe in Y (out of map)
    assert!(!pl.is_pos_safe(RelPos(12, 2), &map));      // Not safe in X (out of map)
    assert!(!pl.is_pos_safe(RelPos(2, 12), &map));      // Not safe in Y (out of map)
}