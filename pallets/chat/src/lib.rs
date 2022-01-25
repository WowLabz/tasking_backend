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

	
	#[derive(Encode, Decode, Default, PartialEq, Eq, Debug, Clone, TypeInfo)]
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

	impl Default for Status {
		fn default() -> Self {
			Status::Active
		}
	}

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

	// #[pallet::storage]
	// #[pallet::getter(fn get_message)]
    // /// For storing the message details
	// pub(super) type MessageStorage<T: Config> = StorageValue<_, Vec<Message<T::AccountId>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_message)]
	pub(super) type MsgStorage<T: Config> = StorageMap<_, Blake2_128Concat, u128, Message<T::AccountId>, ValueQuery>;

	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		MessageCreated(u128,T::AccountId, T::AccountId),
		MessageReplied(u128,T::AccountId, T::AccountId)	
	 }

	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Receiver should be a valid recipient and not the same as sender
		ReceiverNotValid,
		/// To make sure message exists
		MessageDoesNotExist,
		/// To make sure message is active
		MessageIsClosed,
		/// To make sure only the receiver replies to the message
		UnauthorisedToReply
		
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(100)]
		pub fn write_message(origin: OriginFor<T>,receiver: T::AccountId, message: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let message_count =  Self::get_message_count();

			// ensure sender id is not same as receiver id
			ensure!(sender != receiver,<Error<T>>::ReceiverNotValid);

			let msg = Message {
				message_id: message_count,
				sender_id: sender.clone(),
				receiver_id: receiver.clone(),
				message: message,
				reply: None,
				status: Status::Active
			};

			// // If mode = true, storagevalue element is called else storagemap
			// if mode{
			// 	let mut msg_details: Vec<Message<T::AccountId>> = Vec::new();
			// 	msg_details.push(msg.new());
				
			// 	<MessageStorage<T>>::put(msg_details);
				
			// }
			// else{
				
			<MsgStorage<T>>::insert(&message_count,msg.new());
			
			<MessageCount<T>>::put(message_count + 1);

			Self::deposit_event(Event::MessageCreated(message_count,sender,receiver));

			Ok(())
		}

		#[pallet::weight(100)]
		pub fn reply_message(origin: OriginFor<T>, message_id: u128, reply: Vec<u8>) -> DispatchResult {
			let receiver = ensure_signed(origin)?;

			//ensure message id exists
			ensure! (<MsgStorage<T>>::contains_key(&message_id),<Error<T>>::MessageDoesNotExist);

			let mut msg = Self::get_message(message_id.clone());

			// ensure the recipient only replies
			ensure! (receiver == msg.receiver_id,<Error<T>>::UnauthorisedToReply);

			// ensure check to make sure message is active
			ensure! (msg.status == Status::Active,<Error<T>>::MessageIsClosed);
			
			msg.reply = Some(reply);
			msg.status = Status::Closed;
			
			let original_sender = msg.sender_id.clone();

			// let mut reply_details: Vec<&Message<T::AccountId>> = Vec::new();
			// reply_details.push(&msg);
			// <MessageStorage<T>>::put(reply_details);

			<MsgStorage<T>>::insert(&message_id,msg);
	
			Self::deposit_event(Event::MessageReplied( message_id, receiver, original_sender));
			
			Ok(())

		}

	}
}






