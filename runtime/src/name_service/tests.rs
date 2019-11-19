/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok, parameter_types};
	use sr_primitives::{traits::{BlakeTwo256, IdentityLookup}, testing::Header};
	use sr_primitives::weights::Weight;
	use sr_primitives::Perbill;

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type WeightMultiplierUpdate = ();
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
	}

	impl Trait for Test  {
		type Event = ();
	}

	impl timestamp::Trait for Test {
		type Moment = u64;
		type OnTimestampSet = ();
        type MinimumPeriod = ();
	}

	impl balances::Trait for Test {
		type Balance = u128;
		type OnFreeBalanceZero = ();
		type OnNewAccount = ();
		type TransactionPayment = ();
		type TransferPayment = ();
		type DustRemoval = ();
		type Event = ();
		type ExistentialDeposit = ();
		type TransferFee = ();
		type CreationFee = ();
		type TransactionBaseFee = ();
		type TransactionByteFee = ();
		type WeightToFee = ();
	}
	
	type NamingServiceModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			//assert_ok!(TemplateModule::register_domain(""));
			// asserting that the stored value is equal to what we stored
			assert_eq!(NamingServiceModule::total_domains(), 0);
		});
	}

	#[test]
	fn test_register_domain() {
		with_externalities(&mut new_test_ext(), || {
			let alice = 1u64;
			let dummy_hash = H256([2; 32]);
			assert_ok!(NamingServiceModule::register_domain(Origin::signed(alice), dummy_hash));
			assert_eq!(NamingServiceModule::domain(dummy_hash).source, alice);
		});
	}

	// TODO: Test other functions with features
	// - Catching events after the event
	// - Set balance of the test account

	#[test]
	fn test_claim_auction() {
		with_externalities(&mut new_test_ext(), || {
			let alice = 1u64;
			let dummy_hash = H256([2; 32]);
			assert_ok!(NamingServiceModule::register_domain(Origin::signed(alice), dummy_hash));
			assert_eq!(NamingServiceModule::domain(dummy_hash).source, alice);
		});
	}
}
