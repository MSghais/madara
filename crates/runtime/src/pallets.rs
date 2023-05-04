//! Configuration of the pallets used in the runtime.
//! The pallets used in the runtime are configured here.
//! This file is used to generate the `construct_runtime!` macro.

pub use frame_support::traits::{
    ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, OnTimestampSet, Randomness, StorageInfo,
};
pub use frame_support::weights::constants::{
    BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
};
pub use frame_support::weights::{IdentityFee, Weight};
pub use frame_support::{construct_runtime, parameter_types, StorageValue};
pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
/// Import the StarkNet pallet.
pub use pallet_starknet;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{ConstFeeMultiplier, CurrencyAdapter};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_runtime::generic;
use sp_runtime::traits::{AccountIdLookup, BlakeTwo256};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};
use sp_std::marker::PhantomData;

use crate::*;

// Configure FRAME pallets to include in runtime.

// --------------------------------------
// CUSTOM PALLETS
// --------------------------------------

/// Configure the Starknet pallet in pallets/starknet.
impl pallet_starknet::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type StateRoot = pallet_starknet::state_root::IntermediateStateRoot<Self>;
    type SystemHash = mp_starknet::crypto::hash::pedersen::PedersenHasher;
    type TimestampProvider = Timestamp;
    type UnsignedPriority = UnsignedPriority;
}

/// --------------------------------------
/// FRAME SYSTEM PALLET
/// --------------------------------------

/// Configuration of `frame_system` pallet.
impl frame_system::Config for Runtime {
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = frame_support::traits::Everything;
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = BlockWeights;
    /// The maximum length of a block (in bytes).
    type BlockLength = BlockLength;
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The aggregated dispatch type that is available for extrinsics.
    type RuntimeCall = RuntimeCall;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = AccountIdLookup<AccountId, ()>;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Index;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    /// The ubiquitous origin type.
    type RuntimeOrigin = RuntimeOrigin;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// Version of the runtime.
    type Version = Version;
    /// Converts a module to the index of the module in `construct_runtime!`.
    ///
    /// This type is being generated by `construct_runtime!`.
    type PalletInfo = PalletInfo;
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = ();
    /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
    type SS58Prefix = SS58Prefix;
    /// The set code logic, just the default since we're not a parachain.
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

// --------------------------------------
// CONSENSUS RELATED FRAME PALLETS
// --------------------------------------
// Notes:
// Aura is the consensus algorithm used for block production.
// Grandpa is the consensus algorithm used for block finalization.
// We want to support multiple flavors of consensus algorithms.
// Specifically we want to implement some proposals defined in the Starknet community forum.
// For more information see: https://community.starknet.io/t/starknet-decentralized-protocol-i-introduction/2671
// You can also follow this issue on github: https://github.com/keep-starknet-strange/madara/issues/83

/// Authority-based consensus protocol used for block production.
/// TODO: Comment and explain the rationale behind the configuration items.
impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<32>;
}

/// Deterministic finality mechanism used for block finalization.
/// TODO: Comment and explain the rationale behind the configuration items.
impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;

    type WeightInfo = ();
    type MaxAuthorities = ConstU32<32>;
    type MaxSetIdSessionEntries = ConstU64<0>;

    type KeyOwnerProof = sp_core::Void;
    type EquivocationReportSystem = ();
}

/// --------------------------------------
/// OTHER 3RD PARTY FRAME PALLETS
/// --------------------------------------

/// Timestamp manipulation.
/// For instance, we need it to set the timestamp of the Starkknet block.
impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = ConsensusOnTimestampSet<Self>;
    type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
    type WeightInfo = ();
}

/// Provides interaction with balances and accounts.
/// TODO: Comment and explain the rationale behind the configuration items.
impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

/// Provides the logic needed to handle transaction fees
impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = IdentityFee<Balance>;
    type LengthToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ConstFeeMultiplier<FeeMultiplier>;
}

/// Allows executing privileged functions.
/// Right now we use it to configure the fee token address for the Starknet pallet.
impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
}

parameter_types! {
    pub const UnsignedPriority: u64 = 1 << 20;
}

/// A stateless module with helpers for dispatch management which does no re-authentication.
/// We use this to enable batch dispatches.
impl pallet_utility::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type WeightInfo = ();
    type PalletsOrigin = OriginCaller;
}

/// Implement the OnTimestampSet trait to override the default Aura.
/// This is needed to support manual sealing.
pub struct ConsensusOnTimestampSet<T>(PhantomData<T>);
impl<T: pallet_aura::Config> OnTimestampSet<T::Moment> for ConsensusOnTimestampSet<T> {
    fn on_timestamp_set(moment: T::Moment) {
        if EnableManualSeal::get() {
            return;
        }
        <pallet_aura::Pallet<T> as OnTimestampSet<T::Moment>>::on_timestamp_set(moment)
    }
}
