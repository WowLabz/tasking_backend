#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	// use log::{info, trace, warn};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::{
		log,
        sp_runtime::traits::Hash,
        traits::{ 
            Randomness, 
            Currency, 
            tokens::ExistenceRequirement, 
            LockIdentifier, 
            WithdrawReasons, 
            LockableCurrency 
        },
		dispatch::{ EncodeLike },
        transactional
    };
	use sp_std::vec::Vec;
	// use codec::{EncodeLike};
	
	type AccountId<T> = <T as frame_system::Config>::AccountId;
	type Balance<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	
	/// Pallet configuration 
    #[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: LockableCurrency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

		
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {


	 }

	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Receiver should be a valid recipient and not the same as sender

		
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(100)]
		pub	fn court_summon(origin: OriginFor<T>,task_id: u128, )
		}


	}
}







