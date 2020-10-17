#[derive(Eq, PartialEq, Debug, Clone)]
pub struct World {
	pub arr: Vec<u8>,
	width: usize,
	height: usize,
}

pub fn normalize(var: usize, size: usize) -> usize {
	if var >= size { var % size } else { var }
}

const CRITTERS_STEP1: [u8; 81] = [
	0,  1,  54, 3,  36, 7,  18, 5,  72, 
	9,  30, 19, 28, 39, 66, 21, 48, 75, 
	6,  11, 60, 15, 42, 69, 56, 51, 26, 
	27, 12, 55, 10, 37, 64, 57, 46, 73, 
	4,  31, 58, 13, 40, 67, 22, 49, 76, 
	63, 34, 61, 16, 43, 70, 25, 68, 79, 
	2,  29, 24, 33, 38, 65, 20, 47, 62, 
	45, 32, 59, 14, 41, 52, 23, 50, 77, 
	8 , 35, 74, 17, 44, 71, 78, 53, 80,
];

const CRITTERS_STEP2: [u8; 81] = [
	0,  27, 2,  9,  36, 7,  6,  5,  72, 
	3,  30, 19, 28, 13, 66, 21, 48, 75, 
	18, 11, 60, 15, 42, 69, 56, 51, 78, 
	1,  12, 55, 10, 31, 64, 57, 46, 73, 
	4,  37, 58, 39, 40, 67, 22, 49, 76, 
	63, 34, 61, 16, 43, 70, 25, 68, 79, 
	54, 29, 24, 33, 38, 65, 20, 47, 74, 
	45, 32, 59, 14, 41, 52, 23, 50, 77, 
	8,  35, 62, 17, 44, 71, 26, 53, 80,
];

lazy_static::lazy_static! {
    static ref CRITTERS_STEP1_INVERT: Vec<u8> = (0..81)
		.map(|x| {
			CRITTERS_STEP1
				.iter()
				.position(|&y| y == x)
				.unwrap() as u8
		})
		.collect();
    static ref CRITTERS_STEP2_INVERT: Vec<u8> = (0..81)
    	.map(|x| {
    		CRITTERS_STEP2
    			.iter()
    			.position(|&y| y == x)
    			.unwrap() as u8
    	})
    	.collect();
}

impl World {
	pub fn new(halfwidth: usize, halfheight: usize) -> World {
		World {
			arr: vec![0; halfwidth * halfheight * 2 * 2],
			width: halfwidth * 2,
			height: halfheight * 2,
		}
	}

	pub fn get(&self, x: usize, y: usize) -> u8 {
		let x = normalize(x, self.width);
		let y = normalize(y, self.height);

		self.arr[x + y * self.width]
	}

	pub fn set(&mut self, x: usize, y: usize, val: u8) {
		let x = normalize(x, self.width);
		let y = normalize(y, self.height);

		self.arr[x + y * self.width] = val;
	}

	pub fn set_rect(&mut self, x: usize, y: usize, width: usize, height: usize, val: u8) {
		for iy in 0..height {
		    for ix in 0..width {
		        self.set(x + ix, y + iy, val);
		    }
		}
	}

	fn get_block(&self, x: usize, y: usize) -> u8 {
		((self.get(x, y) as u8) * 27) +
		((self.get(x + 1, y) as u8) * 9) +
		((self.get(x, y + 1) as u8) * 3) +
		(self.get(x + 1, y + 1) as u8)
	}

	fn set_block(&mut self, x: usize, y: usize, val: u8) {
		self.set(x, y, val / 27);
		self.set(x+1, y, (val / 9) % 3);
		self.set(x, y+1, (val / 3) % 3);
		self.set(x+1, y+1, val % 3);
	}

	fn for_each_block<F: Fn(u8) -> u8>(&mut self, offset: usize, f: F) {
		for x in (0..self.width/2).map(|x| x * 2 + offset) {
			for y in (0..self.height/2).map(|y| y * 2 + offset) {
				self.set_block(x, y, f(self.get_block(x, y)));
			}
		}
	}

	pub fn step(&mut self) {
		self.for_each_block(0, |current| CRITTERS_STEP1[current as usize]);
		self.for_each_block(1, |current| CRITTERS_STEP2[current as usize]);
	}

	pub fn step_back(&mut self) {
		self.for_each_block(1, |current| CRITTERS_STEP2_INVERT[current as usize]);
		self.for_each_block(0, |current| CRITTERS_STEP1_INVERT[current as usize]);
	}
}
