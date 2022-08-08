//! Simple utility for drawing static images to the terminal.

use glam::UVec2;

use crate::{GridPoint, Size2d};

pub struct Canvas {
    size: UVec2,
    string: String,
}

impl Canvas {
    pub fn new(size: impl Size2d) -> Canvas {
        let string = str::repeat(" ", size.len());

        Canvas {
            size: size.as_uvec2(),
            string,
        }
    }

    pub fn put(&mut self, pos: impl GridPoint, glyph: char) {
        let i = self.to_index(pos);
        self.string
            .replace_range(i..i + 1, std::str::from_utf8(&[glyph as u8]).unwrap());
    }

    fn to_index(&self, point: impl GridPoint) -> usize {
        let [x, y] = point.as_array();
        println!("XY {}, {}, W {}", x, y, self.size.x);
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
    fn print_test() {
        let mut canvas = Canvas::new([10, 5]);
        canvas.put([1, 1], '*');
        canvas.put([2, 2], '*');
        canvas.put([3, 3], '*');
        canvas.put([4, 4], '*');

        canvas.print();
    }
}
