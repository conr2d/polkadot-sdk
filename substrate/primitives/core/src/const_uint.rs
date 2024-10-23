// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::Get;

trait ConstBounded {
	const MIN: u128;
	const MAX: u128;
}

macro_rules! impl_const_bounded {
	($t:ty) => {
		impl ConstBounded for $t {
			const MIN: u128 = <$t>::MIN as u128;
			const MAX: u128 = <$t>::MAX as u128;
		}
	};
}

impl_const_bounded!(u8);
impl_const_bounded!(u16);
impl_const_bounded!(u32);
impl_const_bounded!(u64);
impl_const_bounded!(u128);
impl_const_bounded!(usize);

struct CheckOverflow<const N: u128, T: ConstBounded>(T);

impl<const N: u128, T: ConstBounded> CheckOverflow<N, T> {
	const ASSERTION: () = assert!(N <= T::MAX && N >= T::MIN);
}

/// Const getter for unsigned integers.
#[derive(Default, Clone)]
pub struct ConstUint<const N: u128>;

#[cfg(feature = "std")]
impl<const N: u128> std::fmt::Debug for ConstUint<N> {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		fmt.write_str(&format!("{}<{}>", stringify!(ConstUint), N))
	}
}

#[cfg(not(feature = "std"))]
impl<const N: u128> core::fmt::Debug for ConstUint<N> {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
		fmt.write_str("<wasm:stripped>")
	}
}

macro_rules! impl_const_uint {
	($t:ty) => {
		impl<const N: u128> Get<$t> for ConstUint<N> {
			fn get() -> $t {
				let _ = CheckOverflow::<N, $t>::ASSERTION;
				N as $t
			}
		}
		impl<const N: u128> Get<Option<$t>> for ConstUint<N> {
			fn get() -> Option<$t> {
				let _ = CheckOverflow::<N, $t>::ASSERTION;
				Some(N as $t)
			}
		}
	};
}

impl_const_uint!(u8);
impl_const_uint!(u16);
impl_const_uint!(u32);
impl_const_uint!(u64);
impl_const_uint!(u128);
impl_const_uint!(usize);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn const_uint_works() {
		assert_eq!(<ConstUint<42> as Get<u8>>::get(), 42);
		assert_eq!(<ConstUint<42> as Get<Option<u8>>>::get(), Some(42));
		assert_eq!(<ConstUint<42> as Get<u16>>::get(), 42);
		assert_eq!(<ConstUint<42> as Get<u32>>::get(), 42);
		assert_eq!(<ConstUint<42> as Get<u64>>::get(), 42);
		assert_eq!(<ConstUint<42> as Get<u128>>::get(), 42);
		assert_eq!(<ConstUint<42> as Get<usize>>::get(), 42);
		// compile-time error
		// assert_eq!(<ConstUint<256> as Get<u8>>::get(), 256);
	}
}
