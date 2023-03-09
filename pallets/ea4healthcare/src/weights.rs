#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn add_info(n: u32) -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);

impl <T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn add_info(n: u32) -> Weight {
		(Weight::from_ref_time(10_000))
			.add(n as u64)
			.saturating_add(Weight::from_ref_time(500))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

impl WeightInfo for () {
	fn add_info(n: u32) -> Weight {
		(Weight::from_ref_time(10_000))
			.add(n as u64)
			.saturating_add(Weight::from_ref_time(500))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}