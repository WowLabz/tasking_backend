#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
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
        traits::{ Randomness, Currency, tokens::ExistenceRequirement, LockIdentifier, WithdrawReasons, LockableCurrency },
		dispatch::{ EncodeLike },
        transactional
    };
	use sp_std::vec::Vec;
	// use codec::{EncodeLike};

	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	// NOTE: Need to refactor, duplicate code for testing purposes
	type AccountId<T> = <T as frame_system::Config>::AccountId;
	type Balance<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	pub const LOCKSECRET: LockIdentifier = *b"mylockab";

	#[derive(Encode, Decode, PartialEq, Eq, Debug, Clone, TypeInfo)]
	pub enum TaskTypeTags {
		WebDevelopment,
		MobileDevelopment,
		MachineLearning, 
		DeepLearning,
		FullStackDevelopment,
		CoreBlockchainDevelopment,
	}

	#[derive(Encode, Decode, PartialEq, Eq, Debug, Clone, TypeInfo)]
	pub enum Status {
		Open,
		InProgress,
		PendingApproval,
		PendingRatings,
		Completed,
	}

	impl Default for Status {
		fn default() -> Self {
			Status::Open
		}
	}

	#[derive(Encode, Decode, Default, Debug, PartialEq, Clone, Eq, TypeInfo)]
	pub struct TaskDetails<AccountId, Balance> {
		task_id: u128,
		publisher: AccountId,
		worker_id: Option<AccountId>,
		publisher_name: Option<Vec<u8>>,
		worker_name: Option<Vec<u8>>,
		task_tags: Vec<TaskTypeTags>,
		task_deadline: u64,
		cost: Balance,
		status: Status,
		task_description: Vec<u8>,
		attachments: Option<Vec<Vec<u8>>>
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: LockableCurrency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn get_task_count)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type TaskCount<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn task)]
	pub(super) type TaskStorage<T: Config> = StorageMap<_, Blake2_128Concat, u128, TaskDetails<T::AccountId, BalanceOf<T>>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		TaskCreated(T::AccountId, Vec<u8>, u128, u64, BalanceOf<T>, Vec<u8>)
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000)]
		pub fn create_task(
			origin: OriginFor<T>, 
			task_duration: u64, 
			task_cost: BalanceOf<T>, 
			task_des: Vec<u8>, 
			publisher_name: Vec<u8>, 
			task_tags: Vec<TaskTypeTags>, 
			publisher_attachments: Option<Vec<Vec<u8>>>
		) -> DispatchResult {
			let who = ensure_signed(origin).unwrap();
			let current_task_count = Self::get_task_count();
			// log::info!("$$$$$ Current task count: {:#?}", current_task_count);
			let result_from_locking = T::Currency::set_lock(LOCKSECRET, &who, task_cost.clone(), WithdrawReasons::TRANSACTION_PAYMENT);
			// log::info!("$$$$$ Locked amount of publisher: {:#?}", result_from_locking);
			let task_details = TaskDetails {
				task_id: current_task_count.clone(),
				publisher: who.clone(),
				worker_id: None,
				publisher_name: Some(publisher_name.clone()),
				worker_name: None,
				task_tags: task_tags.clone(),
				task_deadline: task_duration.clone(),
				cost:task_cost.clone(),
				status: Default::default(),
				task_description: task_des.clone(),
				attachments: publisher_attachments.clone(),
			};
			// log::info!("$$$$$ Task details: {:#?}", task_details);
			<TaskStorage<T>>::insert(current_task_count.clone(), task_details);
			Self::deposit_event(
				Event::TaskCreated(
					who, 
					publisher_name.clone(), 
					current_task_count.clone(), 
					task_duration.clone(), 
					task_cost.clone(), 
					task_des.clone()
				)
			);
			<TaskCount<T>>::put(current_task_count + 1);
			Ok(())
		}

		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
		// 	let who = ensure_signed(origin)?;
		// 	<Something<T>>::put(something);
		// 	Self::deposit_event(Event::SomethingStored(something, who));
		// 	Ok(())
		// }

		// #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		// pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
		// 	let _who = ensure_signed(origin)?;

		// 	match <Something<T>>::get() {
		// 		None => Err(Error::<T>::NoneValue)?,
		// 		Some(old) => {
		// 			let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
		// 			<Something<T>>::put(new);
		// 			Ok(())
		// 		},
		// 	}
		// }
	}
}

/*
#[weight = 10_000]
pub fn create_task(origin, task_duration: u64, task_cost: BalanceOf<T>, task_des: Vec<u8>, publisher_name: Vec<u8>, task_tags: Vec<TaskTypeTags>, publisher_attachments: Option<Vec<Vec<u8>>>) {
	let sender = ensure_signed(origin)?;
	let current_count = Self::get_task_count();

	let result_from_locking = T::Currency::set_lock(LOCKSECRET, &sender, task_cost.clone(), WithdrawReasons::TRANSACTION_PAYMENT);
	debug::info!("result_from_locking : {:#?}", result_from_locking);

	let temp= TaskDetails {
		task_id: current_count.clone(),
		publisher: sender.clone(),
		worker_id: None,
		publisher_name: Some(publisher_name.clone()),
		worker_name: None,
		task_tags: task_tags.clone(),
		task_deadline: task_duration.clone(),
		cost:task_cost.clone(),
		status: Default::default(),
		task_description: task_des.clone(),
		attachments: publisher_attachments.clone(),
	};

	TaskStorage::<T>::insert(current_count.clone(), temp);
	Self::deposit_event(RawEvent::TaskCreated(sender, publisher_name.clone(), current_count.clone(), task_duration.clone(), task_cost.clone(), task_des.clone()));
	TaskCount::put(current_count + 1);
}
*/
