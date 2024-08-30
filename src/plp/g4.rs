use crate::plp::*;
use vector2d::Vector2D;

/// Solves the plp with the recursive G4 heuristic
///
/// References :
/// - [The G4-Heuristic for the Pallet Loading Problem](../papers/1996-Scheithauer-G4-Heuristic-Pallet-Loading.pdf)
pub fn plp_g4(bin: Vector2D<u32>, item: Vector2D<u32>) -> Vec<Placement> {
    Vec::new()
}
