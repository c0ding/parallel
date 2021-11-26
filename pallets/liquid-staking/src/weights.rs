// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_liquid_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-26, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("vanilla-dev"), DB CACHE: 128

// Executed Command:
// target/release/parallel
// benchmark
// --chain=vanilla-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet-liquid-staking
// --extrinsic=*
// --steps=50
// --repeat=20
// --heap-pages=4096
// --template=./.maintain/frame-weight-template.hbs
// --output=./pallets/liquid-staking/src/weights.rs


#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::all)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_liquid_staking.
pub trait WeightInfo {
	fn stake() -> Weight;
	fn unstake() -> Weight;
	fn bond() -> Weight;
	fn nominate() -> Weight;
	fn bond_extra() -> Weight;
	fn settlement() -> Weight;
	fn unbond() -> Weight;
	fn rebond() -> Weight;
	fn withdraw_unbonded() -> Weight;
	fn record_staking_settlement() -> Weight;
	fn set_liquid_currency() -> Weight;
	fn set_staking_currency() -> Weight;
	fn update_reserve_factor() -> Weight;
	fn update_staking_pool_capacity() -> Weight;
	fn update_xcm_fees_compensation() -> Weight;
	fn update_xcm_weight() -> Weight;
	fn add_insurances() -> Weight;
	fn payout_slashed() -> Weight;
	fn on_idle() -> Weight;
}

/// Weights for pallet_liquid_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn stake() -> Weight {
		(187_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(14 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	fn unstake() -> Weight {
		(166_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn bond() -> Weight {
		(144_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn nominate() -> Weight {
		(141_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn bond_extra() -> Weight {
		(145_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn settlement() -> Weight {
		(202_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn unbond() -> Weight {
		(140_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn rebond() -> Weight {
		(140_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn withdraw_unbonded() -> Weight {
		(172_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn record_staking_settlement() -> Weight {
		(50_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn set_liquid_currency() -> Weight {
		(4_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_staking_currency() -> Weight {
		(4_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn update_reserve_factor() -> Weight {
		(29_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn update_staking_pool_capacity() -> Weight {
		(29_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn update_xcm_fees_compensation() -> Weight {
		(28_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn update_xcm_weight() -> Weight {
		(30_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn add_insurances() -> Weight {
		(104_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn payout_slashed() -> Weight {
		(159_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn on_idle() -> Weight {
		(131_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn stake() -> Weight {
		(187_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(14 as Weight))
			.saturating_add(RocksDbWeight::get().writes(9 as Weight))
	}
	fn unstake() -> Weight {
		(166_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	fn bond() -> Weight {
		(144_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn nominate() -> Weight {
		(141_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn bond_extra() -> Weight {
		(145_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn settlement() -> Weight {
		(202_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(13 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	fn unbond() -> Weight {
		(140_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn rebond() -> Weight {
		(140_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn withdraw_unbonded() -> Weight {
		(172_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn record_staking_settlement() -> Weight {
		(50_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn set_liquid_currency() -> Weight {
		(4_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_staking_currency() -> Weight {
		(4_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn update_reserve_factor() -> Weight {
		(29_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn update_staking_pool_capacity() -> Weight {
		(29_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn update_xcm_fees_compensation() -> Weight {
		(28_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn update_xcm_weight() -> Weight {
		(30_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn add_insurances() -> Weight {
		(104_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn payout_slashed() -> Weight {
		(159_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(13 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	fn on_idle() -> Weight {
		(131_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
}
