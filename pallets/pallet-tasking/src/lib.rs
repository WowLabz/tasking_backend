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
	use frame_support::PalletId;
	use frame_system::pallet_prelude::*;
	//use pallet_court::*;
	use frame_support::{
		log,
		sp_runtime::traits::{AccountIdConversion, SaturatedConversion},
		traits::{
			tokens::ExistenceRequirement, Currency, LockIdentifier, LockableCurrency,
			WithdrawReasons,
		},
	};

	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};
	use num_traits::float::Float;
	// use pallet_scheduler;
	use codec::{Decode, Encode};
	use sp_std::collections::btree_map::BTreeMap;
	use sp_std::vec::Vec;

	//type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;

	pub const LOCKSECRET: LockIdentifier = *b"mylockab";

	#[derive(Encode, Decode, PartialEq, Eq, Debug, Clone, TypeInfo)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
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
		/// -> Change
		DisputeRaised,
		VotingPeriod,
		JuryDecisionReached,
		CompletedByDefault,
		CaseClosed,
		/// -> Change
		Completed,
	}

	impl Default for Status {
		fn default() -> Self {
			Status::Open
		}
	}

	#[derive(Encode, Decode, PartialEq, Eq, Debug, Clone, TypeInfo)]
	pub enum Reason {
		DisapproveTask,
		UnsatisfiedWorkerRating,
		UnsatisfiedPublisherRating
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
		attachments: Option<Vec<Vec<u8>>>,
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

	#[derive(Encode, Decode, Default, Debug, PartialEq, Clone, Eq, TypeInfo)]
	pub struct JurorDecisionDetails {
		voted_for: Option<UserType>,
		publisher_rating: Option<u8>,
		worker_rating: Option<u8>,
	}

	// Storage elements are deleted after usage
	#[derive(Encode, Decode, Default, Debug, PartialEq, Clone, Eq, TypeInfo)]
	pub struct DisputeTimeframe<BlockNumber> {
		task_id: u128,
		jury_acceptance_period: BlockNumber,
		total_case_period: BlockNumber,
	}

	#[derive(Encode, Decode, Default, Debug, PartialEq, Clone, Eq, TypeInfo)]
	pub struct CourtDispute<AccountId, Balance, BlockNumber> {
		task_details: TaskDetails<AccountId, Balance>,
		potential_jurors: Vec<AccountId>,
		// In vector: 1. Worker / Publisher, 2. Publisher rating, 3. Worker rating
		final_jurors: BTreeMap<AccountId, JurorDecisionDetails>,
		winner: Option<UserType>,
		//status: Status,
		votes_for_worker: Option<u8>,
		votes_for_customer: Option<u8>,
		avg_worker_rating: Option<u8>,
		avg_publisher_rating: Option<u8>,
		jury_acceptance_period: BlockNumber,
		total_case_period: BlockNumber,
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

			Self { account_id, user_type, rating, ratings_vec }
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
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub struct AccountDetails<Balance> {
		pub balance: Balance,
		pub ratings: Vec<u8>,
		pub avg_rating: Option<u8>,
		pub tags: Vec<TaskTypeTags>,
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
		type PalletId: Get<PalletId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// A map that has enumerable entries.
	#[pallet::storage]
	#[pallet::getter(fn accounts)]
	pub type AccountMap<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, AccountDetails<BalanceOf<T>>, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		// pub single_value: BalanceOf<T>,
		pub account_map: Vec<(T::AccountId, AccountDetails<BalanceOf<T>>)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { account_map: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			// <SingleValue<T>>::put(&self.single_value);
			for (a, b) in &self.account_map {
				<AccountMap<T>>::insert(a, b);
			}
		}
	}
	// -----

	#[pallet::storage]
	#[pallet::getter(fn get_task_count)]
	/// For storing the number of tasks
	pub type TaskCount<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn task)]
	/// For storing the task details
	pub(super) type TaskStorage<T: Config> =
		StorageMap<_, Blake2_128Concat, u128, TaskDetails<T::AccountId, BalanceOf<T>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_worker_ratings)]
	/// For storing the worker ratings
	pub(super) type WorkerRatings<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, User<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_customer_ratings)]
	/// For storing customer ratings
	pub(super) type CustomerRatings<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, User<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_account_balances)]
	/// For storing account balances
	pub(super) type AccountBalances<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_count)]
	/// For fetching the count of transfers
	pub(super) type Count<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_transfers)]
	/// For fetching the transfer details
	pub(super) type Transfers<T: Config> =
		StorageValue<_, Vec<TransferDetails<T::AccountId, BalanceOf<T>>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_dispute_timeframes)]
	/// For fetching the dispute timeframes
	pub(super) type Timeframes<T: Config> =
		StorageValue<_, Vec<DisputeTimeframe<BlockNumberOf<T>>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_disupte_details)]
	pub(super) type Courtroom<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u128,
		CourtDispute<T::AccountId, BalanceOf<T>, BlockNumberOf<T>>,
		ValueQuery,
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
		TransferMoney(
			T::AccountId,
			BalanceOf<T>,
			BalanceOf<T>,
			T::AccountId,
			BalanceOf<T>,
			BalanceOf<T>,
		),
		CourtSummoned(u128, Reason, UserType, T::AccountId),
		NewJurorAdded(u128, T::AccountId),
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
		/// To check if the sender has sufficient balance for a transfer
		NotEnoughBalance,
		// To check if an account is qualified to be a juror
		NotPotentialJuror,
		// To ensure final nuber of jurors does not exceed a certain value
		CannotAddMoreJurors,
		/// To ensure if the dispute exists in storage
		DisputeDoesNotExist,
		/// To ensure approval is pending
		TaskInProgress,
		/// To ensure publisher is the one disapproving
		UnauthorisedToDisapprove,
		/// To ensure if the juror hasn't already voted
		JurorHasVoted,
		/// To stop accepting participants for jury after elapsed time
		JurySelectionPeriodElapsed,
		/// To stop jurors to vote before the actual voting period
		JurySelectionInProcess,
		/// To ensure jurors can't vote beyond the voting period
		CaseClosed,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(now: T::BlockNumber) -> Weight {
			let total_weight: Weight = 10;
			Self::settle_dispute(now);
			total_weight
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000)]
		pub fn disapprove_rating(origin: OriginFor<T>, task_id: u128, user_type: UserType) -> DispatchResult {
			let who = ensure_signed(origin)?;

			//ensure task exists and is active
			ensure!(<TaskStorage<T>>::contains_key(&task_id), <Error<T>>::TaskDoesNotExist);

			let task_details = Self::task(task_id.clone());

			let status = task_details.status.clone();

			ensure!(status == Status::PendingRatings, <Error<T>>::TaskIsNotPendingRating);

			Self::register_dispute(task_id.clone(), task_details);

			let reason = match user_type {
				UserType::Customer => Reason::UnsatisfiedPublisherRating,
				UserType::Worker => Reason::UnsatisfiedWorkerRating
			};

			Self::deposit_event(Event::CourtSummoned(task_id, reason, user_type, who));

			Ok(())

		}

		#[pallet::weight(10_000)]
		pub fn disapprove_task(origin: OriginFor<T>, task_id: u128) -> DispatchResult {
			let publisher = ensure_signed(origin)?;

			//ensure task exists and is active
			ensure!(<TaskStorage<T>>::contains_key(&task_id), <Error<T>>::TaskDoesNotExist);

			let task_details = Self::task(task_id.clone());

			let status = task_details.status.clone();

			let customer = task_details.publisher.clone();

			ensure!(status == Status::PendingApproval, <Error<T>>::TaskInProgress);

			//ensure the customer is the one disapproving the task

			ensure!(publisher == customer, <Error<T>>::UnauthorisedToDisapprove);

			Self::register_dispute(task_id.clone(), task_details);

			Self::deposit_event(Event::CourtSummoned(task_id, Reason::DisapproveTask, UserType::Customer , publisher));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn accept_jury_duty(origin: OriginFor<T>, task_id: u128) -> DispatchResult {
			let juror = ensure_signed(origin)?;

			ensure!(<Courtroom<T>>::contains_key(&task_id), <Error<T>>::DisputeDoesNotExist);

			let mut dispute_details = Self::get_disupte_details(task_id.clone());

			// To stop accepting participants for jury after elapsed time
			// -----
			let current_period = <frame_system::Pallet<T>>::block_number();
			let jury_acceptance_period = dispute_details.jury_acceptance_period.clone();
			ensure!(
				current_period < jury_acceptance_period,
				<Error<T>>::JurySelectionPeriodElapsed
			);
			// -----

			ensure!(
				dispute_details.potential_jurors.contains(&juror),
				<Error<T>>::NotPotentialJuror
			);

			// Less than 2 is 2 people as we are ensuring first and then storing
			ensure!(dispute_details.final_jurors.len() < 2, <Error<T>>::CannotAddMoreJurors);

			let juror_details = JurorDecisionDetails {
				voted_for: None,
				publisher_rating: None,
				worker_rating: None,
			};

			dispute_details.final_jurors.insert(juror.clone(), juror_details);

			Self::deposit_event(Event::NewJurorAdded(task_id.clone(), juror));

			<Courtroom<T>>::insert(&task_id, dispute_details.clone());
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn cast_vote(
			origin: OriginFor<T>,
			task_id: u128,
			voted_for: UserType,
			customer_rating: u8,
			worker_rating: u8,
		) -> DispatchResult {
			let juror = ensure_signed(origin)?;

			let mut dispute_details = Self::get_disupte_details(&task_id);

			//TODO -> Move Dispute details to task details 

			// To stop jurors to vote before the actual voting period
			// -----
			let current_period = <frame_system::Pallet<T>>::block_number();
			let jury_acceptance_period = dispute_details.jury_acceptance_period.clone();
			ensure!(current_period > jury_acceptance_period, <Error<T>>::JurySelectionInProcess);
			// -----

			log::info!("{:#?}", dispute_details);

			ensure!(
				dispute_details.task_details.status == Status::VotingPeriod,
				<Error<T>>::CaseClosed
			);

			let mut juror_decision_details =
				dispute_details.final_jurors.get(&juror).cloned().unwrap();

			// Ensuring qualified juror doesn't vote more than once
			ensure!(juror_decision_details.voted_for == None, <Error<T>>::JurorHasVoted);

			log::info!("{:#?}", juror_decision_details);
			// Aim for settling dispute to begin with
			let mut dispute_closed: bool = true;

			juror_decision_details.voted_for = Some(voted_for.clone());
			juror_decision_details.publisher_rating = Some(customer_rating);
			juror_decision_details.worker_rating = Some(worker_rating);

			dispute_details.final_jurors.insert(juror.clone(), juror_decision_details);

			let mut votes_for_customer = dispute_details.votes_for_customer.unwrap_or_else(|| 0);
			let mut votes_for_worker = dispute_details.votes_for_worker.unwrap_or_else(|| 0);

			match voted_for {
				UserType::Customer => {
					votes_for_customer += 1;
					dispute_details.votes_for_customer = Some(votes_for_customer.clone());
				}
				UserType::Worker => {
					votes_for_worker += 1;
					dispute_details.votes_for_worker = Some(votes_for_worker.clone());
				}
			}

			// let total_votes = votes_for_customer + votes_for_worker;
			let final_jurors_count = dispute_details.final_jurors.len() as u8;

			let dispute_details_of_final_jurors: Vec<JurorDecisionDetails> =
				dispute_details.final_jurors.values().cloned().collect();

			// NOTE: Can be refactored?
			for juror_decision in dispute_details_of_final_jurors.clone() {
				log::info!("####### Should be settled");
				if juror_decision.voted_for == None {
					log::info!("####### Should not be settled");
					dispute_closed = false;
					break;
				}
			}

			// For providing ratings and releasing funds after ..
			// all final juror votes are cast.
			if dispute_closed {
				log::info!("####### Have settled");
				let mut total_publisher_rating: u8 = 0;
				let mut total_worker_rating: u8 = 0;
				let escrow_id = Self::escrow_account_id(task_id.clone() as u32);

				for juror_decision in dispute_details_of_final_jurors {
					total_publisher_rating += juror_decision.publisher_rating.unwrap_or_else(|| 0);
					total_worker_rating += juror_decision.worker_rating.unwrap_or_else(|| 0);
				}

				let avg_publisher_rating =
					Self::roundoff(total_publisher_rating, final_jurors_count.clone());
				dispute_details.avg_publisher_rating = Some(avg_publisher_rating);

				let avg_worker_rating =
					Self::roundoff(total_worker_rating, final_jurors_count.clone());
				dispute_details.avg_worker_rating = Some(avg_worker_rating);

				if votes_for_customer > votes_for_worker {
					dispute_details.winner = Some(UserType::Customer);
				} else {
					dispute_details.winner = Some(UserType::Worker);
				}

				let mut winner_account_id: Vec<T::AccountId> = Vec::new();

				match dispute_details.winner.clone() {
					Some(UserType::Customer) => {
						winner_account_id
							.push(dispute_details.task_details.worker_id.clone().unwrap());
						winner_account_id.push(dispute_details.task_details.publisher.clone());
					}
					Some(UserType::Worker) => {
						winner_account_id
							.push(dispute_details.task_details.worker_id.clone().unwrap());
					}
					// If there is no winner, money is returned to escrow
					None => winner_account_id.push(escrow_id.clone()),
				};

				let task_cost = dispute_details.task_details.cost;
				let task_cost_converted = task_cost.saturated_into::<u128>();
				let court_fee = (task_cost_converted * 60) / 100 as u128;
				let juror_fee: u32 = (court_fee as u32) / (final_jurors_count as u32);

				let juror_account_ids: Vec<_> =
					dispute_details.final_jurors.keys().cloned().collect();

				for juror_account_id in juror_account_ids {
					T::Currency::transfer(
						&escrow_id,
						&juror_account_id,
						juror_fee.into(),
						ExistenceRequirement::KeepAlive,
					)?;
				}

				let remaining_amount = (task_cost_converted * 140) / 100 as u128;
				let mut remaining_amount_converted = remaining_amount as u32;

				if dispute_details.winner == Some(UserType::Customer) {
					// Note - Value should ideally be task cost and not remaining amount/2
					let remaining_amount_for_customer = remaining_amount / 2;
					let remaining_amount_converted_for_customer =
						remaining_amount_for_customer as u32;
					remaining_amount_converted = remaining_amount_converted_for_customer;
					T::Currency::transfer(
						&escrow_id,
						&winner_account_id[1],
						remaining_amount_converted_for_customer.into(),
						ExistenceRequirement::KeepAlive,
					)?;
				}

				T::Currency::transfer(
					&escrow_id,
					&winner_account_id[0],
					remaining_amount_converted.into(),
					ExistenceRequirement::AllowDeath,
				)?;

				dispute_details.task_details.status = Status::CaseClosed;

				//TODO -> Account Map storage update
				//TODO -> Courtroom storage Update
				//TODO -> Task details update
				//TODO -> Delete instance from timeframe once case is closed

				log::info!("Hello, this is court fee{:#?}", court_fee);
			}

			//log::info!("{:#?}", dispute_details.clone());

			<Courtroom<T>>::insert(&task_id, dispute_details);

			Ok(())
		}

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
			publisher_attachments: Option<Vec<Vec<u8>>>,
		) -> DispatchResult {
			// User authentication
			let who = ensure_signed(origin)?;
			// Fetching the latest task count
			let current_task_count = Self::get_task_count();
			let escrow_id = Self::escrow_account_id(current_task_count.clone() as u32);
			// Locking the amount from the publisher for the task
			T::Currency::transfer(
				&who,
				&escrow_id,
				task_cost.clone(),
				ExistenceRequirement::KeepAlive,
			)?;

			log::info!("Hello, this escrow  balance{:#?}", T::Currency::free_balance(&escrow_id));

			// Details related to task created for storage
			let task_details = TaskDetails {
				task_id: current_task_count.clone(),
				publisher: who.clone(),
				worker_id: None,
				publisher_name: Some(publisher_name.clone()),
				worker_name: None,
				task_tags: task_tags.clone(),
				task_deadline: task_duration.clone(),
				cost: task_cost.clone(),
				status: Default::default(),
				task_description: task_des.clone(),
				attachments: publisher_attachments.clone(),
			};

			// Inserting the new task details to storage
			<TaskStorage<T>>::insert(current_task_count.clone(), task_details);
			// Notifying the user about the transaction event
			Self::deposit_event(Event::TaskCreated(
				who,
				publisher_name.clone(),
				current_task_count.clone(),
				task_duration.clone(),
				task_cost.clone(),
				task_des.clone(),
			));
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
			origin: OriginFor<T>,
			task_id: u128,
			worker_name: Vec<u8>,
		) -> DispatchResult {
			// User authentication
			let bidder = ensure_signed(origin)?;
			// Does task exists?
			ensure!(<TaskStorage<T>>::contains_key(&task_id), <Error<T>>::TaskDoesNotExist);
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
			ensure!(publisher != bidder.clone(), <Error<T>>::UnauthorisedToBid);
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
			<TaskStorage<T>>::insert(&task_id, task.clone());

			let escrow_id = Self::escrow_account_id(task_id.clone() as u32);
			// Locking the amount from the publisher for the task
			T::Currency::transfer(
				&bidder,
				&escrow_id,
				// NOTE: Have to add te amount bid by the worker and not task cost
				task_cost.clone(),
				ExistenceRequirement::KeepAlive,
			)?;

			// Notifying the user
			Self::deposit_event(Event::TaskIsBid(
				bidder.clone(),
				worker_name.clone(),
				task_id.clone(),
			));

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
			worker_attachments: Option<Vec<Vec<u8>>>, // TODO: Mandatory
		) -> DispatchResult {
			// User authentication
			let bidder = ensure_signed(origin)?;
			// Does task exist?
			ensure!(<TaskStorage<T>>::contains_key(task_id.clone()), <Error<T>>::TaskDoesNotExist);
			// Get task details from storage
			let mut task = Self::task(task_id.clone());
			// Accessing task status
			let status = task.status;
			// Is task in progress?
			ensure!(status == Status::InProgress, <Error<T>>::TaskIsNotInProgress);
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
			if let Some(attachments) = existing_attachments {
				updated_attachments.extend(attachments.clone());
			}
			// update only if new attachments exist
			if let Some(work_attachments) = worker_attachments {
				updated_attachments.extend(work_attachments.clone());
			}
			// Updating the attachments for storage
			task.attachments = Some(updated_attachments);
			// Inserting the updated task details
			<TaskStorage<T>>::insert(&task_id, task.clone());
			// Notify user
			Self::deposit_event(Event::TaskCompleted(
				worker.clone(),
				task_id.clone(),
				publisher.clone(),
			));

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
			rating_for_the_worker: u8,
		) -> DispatchResult {
			// User authentication
			let publisher = ensure_signed(origin)?;
			// Does task exist?
			ensure!(<TaskStorage<T>>::contains_key(task_id.clone()), <Error<T>>::TaskDoesNotExist);
			// Getting task details from storage
			let mut task = Self::task(task_id.clone());
			// Accessing task status
			let status = task.status;
			// Is approval pending?
			ensure!(status == Status::PendingApproval, <Error<T>>::TaskIsNotPendingApproval);
			// Accessing publisher
			let approver = task.publisher.clone();
			// Is publisher the approver?
			ensure!(publisher == approver.clone(), <Error<T>>::UnauthorisedToApprove);
			// Checking if the worker is set or not
			let bidder = task.worker_id.clone().ok_or(<Error<T>>::WorkerNotSet)?;
			// Getting Worker Rating from RatingMap
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
			rating_for_customer: u8,
		) -> DispatchResult {
			// User authentication
			let bidder = ensure_signed(origin)?;
			// Getting task details from storage
			let mut task = Self::task(task_id.clone());
			// Accessing status
			let status = task.status;
			// Is rating pending from worker to publisher?
			ensure!(status == Status::PendingRatings, <Error<T>>::TaskIsNotPendingRating);
			// Get worker id
			let worker = task.worker_id.clone().ok_or(<Error<T>>::WorkerNotSet)?;
			// Is worker the bidder?
			ensure!(worker == bidder.clone(), <Error<T>>::UnauthorisedToProvideCustomerRating);
			// Accessing reference of the publisher
			let customer = &task.publisher;
			// Get existing customer ratings
			let existing_customer_rating: User<T::AccountId> =
				Self::get_customer_ratings(&customer);
			// Creating a temp rating vector
			let mut temp_rating_vec = Vec::<u8>::new();
			// Looping over all the existing customer ratings
			for rating in existing_customer_rating.ratings_vec {
				temp_rating_vec.push(rating);
			}
			// Updating temp rating vector with new rating
			temp_rating_vec.push(rating_for_customer);
			// Creating new user instance with new rating
			let curr_customer_ratings =
				User::new(customer.clone(), UserType::Customer, temp_rating_vec);
			// Inserting new user instance in customer rating storage
			<CustomerRatings<T>>::insert(customer.clone(), curr_customer_ratings.clone());
			// Accessing task cost
			let transfer_amount = task.cost;
			// Getting escrow account id of the task
			let escrow_id = Self::escrow_account_id(task_id.clone() as u32);
			// Transfering amount from customer to bidder
			T::Currency::transfer(
				&escrow_id,
				&bidder,
				transfer_amount,
				ExistenceRequirement::KeepAlive,
			)?;
			// Updating task status
			task.status = Status::Completed;
			// Inserting updated task details
			TaskStorage::<T>::insert(&task_id, task.clone());
			// Notify user about the transfered amount
			Self::deposit_event(Event::AmountTransfered(
				customer.clone(),
				bidder.clone(),
				transfer_amount.clone(),
			));
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
			transfer_amount: BalanceOf<T>,
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
			Self::deposit_event(Event::TransferMoney(
				sender.clone(),
				sender_account_balance.clone(),
				updated_sender_account_balance.clone(),
				to.clone(),
				to_account_balance.clone(),
				updated_to_account_balance.clone(),
			));

			Ok(())
		}
	}

	//Helper functions for our pallet

	impl<T: Config> Pallet<T> {
		pub fn register_dispute(
			task_id: u128,
			mut task_details: TaskDetails<T::AccountId, BalanceOf<T>>,
		) {

				let case_period = Self::calculate_case_period(task_details.clone());

				task_details.status = Status::DisputeRaised;
	
				let potential_jurors = Self::potential_jurors(task_details.clone());
	
				let dispute = CourtDispute {
					task_details,
					potential_jurors,
					final_jurors: BTreeMap::new(),
					winner: None,
					votes_for_worker: None,
					votes_for_customer: None,
					avg_worker_rating: None,
					avg_publisher_rating: None,
					jury_acceptance_period: case_period.0,
					total_case_period: case_period.1,
				};
	
				<Courtroom<T>>::insert(task_id, dispute);
			}

		pub fn settle_dispute(block_number: BlockNumberOf<T>) {
			let dispute_timeframes = Self::get_dispute_timeframes();
			for dispute_timeframe in dispute_timeframes.iter() {
				if block_number == dispute_timeframe.jury_acceptance_period {
					let mut dispute_details =
						Self::get_disupte_details(dispute_timeframe.task_id.clone());
					dispute_details.task_details.status = Status::VotingPeriod;
					<Courtroom<T>>::insert(&dispute_timeframe.task_id, dispute_details.clone());
				}
				// Case: where no one has voted (If there are zero votes)
				// Checking for number of votes
				// If 0 we force close it
				// If not 0, we check the number of votes
				if block_number == dispute_timeframe.total_case_period {
					let mut dispute_details =
						Self::get_disupte_details(dispute_timeframe.task_id.clone());
					dispute_details.avg_publisher_rating = Some(3);
					dispute_details.avg_worker_rating = Some(3);
					<Courtroom<T>>::insert(&dispute_timeframe.task_id, dispute_details.clone());
				}
			}
		}

		pub fn potential_jurors(
			task_details: TaskDetails<T::AccountId, BalanceOf<T>>,
		) -> Vec<T::AccountId> {
			let all_account_details = <AccountMap<T>>::iter();
			let mut jurors: Vec<T::AccountId> = Vec::new();

			for (acc_id, acc_details) in all_account_details {
				if acc_details.avg_rating >= Some(4) {
					for task_tag in &task_details.task_tags {
						if acc_details.tags.contains(&task_tag)
							&& acc_id.clone() != task_details.publisher
							&& Some(acc_id.clone()) != task_details.worker_id
						{
							jurors.push(acc_id.clone());
							break;
						}
					}
				}
			}

			jurors
		}

		pub fn escrow_account_id(id: u32) -> T::AccountId {
			T::PalletId::get().into_sub_account(id)
		}

		// -> Change

		pub fn calculate_case_period(
			task_details: TaskDetails<T::AccountId, BalanceOf<T>>,
		) -> (BlockNumberOf<T>, BlockNumberOf<T>) {
			// One era is one day
			const ONE_ERA: u32 = 20;
			// Retrieving complete task details
			let task_id = task_details.task_id.clone();
			// Time span for participant to become jurors
			let jury_acceptance_period = <frame_system::Pallet<T>>::block_number() + ONE_ERA.into();
			// Total case time
			let total_case_period = jury_acceptance_period + (ONE_ERA * 2).into();
			// Structure for time frame storage
			let dispute_timeframe =
				DisputeTimeframe { task_id, jury_acceptance_period, total_case_period };
			// Get the time frame storage vector
			let mut dispute_timeframe_storage = Self::get_dispute_timeframes();
			// Updating the timeframe storage vector
			dispute_timeframe_storage.push(dispute_timeframe);
			// Updating the timeframe storage
			<Timeframes<T>>::put(dispute_timeframe_storage);

			(jury_acceptance_period, total_case_period)
		}

		// -> Change

		fn roundoff(total_rating: u8, number_of_users: u8) -> u8 {
			let output: u8;
			let avg_rating: f32 = total_rating as f32 / number_of_users as f32;
			let rounded_avg_rating: u8 = avg_rating as u8;
			if avg_rating.fract() > 0.5 {
				output = rounded_avg_rating + 1;
			} else {
				output = rounded_avg_rating - 1;
			}

			output
		}
	}
}
