
//! Autogenerated weights for `pallet_template`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-25, STEPS: `30`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/node-template
// benchmark
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_template
// --extrinsic
// *
// --steps
// 30
// --repeat
// 20
// --output
// weights.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_template`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_template::WeightInfo for WeightInfo<T> {
	// Storage: TemplateModule Something (r:0 w:1)
	fn store_something(_s: u32, ) -> Weight {
		(26_496_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn benign_repeat_hashing(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 2_000
			.saturating_add((659_000 as Weight).saturating_mul(i as Weight))
	}
}
