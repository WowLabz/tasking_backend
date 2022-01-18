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

	#[derive(Encode, Decode, PartialEq, Eq, Debug, Clone, TypeInfo)]
	pub enum UserType {
		Customer,
		Worker,
	}

	impl Default for UserType {
		fn default() -> Self {
			UserType::Worker
		}
	}

	#[derive(Encode, Decode, Debug, PartialEq, Clone, Eq, Default, TypeInfo)]
	pub struct User<AccountId> {
		account_id: AccountId,
		user_type: UserType,
		rating: Option<u8>,
		ratings_vec: Vec<u8>,
	}

	impl<AccountId> User<AccountId> {
		pub fn new(account_id: AccountId, user_type: UserType, ratings_vec: Vec<u8>) -> Self {
			let rating = Some(Self::get_list_average(ratings_vec.clone()));
			

			Self {
				account_id,
				user_type,
				rating,
				ratings_vec,
			}
		}

		pub fn get_list_average(list: Vec<u8>) -> u8 {
			
			let list_len: u8 = list.len() as u8;

			if list_len == 1 {
				return list[0];
			}

			let mut total_sum = 0;
			for item in list.iter() {
				total_sum += item;
			}
			
			let average = total_sum / list_len;
			
			average
		}
	}

	#[derive(Encode, Decode, Default, Debug, PartialEq, Clone, Eq, TypeInfo)]
	pub struct TransferDetails<AccountId, Balance> {
		transfer_from: AccountId,
		from_before: Balance,
		from_after: Balance,
		transfer_to: AccountId,
		to_before: Balance,
		to_after: Balance,
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

	#[pallet::storage]
	#[pallet::getter(fn get_worker_ratings)]
	pub(super) type WorkerRatings<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, User<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_customer_ratings)]
	pub(super) type CustomerRatings<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, User<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_account_balances)]
	pub(super) type AccountBalances<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_count)]
	pub(super) type Count<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_transfers)]
	pub(super) type Transfers<T: Config> = StorageValue<_, Vec<TransferDetails<T::AccountId, BalanceOf<T>>>, ValueQuery>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		TaskCreated(T::AccountId, Vec<u8>, u128, u64, BalanceOf<T>, Vec<u8>),
		TaskIsBid(T::AccountId, Vec<u8>, u128),
		TaskCompleted(T::AccountId, u128, T::AccountId),
		TaskApproved(u128),
		AmountTransfered(T::AccountId, T::AccountId, BalanceOf<T>),
		TaskClosed(u128),
		AccBalance(T::AccountId, BalanceOf<T>),
		CountIncreased(u128),
		TransferMoney(T::AccountId, BalanceOf<T>, BalanceOf<T>, T::AccountId, BalanceOf<T>, BalanceOf<T>)
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// To ensure that the  task exists
		TaskDoesNotExist,
		/// To check the status and availibility of the task
		TaskIsNotOpen,
		/// To check balance of bidder for ability to stake amount
		NotEnoughBalanceToBid,
		/// To ensure publisher does not bid for the same task posted
		UnauthorisedToBid,
		/// To ensure that a task is bid for and is in progress
		TaskIsNotInProgress,
		/// To ensure a worker is chosen and is assigned to the task
		WorkerNotSet,
		/// To ensure only the assigned worker completes the task
		UnauthorisedToComplete,
		/// To ensure task status is completed and is waiting for approval from the publisher
		TaskIsNotPendingApproval,
		/// To ensure only the publisher approves the task
		UnauthorisedToApprove,
		/// To ensure task is approved by the publisher
		TaskIsNotPendingRating,
		/// To ensure the worker only provides the publisher rating
		UnauthorisedToProvideCustomerRating,
		/// TO check if the sender has sufficient balance for a transfer
		NotEnoughBalance

	}

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
			// Extrinsic for creating tasks on the blockchain. This is called by the ..
			// publisher who wants to post tasks on the chain that can be ..
			// put up for bidding.
			let who = ensure_signed(origin)?;
			let current_task_count = Self::get_task_count();
			let result_from_locking = T::Currency::set_lock(
				LOCKSECRET, 
				&who, 
				task_cost.clone(), 
				WithdrawReasons::TRANSACTION_PAYMENT
			);
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

		#[pallet::weight(10_000)]
		pub fn bid_for_task(
			origin:OriginFor<T>, 
			task_id:u128, 
			worker_name:Vec<u8>
		) -> DispatchResult {
			// Extrinsic for bidding for tasks on the blockchain. This is called by ..
			// the worker who wants to take up tasks on the chain that ..
			// can be completed in the given span of time.
			let bidder = ensure_signed(origin)?;
			ensure!(
				<TaskStorage<T>>::contains_key(&task_id), 
				<Error<T>>::TaskDoesNotExist
			);
			let mut task = Self::task(task_id.clone());
			let task_cost = task.cost.clone();
			ensure!(
				T::Currency::free_balance(&bidder.clone()) > task_cost, 
				<Error<T>>::NotEnoughBalanceToBid
			);
			let publisher = task.publisher.clone();
			ensure!(
				publisher != bidder.clone(), 
				<Error<T>>::UnauthorisedToBid
			);
			let status = task.status.clone();
			ensure!(status == Status::Open, <Error<T>>::TaskIsNotOpen);
			task.worker_id = Some(bidder.clone());
			task.worker_name = Some(worker_name.clone());
			task.status = Status::InProgress;
			<TaskStorage<T>>::insert(&task_id,task);
			T::Currency::set_lock(
				LOCKSECRET, 
				&bidder, 
				task_cost.clone(), 
				WithdrawReasons::TRANSACTION_PAYMENT
			);		
			Self::deposit_event(
				Event::TaskIsBid(bidder.clone(), 
				worker_name.clone(), 
				task_id.clone())
			);
			Ok(())
		}

		#[pallet::weight(10_000)]
		// NOTE: work_attachments should not be an option, should be mandatory for the jurors to review work in case of a dispute
		pub fn task_completed(origin:OriginFor<T>, task_id:u128, worker_attachments: Option<Vec<Vec<u8>>>)-> DispatchResult{
			let bidder = ensure_signed(origin)?;
			ensure!(<TaskStorage<T>>::contains_key(task_id.clone()), <Error<T>>::TaskDoesNotExist);

			let mut task = Self::task(task_id.clone());
			let status = task.status;
			ensure!(status == Status::InProgress, <Error<T>>::TaskIsNotInProgress);

			let publisher = task.publisher.clone();
			let worker = task.worker_id.clone().ok_or(<Error<T>>::WorkerNotSet)?;

			// only the worker can complete the task
            ensure!(worker == bidder.clone(), <Error<T>>::UnauthorisedToComplete);

			task.status = Status::PendingApproval;

			// Update the attachments vector to hold both publisher and worker file urls
			let existing_attachments = task.attachments.clone();
			let mut updated_attachments: Vec<Vec<u8>> = Vec::new();

			// update only if attachments exist 
            if let Some(attachments) =  existing_attachments {
                updated_attachments.extend(attachments.clone());
            }

			// update only if attachments exist 
            if let Some(work_attachments) =  worker_attachments {
                updated_attachments.extend(work_attachments.clone());
            }

			task.attachments = Some(updated_attachments);

			<TaskStorage<T>>::insert(&task_id,task.clone());
			Self::deposit_event(Event::TaskCompleted(worker.clone(), task_id.clone(), publisher.clone()));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn approve_task(origin:OriginFor<T>, task_id:u128, rating_for_the_worker:u8)-> DispatchResult{
			
			let publisher = ensure_signed(origin)?;
			ensure!(<TaskStorage<T>>::contains_key(task_id.clone()), <Error<T>>::TaskDoesNotExist);

			let mut task = Self::task(task_id.clone());
			let status = task.status;
			ensure!(status == Status::PendingApproval, <Error<T>>::TaskIsNotPendingApproval);
			let approver = task.publisher.clone();

			ensure!(publisher == approver.clone(), <Error<T>>::UnauthorisedToApprove);

			let bidder = task.worker_id.clone().ok_or(<Error<T>>::WorkerNotSet)?;

			// Inserting Worker Rating to RatingMap
			let existing_bidder_ratings: User<T::AccountId> = Self::get_worker_ratings(&bidder);

			let mut temp_rating_vec = Vec::<u8>::new();
			for rating in existing_bidder_ratings.ratings_vec {
				temp_rating_vec.push(rating);
			}
			temp_rating_vec.push(rating_for_the_worker);

			let curr_bidder_ratings = User::new(bidder.clone(), UserType::Worker, temp_rating_vec);
			<WorkerRatings<T>>::insert(bidder.clone(), curr_bidder_ratings.clone());

			// Updating Task Status
			task.status = Status::PendingRatings;
			<TaskStorage<T>>::insert(&task_id,task.clone());
			Self::deposit_event(Event::TaskApproved(task_id.clone()));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn provide_customer_rating(origin:OriginFor<T>, task_id:u128, rating_for_customer:u8)-> DispatchResult{

			let bidder = ensure_signed(origin)?;

			let mut task = Self::task(task_id.clone());
			let status = task.status;
			ensure!(status == Status::PendingRatings, <Error<T>>::TaskIsNotPendingRating);

			let worker = task.worker_id.clone().ok_or(<Error<T>>::WorkerNotSet)?;

			// only the bidder/worker should be able to provide customer ratings
			ensure!(worker == bidder.clone(), <Error<T>>::UnauthorisedToProvideCustomerRating);

			let customer = &task.publisher;

			let existing_customer_rating: User<T::AccountId> = Self::get_customer_ratings(&customer);

			let mut temp_rating_vec = Vec::<u8>::new();
			for rating in existing_customer_rating.ratings_vec {
				temp_rating_vec.push(rating);
			}
			temp_rating_vec.push(rating_for_customer);

			let curr_customer_ratings = User::new(customer.clone(), UserType::Customer, temp_rating_vec);
			<CustomerRatings<T>>::insert(customer.clone(), curr_customer_ratings.clone());

			let transfer_amount = task.cost;
			T::Currency::remove_lock(LOCKSECRET,&customer);
			T::Currency::remove_lock(LOCKSECRET,&bidder);
			T::Currency::transfer(&customer,&bidder, transfer_amount, ExistenceRequirement::KeepAlive)?;

			// Updating Task Status
			task.status = Status::Completed;
			TaskStorage::<T>::insert(&task_id,task.clone());

			Self::deposit_event(Event::AmountTransfered(customer.clone(),bidder.clone(),transfer_amount.clone()));

			Self::deposit_event(Event::TaskClosed(task_id.clone()));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn get_account_balance(origin:OriginFor<T>)-> DispatchResult{

			// To check balance of an account
			// 1. Returns the account balance
			// 2. Store the balances in a map
			// 3. if the balance of the accountId already exists in the map, then get that value and return it
			// 4. else make a call using the Currency::total_balance function to get the account balance and
			//  store it in the map and also return the value

			let result;
			let current_balance;
			let sender = ensure_signed(origin)?;

			result = <AccountBalances<T>>::contains_key(&sender);
			if !result {
				current_balance = T::Currency::total_balance(&sender);
				<AccountBalances<T>>::insert(&sender, &current_balance);
			} else {
				current_balance = Self::get_account_balances(&sender);
			}

			Self::deposit_event(Event::AccBalance(sender, current_balance));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn transfer_money(origin:OriginFor<T>, to: T::AccountId, transfer_amount:BalanceOf<T>)-> DispatchResult{

			// 1. Transfer Money
			// 2. Check if the sender has enough funds to send money else throw Error
			// 2. Store the details in a struct
			// 3. Store the details in a vec

			let sender = ensure_signed(origin)?;
			let sender_account_balance = T::Currency::total_balance(&sender);

			let is_valid_to_transfer = sender_account_balance.clone() < transfer_amount.clone();
			ensure!(!is_valid_to_transfer, <Error<T>>::NotEnoughBalance);

			let to_account_balance = T::Currency::total_balance(&to);

			let result = T::Currency::transfer(&sender, &to, transfer_amount, ExistenceRequirement::KeepAlive)?;

			let updated_sender_account_balance = T::Currency::total_balance(&sender);
			let updated_to_account_balance = T::Currency::total_balance(&to);
			Self::deposit_event(Event::CountIncreased(Self::get_count()));

			// Initializing a vec and storing the details is a Vec
			let mut details: Vec<TransferDetails<T::AccountId, BalanceOf<T>>> = Vec::new();

			let transfer_details = TransferDetails {
				transfer_from: sender.clone(),
				from_before: sender_account_balance.clone(),
				from_after: updated_sender_account_balance.clone(),
				transfer_to: to.clone(),
				to_before: to_account_balance.clone(),
				to_after: updated_to_account_balance.clone(),
			};
			details.push(transfer_details);
			<Transfers<T>>::put(details);

			Self::deposit_event(Event::TransferMoney(sender.clone(), sender_account_balance.clone(), updated_sender_account_balance.clone(), to.clone(), to_account_balance.clone(), updated_to_account_balance.clone()));
			Ok(())

		}




		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// pub fn do_something(origin: Or;iginFor<T>, something: u32) -> DispatchResult {
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
// #[weight = 10_000]
// pub fn transfer_money(origin, to: T::AccountId, transfer_amount: BalanceOf<T>) -> dispatch::DispatchResult {
// 	// 1. Transfer Money
// 	// 2. Check if the sender has enough funds to send money else throw Error
// 	// 2. Store the details in a struct
// 	// 3. Store the details in a vec
// 	let sender = ensure_signed(origin)?;
// 	let sender_account_balance = T::Currency::total_balance(&sender);

// 	// let is_valid_to_transfer = sender_account_balance.clone() < transfer_amount.clone();
// 	// debug::info!("is_valid_to_transfer {:?}", is_valid_to_transfer);
// 	// ensure!(!is_valid_to_transfer, Error::<T>::NotEnoughBalance);

// 	let to_account_balance = T::Currency::total_balance(&to);

// 	let result = T::Currency::transfer(&sender, &to, transfer_amount, ExistenceRequirement::KeepAlive)?;
// 	debug::info!("Transfer Result {:?}", result);

// 	let updated_sender_account_balance = T::Currency::total_balance(&sender);
// 	let updated_to_account_balance = T::Currency::total_balance(&to);
// 	Self::deposit_event(RawEvent::CountIncreased(Self::get_count()));

// 	// Initializing a vec and storing the details is a Vec
// 	let mut details: Vec<TransferDetails<T::AccountId, BalanceOf<T>>> = Vec::new();
// 	let transfer_details = TransferDetails {
// 		transfer_from: sender.clone(),
// 		from_before: sender_account_balance.clone(),
// 		from_after: updated_sender_account_balance.clone(),
// 		transfer_to: to.clone(),
// 		to_before: to_account_balance.clone(),
// 		to_after: updated_to_account_balance.clone(),
// 	};
// 	details.push(transfer_details);
// 	Transfers::<T>::put(details);
// 	debug::info!("Transfer Details Sender: {:#?}", &sender);
// 	debug::info!("Transfer Details Before Balance{:#?}", sender_account_balance.clone());
// 	debug::info!("Transfer Details After Balance: {:#?}", updated_sender_account_balance.clone());
// 	debug::info!("Transfer Details To Account: {:#?}", &to);
// 	debug::info!("Transfer Details Before Balance {:#?}", to_account_balance.clone());
// 	debug::info!("Transfer Details After Balance: {:#?}", updated_to_account_balance.clone());
// 	let transfers_in_store = Self::get_transfers();
// 	debug::info!("Transfer Details From Vec: {:#?}", &transfers_in_store[0]);
// 	Self::deposit_event(RawEvent::TransferMoney(sender.clone(), sender_account_balance.clone(), updated_sender_account_balance.clone(), to.clone(), to_account_balance.clone(), updated_to_account_balance.clone()));
// 	Ok(())
// }

// }






