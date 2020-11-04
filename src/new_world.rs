#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CellState(pub u8);

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BlockInt(u8);

pub trait Rules {
	fn to_block_int(cells: [CellState; 4]) -> BlockInt;
	fn from_block_int(int: BlockInt) -> [CellState; 4];

	fn step1(&self) -> &[BlockInt];
	fn step2(&self) -> &[BlockInt];

	fn step1_invert(&self) -> &[BlockInt];
	fn step2_invert(&self) -> &[BlockInt];

	fn mouse_1(&self) -> CellState;
	fn mouse_2(&self) -> CellState;
	fn mouse_3(&self) -> CellState;

	fn default_state(&self) -> CellState;
}

pub struct RulesTwoStates {
	step1: [BlockInt; 16],
	step2: [BlockInt; 16],
	step1_invert: [BlockInt; 16],
	step2_invert: [BlockInt; 16],
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

impl RulesSingleState {
	pub fn from_one_step(step_permutation: [BlockInt; 16]) -> Option<Self> {
		let step_invert = invert!(16, u8, step_permutation);
		Some(
			Self {
				step1: step_permutation.clone(),
				step2: step_permutation,
				step1_invert: step_invert.clone(),
				step2_invert: step_invert,
			}
		)
	}

	pub fn from_two_steps(
		step1_permutation: [BlockInt; 16],
		step2_permutation: [BlockInt; 16],
	) -> Option<Self> {
		let step1_invert = invert!(16, u8, step1_permutation);
		let step2_invert = invert!(16, u8, step2_permutation);
		Some(
			Self {
				step1: step1_permutation,
				step2: step2_permutation,
				step1_invert,
				step2_invert,
			}
		)
	}
}

impl Rules for RulesTwoStates {
	fn to_block_int([a, b, c, d]: [CellState; 4]) -> BlockInt {
		BlockInt(
			(a.0 * 8) +
			(b.0 * 4) +
			(c.0 * 2) +
			d.0
		)
	}
	fn from_block_int(int: BlockInt) -> [CellState; 4] {
		[
			int.0 / 8 != 0,
			(int.0 / 4) % 2 != 0,
			(int.0 / 2) % 2 != 0,
			int.0 % 2 != 0,
		]
	}

	fn step1(&self) -> &[BlockInt] {
		self.step1
	}
	fn step2(&self) -> &[BlockInt] {
		self.step2
	}

	fn step1_invert(&self) -> &[BlockInt] {
		self.step1_invert
	}
	fn step2_invert(&self) -> &[BlockInt] {
		self.step2_invert
	}

	fn mouse_1(&self) -> CellState {
		CellState(1)
	}
	fn mouse_2(&self) -> CellState {
		CellState(0)
	}
	fn mouse_3(&self) -> CellState {
		CellState(0)
	}

	fn default_state(&self) -> CellState {
		CellState(0)
	}
}

pub struct RulesThreeStates {
	step1: [BlockInt; 81],
	step2: [BlockInt; 81],
	step1_invert: [BlockInt; 81],
	step2_invert: [BlockInt; 81],
}

impl RulesThreeStates {
	pub fn from_one_step(step_permutation: [BlockInt; 81]) -> Option<Self> {
		let step_invert = invert!(81, u8, step_permutation);
		Some(
			Self {
				step1: step_permutation.clone(),
				step2: step_permutation,
				step1_invert: step_invert.clone(),
				step2_invert: step_invert,
			}
		)
	}

	pub fn from_two_steps(
		step1_permutation: [BlockInt; 81],
		step2_permutation: [BlockInt; 81],
	) -> Option<Self> {
		let step1_invert = invert!(81, u8, step1_permutation);
		let step2_invert = invert!(81, u8, step2_permutation);
		Self {
			step1: step1_permutation,
			step2: step2_permutation,
			step1_invert,
			step2_invert,
		}
	}
}

impl Rules for RulesThreeStates {
	fn to_block_int([a, b, c, d]: [CellState; 4]) -> BlockInt {
		BlockInt(
			(a.0 * 27) +
			(b.0 * 9) +
			(c.0 * 3) +
			d.0
		)
	}
	fn from_block_int(int: BlockInt) -> [CellState; 4] {
		[
			int.0 / 27,
			(int.0 / 9) % 3,
			(int.0 / 3) % 3,
			int.0 % 3,
		]
	}

	fn step1(&self) -> &[BlockInt] {
		self.step1
	}
	fn step2(&self) -> &[BlockInt] {
		self.step2
	}

	fn step1_invert(&self) -> &[BlockInt] {
		self.step1_invert
	}
	fn step2_invert(&self) -> &[BlockInt] {
		self.step2_invert
	}

	fn mouse_1(&self) -> CellState {
		CellState(1)
	}
	fn mouse_2(&self) -> CellState {
		CellState(2)
	}
	fn mouse_3(&self) -> CellState {
		CellState(3)
	}

	fn default_state(&self) -> CellState {
		CellState(0)
	}
}

pub struct World {
	rules: Box<dyn Rules>,
	array: Vec<CellState>,
	width: usize,
	height: usize,
	intermediate_step: bool,
}

macro_rules! for_each_block {
	($offset: expr, $input:ident, $output: ident, $f:expr) => {
		for x in (0..self.width/2).map(|x| x * 2 + $offset) {
			for y in (0..self.height/2).map(|y| y * 2 + $offset) {
				let $input = self.get_block(x, y);
				let output;
				$f;
				self.set_block(x, y, output);
			}
		}
	};
}

impl World {
	pub fn new(rules: Box<dyn Rules>, halfwidth: usize, halfheight: usize) -> Self {
		let width = halfwidth * 2;
		let height = halfheight * 2;
		Self {
			rules,
			array: vec![rules.default_state(); width * height],
			width,
			height,
			intermediate_step: false,
		}
	}

	pub fn arr(&self) -> &[CellState] {
		&self.world.arr[..]
	}

	pub fn arr_mut(&mut self) -> &mut [CellState] {
		&mut self.world.arr[..]
	}

	pub fn set_new_size(&mut self, halfwidth: usize, halfheight: usize) {
		let new_width = halfwidth * 2;
		let new_height = halfheight * 2;
		let mut new_arr = vec![self.rules.default_state(); new_width * new_height];

		for y in 0..self.height.min(new_height) {
			for x in 0..self.width.min(new_width) {
				new_arr[x + y * new_width] = self.arr[x + y * self.width].clone();
			}
		}

		self.arr = new_arr;
		self.width = new_width;
		self.height = new_height;
	}

	pub fn get(&self, x: usize, y: usize) -> CellState {
		let x = normalize(x, self.width);
		let y = normalize(y, self.height);

		self.arr[x + y * self.width].clone()
	}

	pub fn set(&mut self, x: usize, y: usize, val: CellState) {
		let x = normalize(x, self.width);
		let y = normalize(y, self.height);

		self.arr[x + y * self.width] = val;
	}

	pub fn set_rect(&mut self, x: usize, y: usize, width: usize, height: usize, val: CellState) {
		for iy in 0..height {
		    for ix in 0..width {
		        self.set(x + ix, y + iy, val.clone());
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
			for_each_block!(1, current, output, { output = self.rules.step2()[current.into()].clone(); });
		} else {
			for_each_block!(0, current, output, { output = self.rules.step1()[current.into()].clone(); });
		}
		self.intermediate_step = !self.intermediate_step;
	}

	pub fn step_back(&mut self) {
		let rules = &self.rules;
		if self.intermediate_step {
			for_each_block!(1, current, output, { output = self.rules.step1_invert()[current.into()].clone(); });
		} else {
			for_each_block!(0, current, output, { output = self.rules.step2_invert()[current.into()].clone(); });
		}
		self.intermediate_step = !self.intermediate_step;
	}
}