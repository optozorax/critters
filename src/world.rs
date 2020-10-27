use std::fmt::Debug;
use std::marker::PhantomData;
use std::convert::{TryFrom};

pub fn normalize(var: usize, size: usize) -> usize {
	if var >= size { var % size } else { var }
}

pub trait InvertibleRules: Sized {
	type Value: Default + Clone + Debug;
	type BlockInt: Into<usize> + Clone;

	fn calculate() -> Option<Self>;

	fn to_block_int(
		a: Self::Value,
		b: Self::Value,
		c: Self::Value,
		d: Self::Value,
	) -> Self::BlockInt;

	fn from_block_int(int: Self::BlockInt) -> [Self::Value; 4];

	fn step1(&self) -> &[Self::BlockInt];
	fn step2(&self) -> &[Self::BlockInt];

	fn step1_invert(&self) -> &[Self::BlockInt];
	fn step2_invert(&self) -> &[Self::BlockInt];
}

#[derive(Clone, Debug)]
pub struct Critters {
	step: [u8; 16],
	step_invert: [u8; 16],
}

#[derive(Clone, Debug)]
pub struct Bowling {
	step: [u8; 16],
	step_invert: [u8; 16],
}

#[derive(Clone, Debug)]
pub struct TheTenetOfLife {
	step1: [u8; 81],
	step2: [u8; 81],
	step1_invert: [u8; 81],
	step2_invert: [u8; 81],
}

// In current Rust there is no const generics :(
macro_rules! invert {
	($size:literal, $type:ty, $from:ident) => {
		<[$type; $size]>::try_from(
			&(0..$size)
				.map(|x| {
					$from
						.iter()
						.position(|&y| y == x)
						.map(|x| u8::try_from(x).ok())
				})
				.collect::<Option<Option<Vec<u8>>>>()??[..]
		).ok()?
	};
}

impl InvertibleRules for Critters {
	type Value = bool;
	type BlockInt = u8;

	fn calculate() -> Option<Self> {
		let step = [
			0b1111, 0b1110, 0b1101, 0b0011,
			0b1011, 0b0101, 0b0110, 0b0001,
			0b0111, 0b1001, 0b1010, 0b0010,
			0b1100, 0b0100, 0b1000, 0b0000,
		];

		let step_invert = invert!(16, u8, step);

		Some(
			Self {
				step,
				step_invert,
			}
		)
	}

	fn to_block_int(
		a: Self::Value,
		b: Self::Value,
		c: Self::Value,
		d: Self::Value,
	) -> Self::BlockInt {
		((a as u8) * 8) +
		((b as u8) * 4) +
		((c as u8) * 2) +
		(d as u8)
	}

	fn from_block_int(int: Self::BlockInt) -> [Self::Value; 4] {
		[
			int / 8 != 0,
			(int / 4) % 2 != 0,
			(int / 2) % 2 != 0,
			int % 2 != 0,
		]
	}

	fn step1(&self) -> &[Self::BlockInt] { &self.step }
	fn step2(&self) -> &[Self::BlockInt] { &self.step }

	fn step1_invert(&self) -> &[Self::BlockInt] { &self.step_invert }
	fn step2_invert(&self) -> &[Self::BlockInt] { &self.step_invert }
}

impl InvertibleRules for Bowling {
	type Value = bool;
	type BlockInt = u8;

	fn calculate() -> Option<Self> {
		let step = [
			0b0000, 0b1000, 0b0100, 0b0011,
			0b0010, 0b0101, 0b1001, 0b0111,
			0b0001, 0b0110, 0b1010, 0b1011,
			0b1100, 0b1101, 0b1110, 0b1111,
		];

		let step_invert = invert!(16, u8, step);

		Some(
			Self {
				step,
				step_invert,
			}
		)
	}

	fn to_block_int(
		a: Self::Value,
		b: Self::Value,
		c: Self::Value,
		d: Self::Value,
	) -> Self::BlockInt {
		Critters::to_block_int(a, b, c, d)
	}

	fn from_block_int(int: Self::BlockInt) -> [Self::Value; 4] {
		Critters::from_block_int(int)
	}

	fn step1(&self) -> &[Self::BlockInt] { &self.step }
	fn step2(&self) -> &[Self::BlockInt] { &self.step }

	fn step1_invert(&self) -> &[Self::BlockInt] { &self.step_invert }
	fn step2_invert(&self) -> &[Self::BlockInt] { &self.step_invert }
}

impl InvertibleRules for TheTenetOfLife {
	type Value = u8;
	type BlockInt = u8;

	fn calculate() -> Option<Self> {
		let step1: [u8; 81] = [
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

		let step1_invert = invert!(81, u8, step1);

		let step2: [u8; 81] = [
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

		let step2_invert = invert!(81, u8, step2);

		Some(
			Self {
				step1,
				step2,
				step1_invert,
				step2_invert,
			}
		)
	}

	fn to_block_int(
		a: Self::Value,
		b: Self::Value,
		c: Self::Value,
		d: Self::Value,
	) -> Self::BlockInt {
		(a * 27) +
		(b * 9) +
		(c * 3) +
		d
	}

	fn from_block_int(int: Self::BlockInt) -> [Self::Value; 4] {
		[
			int / 27,
			(int / 9) % 3,
			(int / 3) % 3,
			int % 3,
		]
	}

	fn step1(&self) -> &[Self::BlockInt] { &self.step1 }
	fn step2(&self) -> &[Self::BlockInt] { &self.step2 }

	fn step1_invert(&self) -> &[Self::BlockInt] { &self.step1_invert }
	fn step2_invert(&self) -> &[Self::BlockInt] { &self.step2_invert }
}

#[derive(Debug, Clone)]
pub struct World<T: InvertibleRules> {
	rules: T,
	intermediate_step: bool,
	world: OnlyWorld<T>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct OnlyWorld<T: InvertibleRules> {
	arr: Vec<T::Value>,
	width: usize,
	height: usize,
	rules: PhantomData<T>,
}

impl<T: InvertibleRules> OnlyWorld<T> {
	fn new(halfwidth: usize, halfheight: usize) -> Self {
		Self {
			arr: vec![T::Value::default(); halfwidth * 2 * halfheight * 2],
			width: halfwidth * 2,
			height: halfheight * 2,
			rules: PhantomData,
		}
	}

	fn set_new_size(&mut self, halfwidth: usize, halfheight: usize) {
		let new_width = halfwidth * 2;
		let new_height = halfheight * 2;
		let mut new_arr = vec![T::Value::default(); new_width * new_height];

		for y in 0..self.height.min(new_height) {
			for x in 0..self.width.min(new_width) {
				new_arr[x + y * new_width] = self.arr[x + y * self.width].clone();
			}
		}

		self.arr = new_arr;
		self.width = new_width;
		self.height = new_height;
	}

	fn get(&self, x: usize, y: usize) -> T::Value {
		let x = normalize(x, self.width);
		let y = normalize(y, self.height);

		self.arr[x + y * self.width].clone()
	}

	fn set(&mut self, x: usize, y: usize, val: T::Value) {
		let x = normalize(x, self.width);
		let y = normalize(y, self.height);

		self.arr[x + y * self.width] = val;
	}

	fn set_rect(&mut self, x: usize, y: usize, width: usize, height: usize, val: T::Value) {
		for iy in 0..height {
		    for ix in 0..width {
		        self.set(x + ix, y + iy, val.clone());
		    }
		}
	}

	fn get_block(&self, x: usize, y: usize) -> T::BlockInt {
		T::to_block_int(
			self.get(x, y),
			self.get(x + 1, y),
			self.get(x, y + 1),
			self.get(x + 1, y + 1),
		)
	}

	#[allow(clippy::many_single_char_names)]
	fn set_block(&mut self, x: usize, y: usize, val: T::BlockInt) {
		let [a, b, c, d] = T::from_block_int(val);
		self.set(x, y, a);
		self.set(x+1, y, b);
		self.set(x, y+1, c);
		self.set(x+1, y+1, d);
	}

	fn for_each_block<F: Fn(T::BlockInt) -> T::BlockInt>(&mut self, offset: usize, f: F) {
		for x in (0..self.width/2).map(|x| x * 2 + offset) {
			for y in (0..self.height/2).map(|y| y * 2 + offset) {
				self.set_block(x, y, f(self.get_block(x, y)));
			}
		}
	}
}

impl<T: InvertibleRules> World<T> {
	pub fn new(rules: T, halfwidth: usize, halfheight: usize) -> Self {
		Self {
			rules,
			intermediate_step: false,
			world: OnlyWorld::new(halfwidth, halfheight),
		}
	}

	pub fn arr(&self) -> &[T::Value] {
		&self.world.arr[..]
	}

	pub fn arr_mut(&mut self) -> &mut [T::Value] {
		&mut self.world.arr[..]
	}

	pub fn set_new_size(&mut self, halfwidth: usize, halfheight: usize) {
		self.world.set_new_size(halfwidth, halfheight);
	}

	pub fn get(&self, x: usize, y: usize) -> T::Value {
		self.world.get(x, y)
	}

	pub fn set(&mut self, x: usize, y: usize, val: T::Value) {
		self.world.set(x, y, val);
	}

	pub fn set_rect(&mut self, x: usize, y: usize, width: usize, height: usize, val: T::Value) {
		self.world.set_rect(x, y, width, height, val);
	}

	pub fn step(&mut self) {
		let rules = &self.rules;
		if self.intermediate_step {
			self.world.for_each_block(1, |current| rules.step2()[current.into()].clone());
		} else {
			self.world.for_each_block(0, |current| rules.step1()[current.into()].clone());
		}
		self.intermediate_step = !self.intermediate_step;
	}

	pub fn is_intermediate_step(&mut self) -> bool {
		self.intermediate_step
	}

	pub fn step_back(&mut self) {
		let rules = &self.rules;
		if self.intermediate_step {
			self.world.for_each_block(0, |current| rules.step1_invert()[current.into()].clone());
		} else {
			self.world.for_each_block(1, |current| rules.step2_invert()[current.into()].clone());
		}
		self.intermediate_step = !self.intermediate_step;
	}
}
