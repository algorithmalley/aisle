use crate::plp::*;
use vector2d::Vector2D;

/// Solves the plp with a non-recursive 1-block strategy
///
/// Simplistic implementation that places all items contiguously as a single block, either
/// horizontally or vertically, depending on which of both places the most items.
pub fn plp_n1(bin: Vector2D<u32>, item: Vector2D<u32>) -> Vec<Placement> {
    let mut result: Vec<Placement> = Vec::new();

    if bin.x == 0 || bin.y == 0 || item.x == 0 || item.y == 0 {
        return result;
    }

    // How many can we place if item is placed horizontally?
    let nxh = bin.x / item.x;
    let nyh = bin.y / item.y;

    // How many can we place if item is placed vertically?
    let nxv = bin.x / item.y;
    let nyv = bin.y / item.x;

    // Emit block, depending on which case has most placements
    let o = if nxh * nyh > nxv * nyv { Orientation::Horizontal } else { Orientation::Vertical };
    let nx = if o == Orientation::Horizontal { nxh } else { nxv };
    let ny = if o == Orientation::Horizontal { nyh } else { nyv };
    let dx = if o == Orientation::Horizontal { item.x } else { item.y };
    let dy = if o == Orientation::Horizontal { item.y } else { item.x };
    for ix in 0..nx {
        for iy in 0..ny {
            let pos: Vector2D<u32> = Vector2D { x: ix * dx, y: iy * dy };
            result.push(Placement { pos, o });
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::plp::Orientation;

    use super::plp_n1;
    use vector2d::Vector2D;

    const ZERO: Vector2D<u32> = Vector2D { x: 0, y: 0 };

    #[test]
    fn test_empty_bin() {
        let bin: Vector2D<u32> = ZERO;
        let item: Vector2D<u32> = Vector2D { x: 20, y: 10 };
        assert!(plp_n1(bin, item).is_empty());
    }

    #[test]
    fn test_empty_item() {
        let bin: Vector2D<u32> = Vector2D { x: 10, y: 5 };
        let item: Vector2D<u32> = ZERO;
        assert!(plp_n1(bin, item).is_empty());
    }

    #[test]
    fn test_bin_smaller_than_item() {
        let bin: Vector2D<u32> = Vector2D { x: 10, y: 5 };
        let item: Vector2D<u32> = Vector2D { x: 20, y: 10 };
        assert!(plp_n1(bin, item).is_empty());
    }

    #[test]
    fn test_bin_equals_item_horizontally() {
        let bin: Vector2D<u32> = Vector2D { x: 10, y: 5 };
        let item: Vector2D<u32> = Vector2D { x: 10, y: 5 };
        let sut = plp_n1(bin, item);
        assert_eq!(sut.len(), 1);
        assert_eq!(sut.first().unwrap().pos, ZERO);
        assert_eq!(sut.first().unwrap().o, Orientation::Horizontal);
    }

    #[test]
    fn test_bin_equals_item_vertically() {
        let bin: Vector2D<u32> = Vector2D { x: 10, y: 5 };
        let item: Vector2D<u32> = Vector2D { x: 5, y: 10 };
        let sut = plp_n1(bin, item);
        assert_eq!(sut.len(), 1);
        assert_eq!(sut.first().unwrap().pos, ZERO);
        assert_eq!(sut.first().unwrap().o, Orientation::Vertical);
    }
    #[test]
    fn test_chooses_largest_orientation() {
        let bin: Vector2D<u32> = Vector2D { x: 1200, y: 800 };
        let item: Vector2D<u32> = Vector2D { x: 400, y: 300 };
        let sut = plp_n1(bin, item);
        assert_eq!(sut.len(), 8);
        for plcm in sut.iter() {
            assert_eq!(plcm.o, Orientation::Vertical);
        }
        let mut plcm_iter = sut.iter();
        assert_eq!(plcm_iter.next().unwrap().pos, Vector2D { x: 0, y: 0 });
        assert_eq!(plcm_iter.next().unwrap().pos, Vector2D { x: 0, y: 400 });
        assert_eq!(plcm_iter.next().unwrap().pos, Vector2D { x: 300, y: 0 });
        assert_eq!(plcm_iter.next().unwrap().pos, Vector2D { x: 300, y: 400 });
        assert_eq!(plcm_iter.next().unwrap().pos, Vector2D { x: 600, y: 0 });
        assert_eq!(plcm_iter.next().unwrap().pos, Vector2D { x: 600, y: 400 });
        assert_eq!(plcm_iter.next().unwrap().pos, Vector2D { x: 900, y: 0 });
        assert_eq!(plcm_iter.next().unwrap().pos, Vector2D { x: 900, y: 400 });
    }
}
