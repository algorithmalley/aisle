/// Pallet Loading Problem

use vector2d::Vector2D;

#[derive(Debug)]
pub struct Placement {
    pos: Vector2D<u32>,
    o: Orientation,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

mod n1;
mod g4;
pub use n1::plp_n1;
pub use g4::plp_g4;
