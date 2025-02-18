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

//! Autogenerated weights for `pallet_assets`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-20, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("asset-hub-westend-dev"), DB CACHE: 1024

// Executed Command:
// target/production/polkadot-parachain
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/var/lib/gitlab-runner/builds/zyw4fam_/0/parity/mirrors/cumulus/.git/.artifacts/bench.json
// --pallet=pallet_assets
// --chain=asset-hub-westend-dev
// --header=./file_header.txt
// --output=./parachains/runtimes/assets/asset-hub-westend/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_assets`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_assets::WeightInfo for WeightInfo<T> {
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `107`
		//  Estimated: `4273`
		// Minimum execution time: 31_477_000 picoseconds.
		Weight::from_parts(32_036_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	fn force_create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `4273`
		// Minimum execution time: 13_084_000 picoseconds.
		Weight::from_parts(13_591_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	fn start_destroy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `276`
		//  Estimated: `4273`
		// Minimum execution time: 15_413_000 picoseconds.
		Weight::from_parts(16_009_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Account (r:1001 w:1000)
	/// Proof: ForeignAssets Account (max_values: None, max_size: Some(732), added: 3207, mode: MaxEncodedLen)
	/// Storage: System Account (r:1000 w:1000)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `c` is `[0, 1000]`.
	/// The range of component `c` is `[0, 1000]`.
	fn destroy_accounts(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + c * (208 ±0)`
		//  Estimated: `4273 + c * (3207 ±0)`
		// Minimum execution time: 18_503_000 picoseconds.
		Weight::from_parts(18_705_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			// Standard Error: 5_073
			.saturating_add(Weight::from_parts(12_123_380, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 3207).saturating_mul(c.into()))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Approvals (r:1001 w:1000)
	/// Proof: ForeignAssets Approvals (max_values: None, max_size: Some(746), added: 3221, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy_approvals(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `413 + a * (86 ±0)`
		//  Estimated: `4273 + a * (3221 ±0)`
		// Minimum execution time: 19_736_000 picoseconds.
		Weight::from_parts(20_044_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			// Standard Error: 3_446
			.saturating_add(Weight::from_parts(14_117_950, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 3221).saturating_mul(a.into()))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Metadata (r:1 w:0)
	/// Proof: ForeignAssets Metadata (max_values: None, max_size: Some(738), added: 3213, mode: MaxEncodedLen)
	fn finish_destroy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `4273`
		// Minimum execution time: 15_520_000 picoseconds.
		Weight::from_parts(16_042_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Account (r:1 w:1)
	/// Proof: ForeignAssets Account (max_values: None, max_size: Some(732), added: 3207, mode: MaxEncodedLen)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `4273`
		// Minimum execution time: 27_380_000 picoseconds.
		Weight::from_parts(27_731_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Account (r:1 w:1)
	/// Proof: ForeignAssets Account (max_values: None, max_size: Some(732), added: 3207, mode: MaxEncodedLen)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `350`
		//  Estimated: `4273`
		// Minimum execution time: 33_762_000 picoseconds.
		Weight::from_parts(34_201_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Account (r:2 w:2)
	/// Proof: ForeignAssets Account (max_values: None, max_size: Some(732), added: 3207, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `350`
		//  Estimated: `7404`
		// Minimum execution time: 45_731_000 picoseconds.
		Weight::from_parts(46_587_000, 0)
			.saturating_add(Weight::from_parts(0, 7404))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Account (r:2 w:2)
	/// Proof: ForeignAssets Account (max_values: None, max_size: Some(732), added: 3207, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `350`
		//  Estimated: `7404`
		// Minimum execution time: 40_758_000 picoseconds.
		Weight::from_parts(41_283_000, 0)
			.saturating_add(Weight::from_parts(0, 7404))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Account (r:2 w:2)
	/// Proof: ForeignAssets Account (max_values: None, max_size: Some(732), added: 3207, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `350`
		//  Estimated: `7404`
		// Minimum execution time: 46_265_000 picoseconds.
		Weight::from_parts(47_105_000, 0)
			.saturating_add(Weight::from_parts(0, 7404))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: ForeignAssets Asset (r:1 w:0)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Account (r:1 w:1)
	/// Proof: ForeignAssets Account (max_values: None, max_size: Some(732), added: 3207, mode: MaxEncodedLen)
	fn freeze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `350`
		//  Estimated: `4273`
		// Minimum execution time: 19_186_000 picoseconds.
		Weight::from_parts(19_697_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ForeignAssets Asset (r:1 w:0)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Account (r:1 w:1)
	/// Proof: ForeignAssets Account (max_values: None, max_size: Some(732), added: 3207, mode: MaxEncodedLen)
	fn thaw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `350`
		//  Estimated: `4273`
		// Minimum execution time: 18_930_000 picoseconds.
		Weight::from_parts(19_378_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	fn freeze_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `276`
		//  Estimated: `4273`
		// Minimum execution time: 15_091_000 picoseconds.
		Weight::from_parts(15_429_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	fn thaw_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `276`
		//  Estimated: `4273`
		// Minimum execution time: 14_820_000 picoseconds.
		Weight::from_parts(15_227_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Metadata (r:1 w:0)
	/// Proof: ForeignAssets Metadata (max_values: None, max_size: Some(738), added: 3213, mode: MaxEncodedLen)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `4273`
		// Minimum execution time: 16_832_000 picoseconds.
		Weight::from_parts(17_104_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	fn set_team() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `4273`
		// Minimum execution time: 15_212_000 picoseconds.
		Weight::from_parts(15_819_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ForeignAssets Asset (r:1 w:0)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Metadata (r:1 w:1)
	/// Proof: ForeignAssets Metadata (max_values: None, max_size: Some(738), added: 3213, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn set_metadata(n: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `4273`
		// Minimum execution time: 30_119_000 picoseconds.
		Weight::from_parts(30_931_884, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			// Standard Error: 2_734
			.saturating_add(Weight::from_parts(2_665, 0).saturating_mul(n.into()))
			// Standard Error: 2_734
			.saturating_add(Weight::from_parts(725, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ForeignAssets Asset (r:1 w:0)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Metadata (r:1 w:1)
	/// Proof: ForeignAssets Metadata (max_values: None, max_size: Some(738), added: 3213, mode: MaxEncodedLen)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `406`
		//  Estimated: `4273`
		// Minimum execution time: 30_740_000 picoseconds.
		Weight::from_parts(31_699_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ForeignAssets Asset (r:1 w:0)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Metadata (r:1 w:1)
	/// Proof: ForeignAssets Metadata (max_values: None, max_size: Some(738), added: 3213, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn force_set_metadata(n: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `81`
		//  Estimated: `4273`
		// Minimum execution time: 14_270_000 picoseconds.
		Weight::from_parts(14_791_008, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			// Standard Error: 1_649
			.saturating_add(Weight::from_parts(227, 0).saturating_mul(n.into()))
			// Standard Error: 1_649
			.saturating_add(Weight::from_parts(4_436, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ForeignAssets Asset (r:1 w:0)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Metadata (r:1 w:1)
	/// Proof: ForeignAssets Metadata (max_values: None, max_size: Some(738), added: 3213, mode: MaxEncodedLen)
	fn force_clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `406`
		//  Estimated: `4273`
		// Minimum execution time: 29_653_000 picoseconds.
		Weight::from_parts(30_256_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	fn force_asset_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `4273`
		// Minimum execution time: 13_914_000 picoseconds.
		Weight::from_parts(14_372_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Approvals (r:1 w:1)
	/// Proof: ForeignAssets Approvals (max_values: None, max_size: Some(746), added: 3221, mode: MaxEncodedLen)
	fn approve_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `276`
		//  Estimated: `4273`
		// Minimum execution time: 33_615_000 picoseconds.
		Weight::from_parts(34_064_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Approvals (r:1 w:1)
	/// Proof: ForeignAssets Approvals (max_values: None, max_size: Some(746), added: 3221, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Account (r:2 w:2)
	/// Proof: ForeignAssets Account (max_values: None, max_size: Some(732), added: 3207, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `520`
		//  Estimated: `7404`
		// Minimum execution time: 64_837_000 picoseconds.
		Weight::from_parts(65_668_000, 0)
			.saturating_add(Weight::from_parts(0, 7404))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Approvals (r:1 w:1)
	/// Proof: ForeignAssets Approvals (max_values: None, max_size: Some(746), added: 3221, mode: MaxEncodedLen)
	fn cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `446`
		//  Estimated: `4273`
		// Minimum execution time: 35_648_000 picoseconds.
		Weight::from_parts(36_163_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Approvals (r:1 w:1)
	/// Proof: ForeignAssets Approvals (max_values: None, max_size: Some(746), added: 3221, mode: MaxEncodedLen)
	fn force_cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `446`
		//  Estimated: `4273`
		// Minimum execution time: 36_452_000 picoseconds.
		Weight::from_parts(36_885_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	fn set_min_balance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `4273`
		// Minimum execution time: 15_963_000 picoseconds.
		Weight::from_parts(16_345_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ForeignAssets Account (r:1 w:1)
	/// Proof: ForeignAssets Account (max_values: None, max_size: Some(732), added: 3207, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn touch() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `345`
		//  Estimated: `4273`
		// Minimum execution time: 35_359_000 picoseconds.
		Weight::from_parts(36_006_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: ForeignAssets Account (r:1 w:1)
	/// Proof: ForeignAssets Account (max_values: None, max_size: Some(732), added: 3207, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	fn touch_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `4273`
		// Minimum execution time: 33_524_000 picoseconds.
		Weight::from_parts(34_090_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ForeignAssets Account (r:1 w:1)
	/// Proof: ForeignAssets Account (max_values: None, max_size: Some(732), added: 3207, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn refund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `471`
		//  Estimated: `4273`
		// Minimum execution time: 31_777_000 picoseconds.
		Weight::from_parts(32_319_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: ForeignAssets Account (r:1 w:1)
	/// Proof: ForeignAssets Account (max_values: None, max_size: Some(732), added: 3207, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Asset (r:1 w:1)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	fn refund_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `401`
		//  Estimated: `4273`
		// Minimum execution time: 30_160_000 picoseconds.
		Weight::from_parts(30_665_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ForeignAssets Asset (r:1 w:0)
	/// Proof: ForeignAssets Asset (max_values: None, max_size: Some(808), added: 3283, mode: MaxEncodedLen)
	/// Storage: ForeignAssets Account (r:1 w:1)
	/// Proof: ForeignAssets Account (max_values: None, max_size: Some(732), added: 3207, mode: MaxEncodedLen)
	fn block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `350`
		//  Estimated: `4273`
		// Minimum execution time: 19_011_000 picoseconds.
		Weight::from_parts(19_491_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
