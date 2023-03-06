//! Simple utility for drawing static images to the terminal.

use crate::GridPoint;
use glam::IVec2;

pub struct Canvas {
    size: IVec2,
    string: String,
}

impl Canvas {
    pub fn new(size: impl GridPoint) -> Canvas {
        let string = str::repeat(" ", size.len());

        Canvas {
            size: size.as_ivec2(),
            string,
        }
    }

    pub fn put(&mut self, pos: impl GridPoint, glyph: char) {
        let i = self.to_index(pos);
        self.string
            .replace_range(i..i + 1, std::str::from_utf8(&[glyph as u8]).unwrap());
    }

    fn to_index(&self, point: impl GridPoint) -> usize {
        let p = point.as_ivec2() + self.size / 2;
        let [x, y] = p.to_array();
        //println!("XY {}, {}, W {}", x, y, self.size.x);
        y as usize * self.size.x as usize + x as usize
    }

    pub fn print(&self) {
        let chars: Vec<_> = self.string.replace(' ', ".").chars().collect();
        for line in chars.chunks(self.size.x as usize).rev() {
            println!("{}", String::from_iter(line.iter()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Canvas;

    #[test]
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
