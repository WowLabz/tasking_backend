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
	#[pallet::getter(fn get_task_count)]
    /// For storing the number of tasks
	pub type TaskCount<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn task)]
    /// For storing the task details
	pub(super) type TaskStorage<T: Config> = StorageMap<
        _, 
        Blake2_128Concat, 
        u128, 
        TaskDetails<T::AccountId, BalanceOf<T>>, 
        ValueQuery
    >;

	#[pallet::storage]
	#[pallet::getter(fn get_worker_ratings)]
    /// For storing the worker ratings
	pub(super) type WorkerRatings<T: Config> = StorageMap<
        _, 
        Blake2_128Concat, 
        T::AccountId, 
        User<T::AccountId>, 
        ValueQuery
    >;

	#[pallet::storage]
	#[pallet::getter(fn get_customer_ratings)]
    /// For storing customer ratings
	pub(super) type CustomerRatings<T: Config> = StorageMap<
        _, 
        Blake2_128Concat, 
        T::AccountId, 
        User<T::AccountId>, 
        ValueQuery
    >;

	#[pallet::storage]
	#[pallet::getter(fn get_account_balances)]
    /// For storing account balances
	pub(super) type AccountBalances<T: Config> = StorageMap<
        _, 
        Blake2_128Concat, 
        T::AccountId, 
        BalanceOf<T>, 
        ValueQuery
    >;

	#[pallet::storage]
	#[pallet::getter(fn get_count)]
    /// For fetching the count of transfers
	pub(super) type Count<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_transfers)]
    /// For fetching the transfer details
	pub(super) type Transfers<T: Config> = StorageValue<
        _, 
        Vec<TransferDetails<T::AccountId, BalanceOf<T>>>, 
        ValueQuery
    >;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		TaskCreated(T::AccountId, Vec<u8>, u128, u64, BalanceOf<T>, Vec<u8>),
		TaskIsBid(T::AccountId, Vec<u8>, u128),
		TaskCompleted(T::AccountId, u128, T::AccountId),
		/// [TaskID]
		TaskApproved(u128),
		AmountTransfered(T::AccountId, T::AccountId, BalanceOf<T>),
		/// [TaskID]
		TaskClosed(u128),
		AccBalance(T::AccountId, BalanceOf<T>),
		CountIncreased(u128),
		TransferMoney(T::AccountId, BalanceOf<T>, BalanceOf<T>, T::AccountId, BalanceOf<T>, BalanceOf<T>)
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

        /* Description:
         * Extrinsic for creating tasks on the blockchain. This is called by the ..
         * publisher who wants to post tasks on the chain that can be ..
         * put up for bidding.
        */
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
            // User authentication
			let who = ensure_signed(origin)?;
            // Fetching the latest task count
			let current_task_count = Self::get_task_count();
            // Locking the amount from the publisher for the task
			T::Currency::set_lock(
				LOCKSECRET, 
				&who, 
				task_cost.clone(), 
				WithdrawReasons::TRANSACTION_PAYMENT
			);
            // Details related to task created for storage
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
            // Inserting the new task details to storage
			<TaskStorage<T>>::insert(current_task_count.clone(), task_details);
            // Notifying the user about the transaction event
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
            // Incrementing the task count in storage
			<TaskCount<T>>::put(current_task_count + 1);

			Ok(())
		}

        /* Description:
         * Extrinsic for bidding for tasks on the blockchain. This is called by ..
         * the worker who wants to take up tasks on the chain that ..
         * can be completed in the given span of time.
        */
		#[pallet::weight(10_000)]
		pub fn bid_for_task(
			origin:OriginFor<T>, 
			task_id:u128, 
			worker_name:Vec<u8>
		) -> DispatchResult {
            // User authentication
			let bidder = ensure_signed(origin)?;
            // Does task exists?
			ensure!(
				<TaskStorage<T>>::contains_key(&task_id), 
				<Error<T>>::TaskDoesNotExist
            };
            // Getting task details
			let mut task = Self::task(task_id.clone());
            // Accessing task cost
			let task_cost = task.cost.clone();
            // Is there balance to bid?
			ensure!(
				T::Currency::free_balance(&bidder.clone()) > task_cost, 
				<Error<T>>::NotEnoughBalanceToBid
			);
            // Accessing publisher
			let publisher = task.publisher.clone();
            // Is publisher the bidder?
			ensure!(
				publisher != bidder.clone(), 
				<Error<T>>::UnauthorisedToBid
			);
            // Accessing task status
			let status = task.status.clone();
            // Is task open?
			ensure!(status == Status::Open, <Error<T>>::TaskIsNotOpen);
            // Updating worker id
			task.worker_id = Some(bidder.clone());
            // Updating worker name
			task.worker_name = Some(worker_name.clone());
            // Updating status of task
			task.status = Status::InProgress;
            // Inserting updated task in storage
			<TaskStorage<T>>::insert(&task_id, task);
            // Locking bid amount
			T::Currency::set_lock(
				LOCKSECRET, 
				&bidder, 
				task_cost.clone(), 
				WithdrawReasons::TRANSACTION_PAYMENT
			);		
            // Notifying the user
			Self::deposit_event(
				Event::TaskIsBid(bidder.clone(), 
				worker_name.clone(), 
				task_id.clone())
			);

			Ok(())
		}

        /* Description:
         * Extrinsic for marking tasks as complete on the blockchain. ..
         * This is called by the respective worker who wants to signify that the ..
         * alloted task has been completed and is put up for approval 
         * for the publisher.
        */
		#[pallet::weight(10_000)]
		pub fn task_completed(
            origin: OriginFor<T>, 
            task_id: u128, 
            worker_attachments: Option<Vec<Vec<u8>>> // TODO: Mandatory
        ) -> DispatchResult {
            // User authentication
			let bidder = ensure_signed(origin)?;
            // Does task exist?
			ensure!(
                <TaskStorage<T>>::contains_key(task_id.clone()), 
                <Error<T>>::TaskDoesNotExist
            );
            // Get task details from storage
			let mut task = Self::task(task_id.clone());
            // Accessing task status
			let status = task.status;
            // Is task in progress?
			ensure!(
                status == Status::InProgress, 
                <Error<T>>::TaskIsNotInProgress
            );
            // Accessing publisher
			let publisher = task.publisher.clone();
            // Checking if worker is set or not
			let worker = task.worker_id.clone().ok_or(<Error<T>>::WorkerNotSet)?;
			// Is worker the biider?
            ensure!(worker == bidder.clone(), <Error<T>>::UnauthorisedToComplete);
            // Updating the status
			task.status = Status::PendingApproval;
			// Accessing the task attachments
            let existing_attachments = task.attachments.clone();
            // Creating vector for holding old and new attachents
			let mut updated_attachments: Vec<Vec<u8>> = Vec::new();
			// Update only if old attachments exist 
            if let Some(attachments) =  existing_attachments {
                updated_attachments.extend(attachments.clone());
            }
			// update only if new attachments exist 
            if let Some(work_attachments) =  worker_attachments {
                updated_attachments.extend(work_attachments.clone());
            }
            // Updating the attachments for storage
			task.attachments = Some(updated_attachments);
            // Inserting the updated task details
			<TaskStorage<T>>::insert(&task_id, task.clone());
            // Notify user
			Self::deposit_event(
                Event::TaskCompleted(
                    worker.clone(), 
                    task_id.clone(), 
                    publisher.clone()
                )
            );

			Ok(())
		}

        /* Description:
         * Extrinsic for approving tasks that are completed by the worker on the blockchain. ..
         * This is called by the publisher who put up the task in the first place for signifying .. 
         * that the publisher approves the final work of the worker. Also rating to the worker ..
         * is given while approving the task.
        */
		#[pallet::weight(10_000)]
		pub fn approve_task(
            origin: OriginFor<T>, 
            task_id: u128, 
            rating_for_the_worker: u8
        ) -> DispatchResult {
	        // User authentication
			let publisher = ensure_signed(origin)?;
            // Does task exist?
			ensure!(
                <TaskStorage<T>>::contains_key(task_id.clone()),
                <Error<T>>::TaskDoesNotExist
            );
            // Getting task details from storage
			let mut task = Self::task(task_id.clone());
            // Accessing task status
			let status = task.status;
            // Is approval pending?
			ensure!(
                status == Status::PendingApproval, 
                <Error<T>>::TaskIsNotPendingApproval
            );
            // Accessing publisher
			let approver = task.publisher.clone();
            // Is publisher the approver?
			ensure!(
                publisher == approver.clone(), 
                <Error<T>>::UnauthorisedToApprove
            );
            // Checking if the worker is set or not
			let bidder = task.worker_id.clone().ok_or(<Error<T>>::WorkerNotSet)?;
			// Inserting Worker Rating to RatingMap
			let existing_bidder_ratings: User<T::AccountId> = Self::get_worker_ratings(&bidder);
            // Creating temp rating vector
			let mut temp_rating_vec = Vec::<u8>::new();
            // Looping through all the existing worker ratings
			for rating in existing_bidder_ratings.ratings_vec {
				temp_rating_vec.push(rating);
			}
            // Updating the temp rating vector with new rating
			temp_rating_vec.push(rating_for_the_worker);
            // Creating a new user instance for updating worker details
			let curr_bidder_ratings = User::new(bidder.clone(), UserType::Worker, temp_rating_vec);
            // Inserting into worker rating storage
			<WorkerRatings<T>>::insert(bidder.clone(), curr_bidder_ratings.clone());
			// Updating task status
			task.status = Status::PendingRatings;
            // Inserting updated task into storage
			<TaskStorage<T>>::insert(&task_id, task.clone());
            // Notify user
			Self::deposit_event(Event::TaskApproved(task_id.clone()));

			Ok(())
		}

        /* Description:
         * Extrinsic for providing rating to publisher on the blockchain. ..
         * This is called by the worker who will judge the publisher based ..
         * on a number of factors.
        */
		#[pallet::weight(10_000)]
		pub fn provide_customer_rating(
            origin: OriginFor<T>, 
            task_id: u128, 
            rating_for_customer: u8
        ) -> DispatchResult {
            // User authentication
			let bidder = ensure_signed(origin)?;
            // Getting task details from storage
			let mut task = Self::task(task_id.clone());
            // Accessing status 
			let status = task.status;
            // Is rating pending from worker to publisher?
			ensure!(
                status == Status::PendingRatings, 
                <Error<T>>::TaskIsNotPendingRating
            );
            // Get worker id
            let worker = task.worker_id.clone().ok_or(<Error<T>>::WorkerNotSet)?;
			// Is worker the bidder?
            ensure!(
                worker == bidder.clone(), 
                <Error<T>>::UnauthorisedToProvideCustomerRating
            );
            // Accessing reference of the publisher
			let customer = &task.publisher;
            // Get existing customer ratings
			let existing_customer_rating: User<T::AccountId> = Self::get_customer_ratings(&customer);
            // Creating a temp rating vector
			let mut temp_rating_vec = Vec::<u8>::new();
            // Looping over all the existing customer ratings
			for rating in existing_customer_rating.ratings_vec {
				temp_rating_vec.push(rating);
			}
            // Updating temp rating vector with new rating
			temp_rating_vec.push(rating_for_customer);
            // Creating new user instance with new rating
			let curr_customer_ratings = User::new(customer.clone(), UserType::Customer, temp_rating_vec);
            // Inserting new user instance in customer rating storage
			<CustomerRatings<T>>::insert(customer.clone(), curr_customer_ratings.clone());
            // Accessing task cost
			let transfer_amount = task.cost;
            // Removing the lock on customer's funds
			T::Currency::remove_lock(LOCKSECRET,&customer);
            // Removing the lock on the bidder's funds
			T::Currency::remove_lock(LOCKSECRET,&bidder);
            // Transfering amount from customer to bidder
			T::Currency::transfer(&customer, &bidder, transfer_amount, ExistenceRequirement::KeepAlive)?;
			// Updating task status
			task.status = Status::Completed;
            // Inserting updated task details
			TaskStorage::<T>::insert(&task_id, task.clone());
            // Notify user about the transfered amount
			Self::deposit_event(
                Event::AmountTransfered(
                    customer.clone(),
                    bidder.clone(),
                    transfer_amount.clone()
                )
            );
            // Notify the user about the task being closed
			Self::deposit_event(Event::TaskClosed(task_id.clone()));

			Ok(())
		}

		/* Description:
         * Extrinsic for transfering funds from one account to another ..
         * on the blockchain. This is called by the worker / publisher.
        */
		#[pallet::weight(10_000)]
		pub fn transfer_money(
            origin: OriginFor<T>, 
            to: T::AccountId, 
            transfer_amount: BalanceOf<T>
        ) -> DispatchResult {
            // User authentication
			let sender = ensure_signed(origin)?;
            // Get total balance of sender
			let sender_account_balance = T::Currency::total_balance(&sender);
            // Verify if sender's balance is greater than transfer amount
			let is_valid_to_transfer = sender_account_balance.clone() < transfer_amount.clone();
            // Is the transfer valid based on the sender's balance
			ensure!(!is_valid_to_transfer, <Error<T>>::NotEnoughBalance);
            // Get account balance of receiver
			let to_account_balance = T::Currency::total_balance(&to);
            // Making the transfer
			T::Currency::transfer(&sender, &to, transfer_amount, ExistenceRequirement::KeepAlive)?;
            // Get updated balance of sender
			let updated_sender_account_balance = T::Currency::total_balance(&sender);
            // Get updated balance of receiver
			let updated_to_account_balance = T::Currency::total_balance(&to);
            // Notify user about the increased transfer count
			Self::deposit_event(Event::CountIncreased(Self::get_count()));
			// Initializing a vec and storing the details is a Vec
			let mut details: Vec<TransferDetails<T::AccountId, BalanceOf<T>>> = Vec::new();
            // Preparing the transfer details structure
			let transfer_details = TransferDetails {
				transfer_from: sender.clone(),
				from_before: sender_account_balance.clone(),
				from_after: updated_sender_account_balance.clone(),
				transfer_to: to.clone(),
				to_before: to_account_balance.clone(),
				to_after: updated_to_account_balance.clone(),
			};
            // Updating the vector with transfer details
			details.push(transfer_details);
            // Updating storage with new transfer details
			<Transfers<T>>::put(details);
            // Notify user about the transfer details
			Self::deposit_event(
                Event::TransferMoney(
                    sender.clone(), 
                    sender_account_balance.clone(), 
                    updated_sender_account_balance.clone(), 
                    to.clone(), 
                    to_account_balance.clone(), 
                    updated_to_account_balance.clone()
                )
            );

			Ok(())
		}
	}
}







