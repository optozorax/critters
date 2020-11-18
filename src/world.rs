use crate::rules::{Rules, CellState, BlockInt};

pub fn normalize(var: usize, size: usize) -> usize {
	if var >= size { var % size } else { var }
}

pub struct World {
	rules: Box<dyn Rules>,
	array: Vec<CellState>,
	width: usize,
	height: usize,
	intermediate_step: bool,
}

macro_rules! for_each_block {
	($self: ident, $offset: expr, $input:ident, $output: ident, $f:expr) => {
		for x in (0..$self.width/2).map(|x| x * 2 + $offset) {
			for y in (0..$self.height/2).map(|y| y * 2 + $offset) {
				let $input = $self.get_block(x, y);
				let $output: BlockInt;
				$f;
				$self.set_block(x, y, $output);
			}
		}
	};
}

impl World {
	pub fn new(rules: Box<dyn Rules>, halfwidth: usize, halfheight: usize) -> Self {
		let width = halfwidth * 2;
		let height = halfheight * 2;
		let default_state = rules.default_state();
		Self {
			rules,
			array: vec![default_state; width * height],
			width,
			height,
			intermediate_step: false,
		}
	}

	pub fn get_rules(&self) -> &dyn Rules {
		&*self.rules
	}

	pub fn change_rules(&mut self, rules: Box<dyn Rules>) {
		self.array.iter_mut().for_each(|x| *x = rules.normalize_state(*x));
		self.rules = rules;
	}

	pub fn arr(&self) -> &[CellState] {
		&self.array[..]
	}

	pub fn arr_mut(&mut self) -> &mut [CellState] {
		&mut self.array[..]
	}

	pub fn set_new_size(&mut self, halfwidth: usize, halfheight: usize) {
		let new_width = halfwidth * 2;
		let new_height = halfheight * 2;
		let mut new_arr = vec![self.rules.default_state(); new_width * new_height];

		for y in 0..self.height.min(new_height) {
			for x in 0..self.width.min(new_width) {
				new_arr[x + y * new_width] = self.array[x + y * self.width];
			}
		}

		self.array = new_arr;
		self.width = new_width;
		self.height = new_height;
	}

	pub fn get(&self, x: usize, y: usize) -> CellState {
		let x = normalize(x, self.width);
		let y = normalize(y, self.height);

		self.array[x + y * self.width]
	}

	pub fn set(&mut self, x: usize, y: usize, val: CellState) {
		let x = normalize(x, self.width);
		let y = normalize(y, self.height);

		self.array[x + y * self.width] = val;
	}

	pub fn set_rect(&mut self, x: usize, y: usize, width: usize, height: usize, val: CellState) {
		for iy in 0..height {
		    for ix in 0..width {
		        self.set(x + ix, y + iy, val);
		    }
		}
	}

	pub fn is_intermediate_step(&mut self) -> bool {
		self.intermediate_step
	}

	fn get_block(&self, x: usize, y: usize) -> BlockInt {
		self.rules.to_block_int([
			self.get(x, y),
			self.get(x + 1, y),
			self.get(x, y + 1),
			self.get(x + 1, y + 1),
		])
	}

	#[allow(clippy::many_single_char_names)]
	fn set_block(&mut self, x: usize, y: usize, val: BlockInt) {
		let [a, b, c, d] = self.rules.from_block_int(val);
		self.set(x, y, a);
		self.set(x+1, y, b);
		self.set(x, y+1, c);
		self.set(x+1, y+1, d);
	}


	pub fn step(&mut self) {
		if self.intermediate_step {
			for_each_block!(self, 1, current, output, {
				output = self.rules.step2()[usize::from(current.0)];
			});
		} else {
			for_each_block!(self, 0, current, output, { 
				output = self.rules.step1()[usize::from(current.0)];
			});
		}
		self.intermediate_step = !self.intermediate_step;
	}

	pub fn step_back(&mut self) {
		if self.intermediate_step {
			for_each_block!(self, 0, current, output, {
				output = self.rules.step1_invert()[usize::from(current.0)];
			});
		} else {
			for_each_block!(self, 1, current, output, {
				output = self.rules.step2_invert()[usize::from(current.0)]; 
			});
		}
		self.intermediate_step = !self.intermediate_step;
	}
}
