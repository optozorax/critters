use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CellState(pub u8);

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BlockInt(pub u8);

pub trait Rules {
	fn to_block_int(&self, cells: [CellState; 4]) -> BlockInt;
	fn from_block_int(&self, int: BlockInt) -> [CellState; 4];

	fn step1(&self) -> &[BlockInt];
	fn step2(&self) -> &[BlockInt];

	fn step1_invert(&self) -> &[BlockInt];
	fn step2_invert(&self) -> &[BlockInt];

	fn mouse_1(&self) -> CellState;
	fn mouse_2(&self) -> CellState;
	fn mouse_3(&self) -> CellState;

	fn default_state(&self) -> CellState;

	fn normalize_state(&self, other_rules_state: CellState) -> CellState;

	fn clone_box(&self) -> Box<dyn Rules>;
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RulesTwoStates {
	step1: [BlockInt; 16],
	step2: [BlockInt; 16],
	step1_invert: [BlockInt; 16],
	step2_invert: [BlockInt; 16],
}

// In current Rust there is no const generics :(
macro_rules! invert {
	($size:literal, $from:ident) => {
		<[BlockInt; $size]>::try_from(
			&(0..$size)
				.map(|x| {
					$from
						.iter()
						.position(|&y| y.0 == x)
						.map(|x| u8::try_from(x).ok().map(BlockInt))
				})
				.collect::<Option<Option<Vec<BlockInt>>>>()??[..]
		).ok()?
	};
}

impl RulesTwoStates {
	pub fn from_one_step(step_permutation: [BlockInt; 16]) -> Option<Self> {
		let step_invert = invert!(16, step_permutation);
		Some(
			Self {
				step1: step_permutation,
				step2: step_permutation,
				step1_invert: step_invert,
				step2_invert: step_invert,
			}
		)
	}

	pub fn from_two_steps(
		step1_permutation: [BlockInt; 16],
		step2_permutation: [BlockInt; 16],
	) -> Option<Self> {
		let step1_invert = invert!(16, step1_permutation);
		let step2_invert = invert!(16, step2_permutation);
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
	fn to_block_int(&self, [a, b, c, d]: [CellState; 4]) -> BlockInt {
		BlockInt(
			(a.0 * 8) +
			(b.0 * 4) +
			(c.0 * 2) +
			d.0
		)
	}
	fn from_block_int(&self, int: BlockInt) -> [CellState; 4] {
		[
			CellState(int.0 / 8),
			CellState((int.0 / 4) % 2),
			CellState((int.0 / 2) % 2),
			CellState(int.0 % 2),
		]
	}

	fn step1(&self) -> &[BlockInt] {
		&self.step1
	}
	fn step2(&self) -> &[BlockInt] {
		&self.step2
	}

	fn step1_invert(&self) -> &[BlockInt] {
		&self.step1_invert
	}
	fn step2_invert(&self) -> &[BlockInt] {
		&self.step2_invert
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

	fn normalize_state(&self, other_rules_state: CellState) -> CellState {
		if other_rules_state.0 > 1 {
			CellState(1)
		} else {
			other_rules_state
		}
	}

	fn clone_box(&self) -> Box<dyn Rules> {
		Box::new(*self)
	}
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RulesThreeStates {
	step1: [BlockInt; 81],
	step2: [BlockInt; 81],
	step1_invert: [BlockInt; 81],
	step2_invert: [BlockInt; 81],
}

impl RulesThreeStates {
	pub fn from_one_step(step_permutation: [BlockInt; 81]) -> Option<Self> {
		let step_invert = invert!(81, step_permutation);
		Some(
			Self {
				step1: step_permutation,
				step2: step_permutation,
				step1_invert: step_invert,
				step2_invert: step_invert,
			}
		)
	}

	pub fn from_two_steps(
		step1_permutation: [BlockInt; 81],
		step2_permutation: [BlockInt; 81],
	) -> Option<Self> {
		let step1_invert = invert!(81, step1_permutation);
		let step2_invert = invert!(81, step2_permutation);
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

impl Rules for RulesThreeStates {
	fn to_block_int(&self, [a, b, c, d]: [CellState; 4]) -> BlockInt {
		BlockInt(
			(a.0 * 27) +
			(b.0 * 9) +
			(c.0 * 3) +
			d.0
		)
	}
	fn from_block_int(&self, int: BlockInt) -> [CellState; 4] {
		[
			CellState(int.0 / 27),
			CellState((int.0 / 9) % 3),
			CellState((int.0 / 3) % 3),
			CellState(int.0 % 3),
		]
	}

	fn step1(&self) -> &[BlockInt] {
		&self.step1
	}
	fn step2(&self) -> &[BlockInt] {
		&self.step2
	}

	fn step1_invert(&self) -> &[BlockInt] {
		&self.step1_invert
	}
	fn step2_invert(&self) -> &[BlockInt] {
		&self.step2_invert
	}

	fn mouse_1(&self) -> CellState {
		CellState(1)
	}
	fn mouse_2(&self) -> CellState {
		CellState(2)
	}
	fn mouse_3(&self) -> CellState {
		CellState(0)
	}

	fn default_state(&self) -> CellState {
		CellState(0)
	}

	fn normalize_state(&self, other_rules_state: CellState) -> CellState {
		if other_rules_state.0 > 2 {
			CellState(1)
		} else {
			other_rules_state
		}
	}

	fn clone_box(&self) -> Box<dyn Rules> {
		Box::new(*self)
	}
}
