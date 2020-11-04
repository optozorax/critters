use std::convert::{TryFrom, TryInto};
use num_bigint::BigUint;
use crate::new_world::BlockInt;

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PermutationString {
	pub string: String, 
	pub permutation_size: u8,
}

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PermutationInt {
	pub int: BigUint, 
	pub permutation_size: u8,
}

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PermutationIndex(pub Vec<u8>);

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PermutationArray(pub Vec<BlockInt>);

impl TryFrom<PermutationString> for PermutationInt {
	type Error = &'static str;

	fn try_from(value: PermutationString) -> Result<Self, Self::Error> {
		unimplemented!()
	}
}

impl From<PermutationInt> for PermutationString {
	fn from(value: PermutationInt) -> Self {
		unimplemented!()
	}
}

impl TryFrom<PermutationInt> for PermutationIndex {
	type Error = &'static str;

	fn try_from(value: PermutationInt) -> Result<Self, Self::Error> {
		unimplemented!()
	}
}

impl TryFrom<PermutationIndex> for PermutationInt {
	type Error = &'static str;

	fn try_from(value: PermutationIndex) -> Result<Self, Self::Error> {
		// let len = value.0.len();
		// if !(0..len).all(|x| value.0.iter().find(|y| x == y)) {
		// 	return Err("input is not permutation array");
		// }

		unimplemented!()
	}
}

impl TryFrom<PermutationIndex> for PermutationArray {
	type Error = &'static str;

	fn try_from(value: PermutationIndex) -> Result<Self, Self::Error> {
		let len: u8 = value.0.len().try_into().map_err(|_| "too big for permutation index")?;
		let mut indexes = (0..len).map(|x| BlockInt(x)).collect::<Vec<BlockInt>>();

		let result = value.0
			.iter()
			.map(|x| {
				if usize::from(*x) <= indexes.len() {
					Some(indexes.remove(usize::from(*x)))
				} else {
					None
				}
			})
			.collect::<Option<Vec<BlockInt>>>().ok_or("not permutation index")?;

		Ok(PermutationArray(result))
	}
}

impl TryFrom<PermutationArray> for PermutationIndex {
	type Error = &'static str;

	fn try_from(value: PermutationArray) -> Result<Self, Self::Error> {
		let len: u8 = value.0.len().try_into().map_err(|_| "too big for permutation array")?;
		let mut indexes = (0..len).map(|x| BlockInt(x)).collect::<Vec<BlockInt>>();

		let result = value.0
			.iter()
			.map(|x| {
				indexes
					.iter()
					.position(|y| y == x)
					.map(|pos| { 
						indexes.remove(pos); 
						u8::try_from(pos).unwrap()
					})
			})
			.collect::<Option<Vec<u8>>>().ok_or("not permutation array")?;

		Ok(PermutationIndex(result))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn index() {
		macro_rules! arr2indx {
			($($a:expr),* => $($b:expr),*) => {
				assert_eq!(
					PermutationIndex::try_from(PermutationArray(vec![$($a),*].into_iter().map(BlockInt).collect())), 
					Ok(PermutationIndex(vec![$($b),*]))
				);

				assert_eq!(
					PermutationArray::try_from(PermutationIndex(vec![$($b),*])), 
					Ok(PermutationArray(vec![$($a),*].into_iter().map(BlockInt).collect()))
				);
			};
		}

		arr2indx!(1, 2, 3 => 0, 0, 0);
		arr2indx!(1, 3, 2 => 0, 1, 0);
		arr2indx!(2, 1, 3 => 1, 0, 0);
		arr2indx!(2, 3, 1 => 1, 1, 0);
		arr2indx!(3, 1, 2 => 2, 0, 0);
		arr2indx!(3, 2, 1 => 2, 1, 0);
	}

	#[test]
	fn number() {
		indx2num!(0, 0, 0 => 3 0);
		indx2num!(0, 1, 0 => 3 1);
		indx2num!(1, 0, 0 => 3 2);
		indx2num!(1, 1, 0 => 3 3);
		indx2num!(2, 0, 0 => 3 4);
		indx2num!(2, 1, 0 => 3 5);
	}

	#[test]
	fn string() {
		string!(3 0 => 3 "a");
		string!(3 1 => 3 "b");
		string!(3 2 => 3 "c");
		string!(3 3 => 3 "d");
		string!(3 4 => 3 "e");
		string!(3 5 => 3 "f");
	}
}

/*

    arr    indx   int            str
	1 2 3  0 0 0  0 = 0*1 + 0*2  a
	1 3 2  0 1 0  1 = 1*1 + 0*2  b
	2 1 3  1 0 0  2 = 0*2 + 2*2  c
	2 3 1  1 1 0  3 = 1*1 + 1*2  d
	3 1 2  2 0 0  4 = 0*1 + 2*2  e
	3 2 1  2 1 0  5 = 1*1 + 2*2  f

	numsys 3 2 1
	
*/