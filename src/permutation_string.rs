use num_bigint::BigUint;
use new_world::BlockInt;

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct PermutationString(String, u8);

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct PermutationInt(BigUint, u8);

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct PermutationIndex(Vec<u8>);

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct PermutationArray(Vec<BlockInt>);

impl TryFrom<PermutationString> for PermutationInt {
	type Error = &'static str;

	fn try_from(value: PermutationString) -> Result<Self, Self::Error> {
	}
}

impl From<PermutationInt> for PermutationString {
	fn from(value: PermutationInt) -> Self {
	}
}

impl TryFrom<PermutationInt> for PermutationIndex {
	type Error = &'static str;

	fn try_from(value: PermutationInt) -> Result<Self, Self::Error> {
	}
}

impl TryFrom<PermutationIndex> for PermutationInt {
	type Error = &'static str;

	fn try_from(value: PermutationIndex) -> Result<Self, Self::Error> {
		let len = value.len();
		if !(0..len).all(|x| value.iter().find(|y| x == y)) {
			return Err("input is not permutation array");
		}

		value.iter().enumerate()
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
	
*/