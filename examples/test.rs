use glam::IVec2;
use sark_grids::{*, geometry::*};

struct Emitter<'a> {
    shape: Box<dyn GridShape<Iterator=ShapeIter> + 'a>,
}

impl<'a> Emitter<'a> {
    pub fn new(shape: impl GridShape<Iterator=ShapeIter> + 'a) -> Self {
        Emitter { shape: Box::new(shape) }
    }
}

fn main() {
    let emitter = Emitter::new(GridRect::new([0,0], [10,10]));
    let emitter2 = Emitter::new(GridCircleFilled::new([0,0], 10));
}