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

	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	// NOTE: Need to refactor, duplicate code for testing purposes
	
	type AccountId<T> = <T as frame_system::Config>::AccountId;
	type Balance<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	
	#[derive(Encode, Decode, PartialEq, Eq, Debug, Clone, TypeInfo)]
	pub struct Message<AccountId> {
		message_id: u128,
		sender_id: AccountId,
		receiver_id: AccountId,
		message: Vec<u8>,
		reply: Option<Vec<u8>>,
		status: Status

	}

	impl<AccountId> Message<AccountId>{
		fn new(self) -> Self {
			Self {
				message_id: self.message_id,
				sender_id: self.sender_id,
				receiver_id: self.receiver_id,
				message: self.message,
				reply: self.reply,
				status: self.status
			}
		}
	}

	
	#[derive(Encode, Decode, PartialEq, Eq, Debug, Clone, TypeInfo)]
	pub enum Status {
		Active,
		Closed
	}

	
	// #[derive(Encode, Decode, Debug, PartialEq, Clone, Eq, Default, TypeInfo)]
	// pub struct User<AccountId> {
	// 	account_id: AccountId,
	// 	user_type: UserType,
	// 	rating: Option<u8>,
	// 	ratings_vec: Vec<u8>,
	// }

	// impl<AccountId> User<AccountId> {
	// 	pub fn new(account_id: AccountId, user_type: UserType, ratings_vec: Vec<u8>) -> Self {
	// 		let rating = Some(Self::get_list_average(ratings_vec.clone()));
			
	// 		Self {
	// 			account_id,
	// 			user_type,
	// 			rating,
	// 			ratings_vec,
	// 		}
	// 	}

	// 	pub fn get_list_average(list: Vec<u8>) -> u8 {
	// 		let list_len: u8 = list.len() as u8;
	// 		if list_len == 1 {
	// 			return list[0];
	// 		}
	// 		let mut total_sum = 0;
	// 		for item in list.iter() {
	// 			total_sum += item;
	// 		}
	// 		let average = total_sum / list_len;
			
	// 		average
	// 	}
	// }


	/// Pallet configuration 
    #[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: LockableCurrency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_message_count)]
    /// For storing the number of tasks
	pub type MessageCount<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_message)]
    /// For storing the message details
	pub(super) type MessageStorage<T: Config> = StorageValue<_, Vec<Message<T::AccountId>>, ValueQuery>;

	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
	// 	TaskCreated(T::AccountId, Vec<u8>, u128, u64, BalanceOf<T>, Vec<u8>),
	// 	TaskIsBid(T::AccountId, Vec<u8>, u128),
	// 	TaskCompleted(T::AccountId, u128, T::AccountId),
	// 	/// [TaskID]
	// 	TaskApproved(u128),
	// 	AmountTransfered(T::AccountId, T::AccountId, BalanceOf<T>),
	// 	/// [TaskID]
	// 	TaskClosed(u128),
	// 	AccBalance(T::AccountId, BalanceOf<T>),
	// 	CountIncreased(u128),
	// 	TransferMoney(T::AccountId, BalanceOf<T>, BalanceOf<T>, T::AccountId, BalanceOf<T>, BalanceOf<T>)
	 }

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

		#[pallet::weight(100)]
		pub fn write_message(origin: OriginFor<T>,receiver: T::AccountId, message: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let message_count =  Self::get_message_count();

			let msg = Message {
				message_id: message_count,
				sender_id: sender,
				receiver_id: receiver,
				message: message,
				reply: None,
				status: Status::Active
			};

			let mut msg_details: Vec<Message<T::AccountId>> = Vec::new();
			msg_details.push(msg.new());

			<MessageStorage<T>>::put(msg_details);

			<MessageCount<T>>::put(message_count + 1);

			Ok(())
		}

		#[pallet::weight(100)]
		pub fn reply_message(origin: OriginFor<T>, reply: Vec<u8>) -> DispatchResult {
			let receiver = ensure_signed(origin)?;

			// ensure check to make sure there is a message to reply to

			// ensure check that the receiver is only replying 

			let mut msg = &mut Self::get_message()[0];

			msg.reply = Some(reply);
			msg.status = Status::Closed;

			let mut reply_details: Vec<&Message<T::AccountId>> = Vec::new();
			reply_details.push(&msg);

			<MessageStorage<T>>::put(reply_details);
			
			Ok(())

		}

	}
}







