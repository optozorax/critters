#[derive(Eq, PartialEq, Debug)]
pub struct World {
	pub arr: Vec<bool>,
	width: usize,
	height: usize,
	step_offset: bool,
}

fn normalize(var: usize, size: usize) -> usize {
	if var >= size { var % size } else { var }
}

const CRITTERS_ARRAY: [u8; 16] = [
	0b1111, 0b1110, 0b1101, 0b0011,
	0b1011, 0b0101, 0b0110, 0b0001,
	0b0111, 0b1001, 0b1010, 0b0010,
	0b1100, 0b0100, 0b1000, 0b0000,
];

lazy_static::lazy_static! {
    static ref CRITTERS_ARRAY_INVERT: Vec<u8> = (0..16).map(|x| CRITTERS_ARRAY.iter().position(|&y| y == x).unwrap() as u8).collect();
}

impl World {
	pub fn new(halfwidth: usize, halfheight: usize, step_offset: bool) -> World {
		World {
			arr: vec![false; halfwidth * halfheight * 2 * 2],
			width: halfwidth * 2,
			height: halfheight * 2,
			step_offset,
		}
	}

	pub fn get(&self, x: usize, y: usize) -> bool {
		let x = normalize(x, self.width);
		let y = normalize(y, self.height);

		self.arr[x + y * self.width]
	}

	pub fn set(&mut self, x: usize, y: usize, val: bool) {
		let x = normalize(x, self.width);
		let y = normalize(y, self.height);

		self.arr[x + y * self.width] = val;
	}

	fn get_block(&self, x: usize, y: usize) -> u8 {
		((self.get(x, y) as u8) << 3) +
		((self.get(x + 1, y) as u8) << 2) +
		((self.get(x, y + 1) as u8) << 1) +
		(self.get(x + 1, y + 1) as u8)
	}

	fn set_block(&mut self, x: usize, y: usize, val: u8) {
		self.set(x, y, val & 0b1000 != 0);
		self.set(x+1, y, val & 0b0100 != 0);
		self.set(x, y+1, val & 0b0010 != 0);
		self.set(x+1, y+1, val & 0b0001 != 0);
	}

	pub fn step(&mut self) {
		let offset = self.step_offset as usize;

		for x in (0..self.width/2).map(|x| x * 2 + offset) {
			for y in (0..self.height/2).map(|y| y * 2 + offset) {
				let current = self.get_block(x, y);
				let next = CRITTERS_ARRAY[current as usize];
				self.set_block(x, y, next);
			}
		}

		self.step_offset = !self.step_offset;
	}

	pub fn step_back(&mut self) {
		let offset = !self.step_offset as usize;

		for x in (0..self.width/2).map(|x| x * 2 + offset) {
			for y in (0..self.height/2).map(|y| y * 2 + offset) {
				let current = self.get_block(x, y);
				let next = CRITTERS_ARRAY_INVERT[current as usize];
				self.set_block(x, y, next);
			}
		}

		self.step_offset = !self.step_offset;
	}
}
