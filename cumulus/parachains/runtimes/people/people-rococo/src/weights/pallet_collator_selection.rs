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

//! Autogenerated weights for `pallet_collator_selection`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-31, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("people-kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --chain=people-kusama-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_collator_selection
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./cumulus/parachains/runtimes/people/people-kusama/src/weights/pallet_collator_selection.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_collator_selection`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collator_selection::WeightInfo for WeightInfo<T> {
	/// Storage: Session NextKeys (r:100 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: CollatorSelection Invulnerables (r:0 w:1)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 100]`.
	fn set_invulnerables(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `214 + b * (78 ±0)`
		//  Estimated: `1203 + b * (2554 ±0)`
		// Minimum execution time: 14_702_000 picoseconds.
		Weight::from_parts(14_995_989, 0)
			.saturating_add(Weight::from_parts(0, 1203))
			// Standard Error: 2_975
			.saturating_add(Weight::from_parts(2_630_139, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 2554).saturating_mul(b.into()))
	}
	/// Storage: CollatorSelection DesiredCandidates (r:0 w:1)
	/// Proof: CollatorSelection DesiredCandidates (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_desired_candidates() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_916_000 picoseconds.
		Weight::from_parts(7_224_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: CollatorSelection CandidacyBond (r:0 w:1)
	/// Proof: CollatorSelection CandidacyBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn set_candidacy_bond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_035_000 picoseconds.
		Weight::from_parts(7_334_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: CollatorSelection Candidates (r:1 w:1)
	/// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(48002), added: 48497, mode: MaxEncodedLen)
	/// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	/// Proof: CollatorSelection DesiredCandidates (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: CollatorSelection Invulnerables (r:1 w:0)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Session NextKeys (r:1 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	/// Proof: CollatorSelection CandidacyBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 999]`.
	fn register_as_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1104 + c * (48 ±0)`
		//  Estimated: `49487 + c * (49 ±0)`
		// Minimum execution time: 42_377_000 picoseconds.
		Weight::from_parts(34_785_115, 0)
			.saturating_add(Weight::from_parts(0, 49487))
			// Standard Error: 1_226
			.saturating_add(Weight::from_parts(101_867, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 49).saturating_mul(c.into()))
	}
	/// Storage: CollatorSelection Candidates (r:1 w:1)
	/// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(48002), added: 48497, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// The range of component `c` is `[6, 1000]`.
	fn leave_intent(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `428 + c * (48 ±0)`
		//  Estimated: `49487`
		// Minimum execution time: 33_648_000 picoseconds.
		Weight::from_parts(24_533_176, 0)
			.saturating_add(Weight::from_parts(0, 49487))
			// Standard Error: 1_388
			.saturating_add(Weight::from_parts(103_733, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: System BlockWeight (r:1 w:1)
	/// Proof: System BlockWeight (max_values: Some(1), max_size: Some(48), added: 543, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	fn note_author() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `155`
		//  Estimated: `6196`
		// Minimum execution time: 44_705_000 picoseconds.
		Weight::from_parts(45_288_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Session NextKeys (r:1 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: CollatorSelection Invulnerables (r:1 w:1)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(641), added: 1136, mode: MaxEncodedLen)
	/// Storage: CollatorSelection Candidates (r:1 w:1)
	/// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(4802), added: 5297, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 19]`.
	/// The range of component `c` is `[1, 99]`.
	fn add_invulnerable(b: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `757 + b * (32 ±0) + c * (53 ±0)`
		//  Estimated: `6287 + b * (37 ±0) + c * (53 ±0)`
		// Minimum execution time: 52_720_000 picoseconds.
		Weight::from_parts(56_102_459, 0)
			.saturating_add(Weight::from_parts(0, 6287))
			// Standard Error: 12_957
			.saturating_add(Weight::from_parts(26_422, 0).saturating_mul(b.into()))
			// Standard Error: 2_456
			.saturating_add(Weight::from_parts(128_528, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 37).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 53).saturating_mul(c.into()))
	}
	/// Storage: CollatorSelection Invulnerables (r:1 w:1)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 100]`.
	fn remove_invulnerable(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `119 + b * (32 ±0)`
		//  Estimated: `4687`
		// Minimum execution time: 183_054_000 picoseconds.
		Weight::from_parts(197_205_427, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 13_533
			.saturating_add(Weight::from_parts(376_231, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: CollatorSelection Candidates (r:1 w:0)
	/// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(48002), added: 48497, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:999 w:0)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: CollatorSelection Invulnerables (r:1 w:0)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: System BlockWeight (r:1 w:1)
	/// Proof: System BlockWeight (max_values: Some(1), max_size: Some(48), added: 543, mode: MaxEncodedLen)
	/// Storage: System Account (r:995 w:995)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 1000]`.
	/// The range of component `c` is `[1, 1000]`.
	fn new_session(r: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `22815 + c * (97 ±0) + r * (116 ±0)`
		//  Estimated: `49487 + c * (2519 ±0) + r * (2602 ±0)`
		// Minimum execution time: 16_845_000 picoseconds.
		Weight::from_parts(16_962_000, 0)
			.saturating_add(Weight::from_parts(0, 49487))
			// Standard Error: 858_960
			.saturating_add(Weight::from_parts(30_464_644, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2519).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 2602).saturating_mul(r.into()))
	}
}