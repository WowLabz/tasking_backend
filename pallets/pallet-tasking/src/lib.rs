#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{
	debug, decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
	traits::{
		Currency, ExistenceRequirement, Get, LockIdentifier, LockableCurrency, ReservableCurrency,
		WithdrawReasons,
	},
};
use frame_system::ensure_signed;
use pallet_assets;
use pallet_balances;
use pallet_staking;
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub const LOCKSECRET: LockIdentifier = *b"mylockab";

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Encode, Decode, Default, Debug, PartialEq, Clone, Eq)]
pub struct TaskDetails<AccountId, Balance> {
	task_id: u128,
	publisher: AccountId,
	worker_id: Option<AccountId>,
	task_deadline: u64,
	cost: Balance,
	status: Status,
	task_description: Vec<u8>,
}

#[derive(Encode, Decode, PartialEq, Eq, Debug, Clone)]
pub enum Status {
	Open,
	InProgress,
	PendingApproval,
	Completed,
}

impl Default for Status {
	fn default() -> Self {
		Status::Open
	}
}

#[derive(Encode, Decode, Default, Debug, PartialEq, Clone, Eq)]
pub struct TransferDetails<AccountId, Balance> {
	transfer_from: AccountId,
	from_before: Balance,
	from_after: Balance,
	transfer_to: AccountId,
	to_before: Balance,
	to_after: Balance,
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type Currency: LockableCurrency<Self::AccountId>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
// A unique name is used to ensure that the pallet's storage items are isolated.
// This name may be updated, but each pallet in the runtime must use a unique name.
// ---------------------------------vvvvvvvvvvvvvv
// Learn more about declaring storage items:
// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
decl_storage! {
	trait Store for Module<T: Config> as TaskStore {

			TaskStorage get(fn task):
			map hasher(blake2_128_concat) u128 => TaskDetails<T::AccountId, BalanceOf<T>>;
			TaskCount get(fn get_task_count): u128 = 0;
			AccountBalances get(fn get_account_balances):
			map hasher(blake2_128_concat) T::AccountId => BalanceOf<T>;
			Count get(fn get_count): u128 = 0;
			Transfers get(fn get_transfers): Vec<TransferDetails<T::AccountId, BalanceOf<T>>>;
			StakerStorage get(fn staker_list):
			map hasher(blake2_128_concat) u128 => Vec<T::AccountId>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
		Balance = BalanceOf<T>,
	{
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, AccountId),
		TaskCreated(AccountId, u128, u64, Balance, Vec<u8>),
		AccBalance(AccountId, Balance),
		CountIncreased(u128),
		TransferMoney(AccountId, Balance, Balance, AccountId, Balance, Balance),
		StakerAdded(AccountId),
		TaskIsBidded(AccountId, u128),
		AmountTransfered(AccountId, AccountId, Balance),
		TaskCompleted(AccountId, u128, AccountId),
		TaskApproved(u128),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		OriginNotSigned,
		NotEnoughBalance,
		TaskDoesNotExist,
		AlreadyMember,
		TaskIsNotApproved,
		YouNeverBiddedForThisTask,
		TaskIsNotOpen,
		TaskIsNotInProgress,
		TaskIsNotPendingApproval,
		UnauthorisedToBid,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.


		/// An example dispatchable that may throw a custom error
		#[weight = 10_000]
		pub fn create_task(origin, task_duration: u64, task_cost: BalanceOf<T>, task_des: Vec<u8>) {
		 let sender = ensure_signed(origin)?;
		 let current_count = Self::get_task_count();

		 let result_from_locking = T::Currency::set_lock(LOCKSECRET, &sender, task_cost.clone(), WithdrawReasons::TRANSACTION_PAYMENT);
		 debug::info!("result_from_locking : {:#?}", result_from_locking);

		 let temp= TaskDetails {
			  task_id: current_count.clone(),
			  publisher:sender.clone(),
			  worker_id: None,
			  task_deadline: task_duration.clone(),
			  cost:task_cost.clone(),
			  status: Default::default(),
			  task_description: task_des.clone(),
		  };
		  TaskStorage::<T>::insert(current_count.clone(), temp);
		  Self::deposit_event(RawEvent::TaskCreated(sender, current_count.clone(), task_duration.clone(), task_cost.clone(), task_des.clone()));
		  TaskCount::put(current_count + 1);
		}

		#[weight = 10_000]
		pub fn bid_for_task(origin, task_id: u128) {
			let bidder = ensure_signed(origin)?;
			ensure!(TaskStorage::<T>::contains_key(&task_id), Error::<T>::TaskDoesNotExist);
			let mut task = TaskStorage::<T>::get(task_id.clone());

			let publisher = task.publisher.clone();
			ensure!(publisher != bidder.clone(), Error::<T>::UnauthorisedToBid);

			let status = task.status.clone();
			ensure!(status == Status::Open, Error::<T>::TaskIsNotOpen);

			let task_cost= task.cost.clone();
			task.worker_id = Some(bidder.clone());
			task.status= Status::InProgress;

			TaskStorage::<T>::insert(&task_id, task);
			T::Currency::set_lock(LOCKSECRET, &bidder, task_cost.clone(), WithdrawReasons::TRANSACTION_PAYMENT);
			Self::deposit_event(RawEvent::TaskIsBidded(bidder.clone(), task_id.clone()));

			let task_details_by_helper = Self::get_task(task_id.clone());
			debug::info!("task_details_by_helper : {:#?}", task_details_by_helper);
		}

		#[weight = 10_000]
		pub fn approve_task(origin,task_id:u128) {
			let publisher=ensure_signed(origin)?;
			ensure!(Self::task_exist(task_id.clone()), Error::<T>::TaskDoesNotExist);

			let mut task_struct = TaskStorage::<T>::get(&task_id);
			let status = task_struct.status;
			ensure!(status == Status::PendingApproval, Error::<T>::TaskIsNotPendingApproval);
			let bidder=task_struct.worker_id.clone().unwrap();

			task_struct.status = Status::Completed;
			TaskStorage::<T>::insert(&task_id,task_struct.clone());

			let transfer_amount = task_struct.cost;
			T::Currency::remove_lock(LOCKSECRET,&publisher);
			T::Currency::remove_lock(LOCKSECRET,&bidder);
			T::Currency::transfer(&publisher,&bidder, transfer_amount, ExistenceRequirement::KeepAlive)?;

			Self::deposit_event(RawEvent::AmountTransfered(publisher.clone(),bidder.clone(),transfer_amount.clone()));
			Self::deposit_event(RawEvent::TaskApproved(task_id.clone()));
		}

		#[weight = 10_000]
		pub fn task_completed(origin, task_id: u128) {
			 let bidder=ensure_signed(origin)?;
			 ensure!(Self::task_exist(task_id.clone()), Error::<T>::TaskDoesNotExist);
			 let mut task_struct=TaskStorage::<T>::get(&task_id);

			 let status=task_struct.status;
			 ensure!(status == Status::InProgress,Error::<T>::TaskIsNotInProgress);

			 let publisher = task_struct.publisher.clone();
			 ensure!(task_struct.worker_id.clone().unwrap()==bidder, Error::<T>::YouNeverBiddedForThisTask);

			 task_struct.status = Status::PendingApproval;

			 TaskStorage::<T>::insert(&task_id,task_struct.clone());
			 Self::deposit_event(RawEvent::TaskCompleted(publisher.clone(), task_id.clone(),bidder.clone()));
		}

		#[weight = 10_000]
		pub fn function_for_tasks_and_accounts_using_vec_staking(origin, task_id: u128) -> dispatch::DispatchResult {
			let staker = ensure_signed(origin)?;

			ensure!(TaskStorage::<T>::contains_key(&task_id), Error::<T>::TaskDoesNotExist);
			let mut temp_staker_list = Self::staker_list(&task_id);
			debug::info!("Calling function using get method {:?}", &temp_staker_list);

			match temp_staker_list.binary_search(&staker) {
				// If the search succeeds, the caller is already a member, so just return
				Ok(_) => Err(Error::<T>::AlreadyMember.into()),
				// If the search fails, the caller is not a member and we learned the index where
				// they should be inserted
				Err(index) => {
					temp_staker_list.insert(index, staker.clone());
					StakerStorage::<T>::insert(task_id.clone(), temp_staker_list);
					Self::deposit_event(RawEvent::StakerAdded(staker.clone()));
					Ok(())
				}
			}
		}

		#[weight = 10_000]
		pub fn get_account_balance(origin) -> dispatch::DispatchResult {

			// To check balance of an account
			// 1. Returns the account balance
			// 2. Store the balances in a map
			// 3. if the balance of the accountId already exists in the map, then get that value and return it
			// 4. else make a call using the Currency::total_balance function to get the account balance and
			//  store it in the map and also return the value

			let result;
			let current_balance;
			let sender = ensure_signed(origin)?;

			result = AccountBalances::<T>::contains_key(&sender);
			if !result {
				current_balance = T::Currency::total_balance(&sender);
				AccountBalances::<T>::insert(&sender, &current_balance);
			} else {
				current_balance = AccountBalances::<T>::get(&sender);
			}

			debug::info!("Account Balance: {:?}", current_balance);
			Self::deposit_event(RawEvent::AccBalance(sender, current_balance));
			Ok(())
		}

		#[weight = 10_000]
		pub fn get_data_from_store(origin, task_id: u128) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			let acc_balance = T::Currency::total_balance(&sender);
			// let acc_balance = AccountBalances::<T>::get(&sender);
			debug::info!("get_data_from_store balance: {:?}", acc_balance);

			let task_details = TaskStorage::<T>::get(&task_id);
			debug::info!("get_data_from_store taskstore: {:#?}", task_details);

			let task_details_by_helper = Self::get_task(task_id.clone());
			debug::info!("task_details_by_helper : {:#?}", task_details_by_helper);

			Ok(())
		}

		#[weight = 10_000]
		pub fn increase_counter(origin) {
			ensure_signed(origin)?;
			let current_count = Self::get_count();
			Count::put(current_count + Self::get_one());
			Self::deposit_event(RawEvent::CountIncreased(Self::get_count()));
		}

		#[weight = 10_000]
		pub fn transfer_money(origin, to: T::AccountId, transfer_amount: BalanceOf<T>) -> dispatch::DispatchResult {
			// 1. Transfer Money
			// 2. Check if the sender has enough funds to send money else throw Error
			// 2. Store the details in a struct
			// 3. Store the details in a vec
			let sender = ensure_signed(origin)?;
			let sender_account_balance = T::Currency::total_balance(&sender);

			// let is_valid_to_transfer = sender_account_balance.clone() < transfer_amount.clone();
			// debug::info!("is_valid_to_transfer {:?}", is_valid_to_transfer);
			// ensure!(!is_valid_to_transfer, Error::<T>::NotEnoughBalance);

			let to_account_balance = T::Currency::total_balance(&to);

			let result = T::Currency::transfer(&sender, &to, transfer_amount, ExistenceRequirement::KeepAlive)?;
			debug::info!("Transfer Result {:?}", result);

			let updated_sender_account_balance = T::Currency::total_balance(&sender);
			let updated_to_account_balance = T::Currency::total_balance(&to);
			Self::deposit_event(RawEvent::CountIncreased(Self::get_count()));

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
			Transfers::<T>::put(details);
			debug::info!("Transfer Details Sender: {:#?}", &sender);
			debug::info!("Transfer Details Before Balance{:#?}", sender_account_balance.clone());
			debug::info!("Transfer Details After Balance: {:#?}", updated_sender_account_balance.clone());
			debug::info!("Transfer Details To Account: {:#?}", &to);
			debug::info!("Transfer Details Before Balance {:#?}", to_account_balance.clone());
			debug::info!("Transfer Details After Balance: {:#?}", updated_to_account_balance.clone());
			let transfers_in_store = Self::get_transfers();
			debug::info!("Transfer Details From Vec: {:#?}", &transfers_in_store[0]);
			Self::deposit_event(RawEvent::TransferMoney(sender.clone(), sender_account_balance.clone(), updated_sender_account_balance.clone(), to.clone(), to_account_balance.clone(), updated_to_account_balance.clone()));
			Ok(())
		}

	}
}

impl<T: Config> Module<T> {
	// Helper functions
	pub fn get_one() -> u128 {
		1
	}

	pub fn task_exist(task_id: u128) -> bool {
		TaskStorage::<T>::contains_key(&task_id)
	}

	pub fn get_task(task_id: u128) -> TaskDetails<T::AccountId, BalanceOf<T>> {
		TaskStorage::<T>::get(&task_id)
	}

	pub fn transfer(sender: T::AccountId, to: T::AccountId, amount_to_transfer: BalanceOf<T>) {
		T::Currency::transfer(
			&sender,
			&to,
			amount_to_transfer,
			ExistenceRequirement::KeepAlive,
		)
		.unwrap();
	}
}
