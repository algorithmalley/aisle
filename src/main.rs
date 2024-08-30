// App to test/Debug the lib

use vector2d::Vector2D;
use aisle::plp::plp_n1;

fn main() {
    let bin: Vector2D<u32> = Vector2D { x: 1200, y: 800 };
    let item: Vector2D<u32> = Vector2D { x: 400, y: 300 };
    let plcm = plp_n1(bin, item);
    println!("{:?}", plcm);
}
