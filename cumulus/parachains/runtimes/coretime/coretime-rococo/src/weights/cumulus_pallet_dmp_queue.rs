// Copyright Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> cumulus_pallet_dmp_queue::WeightInfo for WeightInfo<T> {
		/// Storage: `DmpQueue::MigrationStatus` (r:1 w:1)
	/// Proof: `DmpQueue::MigrationStatus` (`max_values`: Some(1), `max_size`: Some(1028), added: 1523, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca7d95d3e948effbeccff2de2c182672836` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca7d95d3e948effbeccff2de2c182672836` (r:1 w:1)
	/// Storage: `MessageQueue::BookStateFor` (r:1 w:1)
	/// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MessageQueue::ServiceHead` (r:1 w:1)
	/// Proof: `MessageQueue::ServiceHead` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `MessageQueue::Pages` (r:0 w:1)
	/// Proof: `MessageQueue::Pages` (`max_values`: None, `max_size`: Some(65585), added: 68060, mode: `MaxEncodedLen`)
	fn on_idle_good_msg() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65696`
		//  Estimated: `69161`
		// Minimum execution time: 124_651_000 picoseconds.
		Weight::from_parts(127_857_000, 0)
			.saturating_add(Weight::from_parts(0, 69161))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `DmpQueue::MigrationStatus` (r:1 w:1)
	/// Proof: `DmpQueue::MigrationStatus` (`max_values`: Some(1), `max_size`: Some(1028), added: 1523, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca7d95d3e948effbeccff2de2c182672836` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca7d95d3e948effbeccff2de2c182672836` (r:1 w:1)
	fn on_idle_large_msg() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65659`
		//  Estimated: `69124`
		// Minimum execution time: 65_684_000 picoseconds.
		Weight::from_parts(68_039_000, 0)
			.saturating_add(Weight::from_parts(0, 69124))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `DmpQueue::MigrationStatus` (r:1 w:1)
	/// Proof: `DmpQueue::MigrationStatus` (`max_values`: Some(1), `max_size`: Some(1028), added: 1523, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca70f923ef3252d0166429d36d20ed665a8` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca70f923ef3252d0166429d36d20ed665a8` (r:1 w:1)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca772275f64c354954352b71eea39cfaca2` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca772275f64c354954352b71eea39cfaca2` (r:1 w:1)
	/// Storage: `MessageQueue::BookStateFor` (r:1 w:1)
	/// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MessageQueue::ServiceHead` (r:1 w:1)
	/// Proof: `MessageQueue::ServiceHead` (`max_values`: Some(1), `max_size`: Some(5), added: 500, mode: `MaxEncodedLen`)
	/// Storage: `MessageQueue::Pages` (r:0 w:1)
	/// Proof: `MessageQueue::Pages` (`max_values`: None, `max_size`: Some(65585), added: 68060, mode: `MaxEncodedLen`)
	fn on_idle_overweight_good_msg() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65726`
		//  Estimated: `69191`
		// Minimum execution time: 117_657_000 picoseconds.
		Weight::from_parts(122_035_000, 0)
			.saturating_add(Weight::from_parts(0, 69191))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `DmpQueue::MigrationStatus` (r:1 w:1)
	/// Proof: `DmpQueue::MigrationStatus` (`max_values`: Some(1), `max_size`: Some(1028), added: 1523, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca754904d6d8c6fe06c4e5965f9b8397421` (r:1 w:0)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca70f923ef3252d0166429d36d20ed665a8` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca70f923ef3252d0166429d36d20ed665a8` (r:1 w:1)
	/// Storage: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca772275f64c354954352b71eea39cfaca2` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xcd5c1f6df63bc97f4a8ce37f14a50ca772275f64c354954352b71eea39cfaca2` (r:1 w:1)
	fn on_idle_overweight_large_msg() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65689`
		//  Estimated: `69154`
		// Minimum execution time: 59_799_000 picoseconds.
		Weight::from_parts(61_354_000, 0)
			.saturating_add(Weight::from_parts(0, 69154))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}