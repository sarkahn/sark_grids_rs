use crate::{GridPoint, GridShape, GridSize};
use glam::UVec2;

pub struct Canvas {
    size: UVec2,
    string: String,
}

impl Canvas {
    pub fn new(size: impl GridSize) -> Canvas {
        let string = str::repeat(".", size.tile_count());

        Canvas {
            size: size.to_uvec2(),
            string,
        }
    }

    pub fn put(&mut self, pos: impl GridPoint, glyph: char) {
        let i = self.to_index(pos);
        self.string
            .replace_range(i..i + 1, std::str::from_utf8(&[glyph as u8]).unwrap());
    }

    pub fn put_bl(&mut self, pos: impl GridPoint, glyph: char) {
        let i = self.to_index(pos.to_ivec2() - self.size.as_ivec2() / 2);
        self.string
            .replace_range(i..i + 1, std::str::from_utf8(&[glyph as u8]).unwrap());
    }

    fn to_index(&self, point: impl GridPoint) -> usize {
        let p = point.to_ivec2() + self.size.as_ivec2() / 2;
        p.as_index(self.size)
    }

    pub fn print(&self) {
        let chars: Vec<_> = self.string.chars().collect();
        for line in chars.chunks(self.size.x as usize).rev() {
            println!("{}", String::from_iter(line.iter()));
        }
    }

    pub fn put_shape(&mut self, shape: impl GridShape, glyph: char) {
        for p in shape.iter() {
            self.put(p, glyph);
        }
    }

    pub fn clear(&mut self) {
        self.string = str::repeat(".", self.size.tile_count());
    }
}

#[cfg(test)]
mod tests {
    use super::Canvas;

    #[test]
    #[ignore]
    fn odd() {
        let mut canvas = Canvas::new([5, 5]);
        canvas.put([-2, -2], '*');
        canvas.put([-1, -1], '*');
        canvas.put([0, 0], '*');
        canvas.put([1, 1], '*');
        canvas.put([2, 2], '*');

        canvas.print();
    }

    #[test]
    #[ignore]
    fn even() {
        let mut canvas = Canvas::new([6, 6]);
        canvas.put([-3, -3], '*');
        canvas.put([-2, -2], '*');
        canvas.put([-1, -1], '*');
        canvas.put([0, 0], '*');
        canvas.put([1, 1], '*');
        canvas.put([2, 2], '*');

        canvas.print();
    }
}
