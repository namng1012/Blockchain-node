/// Money matters.
pub mod currency {
	use crate::Balance;

	pub const MILLICENTS: Balance = 1_000_000_000;
	pub const CENTS: Balance = 1_000 * MILLICENTS; // assume this is worth about a cent.
	pub const DOLLARS: Balance = 100 * CENTS;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 15 * CENTS + (bytes as Balance) * 6 * CENTS
	}
}

pub mod time {
	use crate::BlockNumber;

	pub const MINIMUMPERIOD: u64 = 1000;

	pub const SPENDPERIOD: BlockNumber = (MINIMUMPERIOD as BlockNumber) * 24;
}
