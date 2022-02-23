#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod shuffle;

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
			tokens::ExistenceRequirement, Currency, LockableCurrency,
		},
	};

	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};
	use num_traits::float::Float;
	use codec::{Decode, Encode};
	use sp_std::collections::btree_map::BTreeMap;
	use sp_std::vec::Vec;
	// use parity_scale_codec::alloc::string::ToString;
	use crate::shuffle::dot_shuffle;
	// use serde::__private::ToString;

	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type Item<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;

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
		CustomerRatingPending,
		CustomerRatingProvided,
		/// -> Court Period Statuses
		DisputeRaised,
		VotingPeriod,
		JuryDecisionReached,
		/// -> Court Period Statuses
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
		UnsatisfiedPublisherRating,
		AgainstPublisher,
		AgaisntWorker,
	}

	#[derive(Encode, Decode, Default, Debug, PartialEq, Clone, Eq, TypeInfo)]
	pub struct TaskDetails<AccountId, Balance, BlockNumber> {
		pub task_id: u128,
		pub publisher: AccountId,
		pub worker_id: Option<AccountId>,
		pub publisher_name: Option<Vec<u8>>,
		pub worker_name: Option<Vec<u8>>,
		pub task_tags: Vec<TaskTypeTags>,
		pub task_deadline: u64,
		pub cost: Balance,
		pub status: Status,
		pub task_description: Vec<u8>,
		pub attachments: Option<Vec<Vec<u8>>>,
		pub dispute: Option<CourtDispute<AccountId, BlockNumber>>,
		pub final_worker_rating: Option<u8>,
		pub final_customer_rating: Option<u8>,
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
	pub struct SudoJurorPayload {
		voted_for: UserType,
		publisher_rating: u8,
		worker_rating: u8,
	}

	#[derive(Encode, Decode, Default, Debug, PartialEq, Clone, Eq, TypeInfo)]
	pub struct JurorDecisionDetails {
		voted_for: Option<UserType>,
		publisher_rating: Option<u8>,
		worker_rating: Option<u8>,
	}

	#[derive(Encode, Decode, Default, Debug, PartialEq, Clone, Eq, TypeInfo)]
	pub struct Hearing<BlockNumber> {
		task_id: u128,
		jury_acceptance_period: BlockNumber,
		total_case_period: BlockNumber,
		trial_number: u8,
		is_active: bool,
	}

	#[derive(Encode, Decode, Default, Debug, PartialEq, Clone, Eq, TypeInfo)]
	pub struct TaskAutocompletion<BlockNumber> {
		task_id: u128,
		task_will_complete_at: BlockNumber,
	}

	#[derive(Encode, Decode, Default, Debug, PartialEq, Clone, Eq, TypeInfo)]
	pub struct CourtDispute<AccountId, BlockNumber> {
		potential_jurors: Vec<AccountId>,
		final_jurors: BTreeMap<AccountId, JurorDecisionDetails>,
		winner: Option<UserType>,
		votes_for_worker: Option<u8>,
		votes_for_customer: Option<u8>,
		avg_worker_rating: Option<u8>,
		avg_publisher_rating: Option<u8>,
		jury_acceptance_period: BlockNumber,
		total_case_period: BlockNumber,
		sudo_juror: Option<AccountId>,
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
		pub sudo: bool
	}

	impl<Balance> AccountDetails<Balance> {

		pub fn update_rating<T: Config>(account_id: T::AccountId, new_rating: u8) {
			let mut account_details = <AccountMap<T>>::get(account_id.clone());
			let mut all_ratings = account_details.ratings;
			all_ratings.push(new_rating);
			let avg_rating = Some(Self::get_list_average(all_ratings.clone()));
			account_details.avg_rating = avg_rating;
			account_details.ratings = all_ratings.clone();
			<AccountMap<T>>::insert(account_id, account_details);
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
		type PalletId: Get<PalletId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// ----- ON-CHAIN storages
	#[pallet::storage]
	#[pallet::getter(fn accounts)]
	pub type AccountMap<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, AccountDetails<BalanceOf<T>>, ValueQuery>;

	// * Genesis configuration
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
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
			// Creating new accounts
			for (a, b) in &self.account_map {
				<AccountMap<T>>::insert(a, b);
			}
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn get_task_count)]
	pub type TaskCount<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn task)]
	pub(super) type TaskStorage<T: Config> =
		StorageMap<_, Blake2_128Concat, u128, TaskDetails<T::AccountId, BalanceOf<T>, BlockNumberOf<T>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_worker_ratings)]
	pub(super) type WorkerRatings<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, User<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_customer_ratings)]
	pub(super) type CustomerRatings<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, User<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_account_balances)]
	pub(super) type AccountBalances<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_count)]
	pub(super) type Count<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_transfers)]
	pub(super) type Transfers<T: Config> =
		StorageValue<_, Vec<TransferDetails<T::AccountId, BalanceOf<T>>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_hearings)]
	pub(super) type Hearings<T: Config> =
		StorageValue<_, Vec<Hearing<BlockNumberOf<T>>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_task_autocompletions)]
	pub(super) type TaskAutocompletions<T: Config> =
		StorageValue<_, Vec<TaskAutocompletion<BlockNumberOf<T>>>, ValueQuery>;
	// -----

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		TaskCreated(T::AccountId, Vec<u8>, u128, u64, BalanceOf<T>, Vec<u8>),
		TaskIsBid(u128, T::AccountId, Vec<u8>),
		TaskCompleted(u128, T::AccountId),
		TaskApproved(u128, T::AccountId),
		AmountTransfered(T::AccountId, T::AccountId, BalanceOf<T>),
		TaskClosed(u128),
		AccBalance(T::AccountId, BalanceOf<T>),
		CountIncreased(u128),
		TransferMoney(T::AccountId, BalanceOf<T>, BalanceOf<T>, T::AccountId, BalanceOf<T>, BalanceOf<T>),
		CourtSummoned(u128, UserType, Reason, T::AccountId),
		NewJurorAdded(u128, T::AccountId),
		CustomerRatingProvided(u128, T::AccountId, u8, T::AccountId),
		VoteRecorded(u128,T::AccountId),
		CourtAdjourned(u128),
		CourtReinitiated(u128),
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
		/// To ensure Customer Rating exists
		CustomerRatingNotProvided,
		/// To ensure Court is not summoned again for the same task
		DisputeAlreadyRaised,
		/// To ensure the correct id for raising a dispute
		UnauthorisedToRaiseDispute
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(now: T::BlockNumber) -> Weight {
			let total_weight: Weight = 10;
			Self::collect_cases(now);
			total_weight
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000)]
		pub fn sudo_juror_vote(
			origin: OriginFor<T>,
			task_id: u128,
			voted_for: UserType,
			customer_rating: u8,
			worker_rating: u8,
		) -> DispatchResult {
			// Sudo juror authentication
			let sudo_juror = ensure_signed(origin)?;
			// Does task exist?
			ensure!(<TaskStorage<T>>::contains_key(task_id.clone()), <Error<T>>::TaskDoesNotExist);
			// Get task details using task id
			let mut task_details = Self::task(task_id.clone());
			// Accessing dispute details of the task
			let mut dispute_details = task_details.dispute.clone().unwrap();
			// Only the selected sudo juror can complete the case
			ensure!(dispute_details.sudo_juror.clone().unwrap() == sudo_juror, <Error<T>>::UnauthorisedToComplete);
			// Creating the sudo juror details structure
			let juror_details = JurorDecisionDetails {
				voted_for: Some(voted_for.clone()),
				publisher_rating: Some(customer_rating),
				worker_rating: Some(worker_rating),
			};
			// Updating the final jurors map
			dispute_details.final_jurors.insert(sudo_juror.clone(), juror_details);
			// Accessing number of votes for the publisher
			let mut votes_for_customer = dispute_details.votes_for_customer.clone().unwrap_or(0);
			// Accessing number of votes for the worker
			let mut votes_for_worker = dispute_details.votes_for_worker.clone().unwrap_or(0);
			
			// ------ Allocating the vote to the respective party
			match voted_for {
				UserType::Customer => { 
					votes_for_customer += 1;
					dispute_details.votes_for_customer = Some(votes_for_customer);
				},
				UserType::Worker => { 
					votes_for_worker += 1; 
					dispute_details.votes_for_worker = Some(votes_for_worker);
				},
			}
			// ------

			// Adding the dispute details to task details structure
			task_details.dispute = Some(dispute_details);
			// Updating the task details storage
			<TaskStorage<T>>::insert(task_id.clone(), task_details);
			// Concluding the case
			Self::adjourn_court(task_id.clone());
			
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn raise_dispute(
			origin: OriginFor<T>,
			task_id:u128,
			user_type: UserType,
		)->DispatchResult {

			// User authentication
			let who = ensure_signed(origin)?;
			// Ensure task exists and is active
			ensure!(<TaskStorage<T>>::contains_key(&task_id), <Error<T>>::TaskDoesNotExist);
			// Get task details from storage
			let task_details = Self::task(task_id.clone());
			// Accessing status from task details
			let status = task_details.status.clone();

			if user_type == UserType::Customer {
				ensure!(task_details.publisher.clone() == who, <Error<T>>::UnauthorisedToRaiseDispute);
			} else if user_type == UserType::Worker {
				ensure!(task_details.worker_id.clone().unwrap() == who, <Error<T>>::UnauthorisedToRaiseDispute);
			}

			ensure!(status != Status::InProgress, <Error<T>>::TaskInProgress);
			ensure!(status !=Status::Completed, <Error<T>>::TaskIsNotOpen);
			ensure!(status !=Status::DisputeRaised, <Error<T>>::DisputeAlreadyRaised);
			ensure!(status !=Status::VotingPeriod, <Error<T>>::DisputeAlreadyRaised);

			Self::register_case(task_id.clone(), task_details);

			let against = match user_type {
				UserType::Customer => Reason::AgaisntWorker,
				UserType::Worker => Reason::AgainstPublisher,
			};

			// Notify event for summoning court
			Self::deposit_event(Event::CourtSummoned(task_id, user_type, against, who));


			Ok(())

		}

		#[pallet::weight(10_000)]
		pub fn disapprove_rating(
			origin: OriginFor<T>,
			task_id: u128,
			user_type: UserType,
		) -> DispatchResult {
			// User authentication
			let who = ensure_signed(origin)?;
			// Ensure task exists and is active
			ensure!(<TaskStorage<T>>::contains_key(&task_id), <Error<T>>::TaskDoesNotExist);
			// Get task details from storage
			let task_details = Self::task(task_id.clone());
			// Accessing status from task details
			let status = task_details.status.clone();
			// Ensuring if publisher hasn't provided ratings to the worker

			if user_type == UserType::Customer{
				ensure!(status == Status::CustomerRatingProvided, <Error<T>>::CustomerRatingNotProvided); 
			} else{
				ensure!(status == Status::CustomerRatingPending, <Error<T>>::TaskIsNotPendingRating);
			}
			
			// Regsiter case with the court
			Self::register_case(task_id.clone(), task_details);
			// Show reason respective to the caller
			let reason = match user_type {
				UserType::Customer => Reason::UnsatisfiedPublisherRating,
				UserType::Worker => Reason::UnsatisfiedWorkerRating,
			};
			// Notify event for summoning court
			Self::deposit_event(Event::CourtSummoned(task_id, user_type, reason, who));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn disapprove_task(origin: OriginFor<T>, task_id: u128) -> DispatchResult {
			// User authentication
			let publisher = ensure_signed(origin)?;
			// Ensure task exists and is active
			ensure!(<TaskStorage<T>>::contains_key(&task_id), <Error<T>>::TaskDoesNotExist);
			// Getting task details from storage
			let task_details = Self::task(task_id.clone());
			// Accessing status from task details
			let status = task_details.status.clone();
			// Accessing publisher id from task details
			let customer = task_details.publisher.clone();
			// Ensuring that the customer hasn't approved the task
			ensure!(status == Status::PendingApproval, <Error<T>>::TaskInProgress);
			// Ensure the customer is the one disapproving the task
			ensure!(publisher == customer, <Error<T>>::UnauthorisedToDisapprove);
			// Register the case in court
			Self::register_case(task_id.clone(), task_details);
			// Notifying that the court is summoned
			Self::deposit_event(Event::CourtSummoned(
				task_id,
				UserType::Customer,
				Reason::DisapproveTask,
				publisher,
			));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn accept_jury_duty(origin: OriginFor<T>, task_id: u128) -> DispatchResult {
			// User authentication
			let juror = ensure_signed(origin)?;
			// Ensure task exists and is active
			ensure!(<TaskStorage<T>>::contains_key(&task_id), <Error<T>>::TaskDoesNotExist);
			// Getting task details from storage
			let mut task_details = Self::task(task_id.clone());
			// Ensuring if dispute is raised
			ensure!(task_details.dispute != None, <Error<T>>::DisputeDoesNotExist);
			// Accessing the dispute details related to the task 
			let mut dispute_details = task_details.dispute.unwrap();

			// ----- To stop accepting participants for jury after elapsed time
			let current_period = <frame_system::Pallet<T>>::block_number();
			let jury_acceptance_period = dispute_details.jury_acceptance_period.clone();
			ensure!(
				current_period < jury_acceptance_period,
				<Error<T>>::JurySelectionPeriodElapsed
			);
			// -----

			// Ensuring if one is potential juror
			ensure!(dispute_details.potential_jurors.contains(&juror), <Error<T>>::NotPotentialJuror);
			// Less than 2 is 2 people as we are ensuring first and then storing
			ensure!(dispute_details.final_jurors.len() < 2, <Error<T>>::CannotAddMoreJurors);
			// Creating the initial structure of the final jurors
			let juror_details = JurorDecisionDetails {
				voted_for: None,
				publisher_rating: None,
				worker_rating: None,
			};
			// Updating the final jurors map
			dispute_details.final_jurors.insert(juror.clone(), juror_details);
			// Adding the dispute details to task details structure
			task_details.dispute = Some(dispute_details);
			// Notifying addition of new jurors
			Self::deposit_event(Event::NewJurorAdded(task_id.clone(), juror));
			// Updating task details storage
			<TaskStorage<T>>::insert(&task_id, task_details);

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
			// User authentication
			let juror = ensure_signed(origin)?;
			// Ensure task exists and is active
			ensure!(<TaskStorage<T>>::contains_key(&task_id), <Error<T>>::TaskDoesNotExist);
			// Getting task details from storage
			let mut task_details = Self::task(task_id.clone());
			// Ensuring if dispute is raised
			ensure!(task_details.dispute != None, <Error<T>>::DisputeDoesNotExist);
			// Accessing the dispute details related to the task 
			let mut dispute_details = task_details.dispute.unwrap();
			
			// ----- To stop jurors to vote before the actual voting period
			let current_period = <frame_system::Pallet<T>>::block_number();
			let jury_acceptance_period = dispute_details.jury_acceptance_period.clone();
			ensure!(current_period > jury_acceptance_period, <Error<T>>::JurySelectionInProcess);
			// -----

			// Ensure if the voting period is in progress
			ensure!(task_details.status == Status::VotingPeriod, <Error<T>>::CaseClosed);
			// Get details of the final juror
			let mut juror_decision_details = dispute_details.final_jurors.get(&juror).cloned().unwrap();
			// Ensuring final juror doesn't vote more than once
			ensure!(juror_decision_details.voted_for == None, <Error<T>>::JurorHasVoted);

			// ----- Updating juror details structure
			juror_decision_details.voted_for = Some(voted_for.clone());
			juror_decision_details.publisher_rating = Some(customer_rating);
			juror_decision_details.worker_rating = Some(worker_rating);
			// -----

			// Updating decision details in storage
			dispute_details.final_jurors.insert(juror.clone(), juror_decision_details);
			// Total votes of customer
			let mut votes_for_customer = dispute_details.votes_for_customer.unwrap_or(0);
			// Total votes of worker
			let mut votes_for_worker = dispute_details.votes_for_worker.unwrap_or(0);

			// ----- Updating vote count 
			match voted_for {
				UserType::Customer => {
					votes_for_customer += 1;
					dispute_details.votes_for_customer = Some(votes_for_customer);
				}
				UserType::Worker => {
					votes_for_worker += 1;
					dispute_details.votes_for_worker = Some(votes_for_worker);
				}
			}
			// -----
			
			// Updating the task details structure
			task_details.dispute = Some(dispute_details);
			// Updating the task details storage
			<TaskStorage<T>>::insert(&task_id, task_details);

			Self::deposit_event(Event::VoteRecorded(task_id.clone(),juror));

			Ok(())
		}

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
				dispute: None,
				final_worker_rating: None,
				final_customer_rating: None,
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
			// Getting escrow a/c id
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
				task_id.clone(),
				bidder.clone(),
				worker_name.clone(),
			));

			Ok(())
		}

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
				task_id.clone(),
				worker.clone(),
			));

			Ok(())
		}

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

			task.final_worker_rating = Some(rating_for_the_worker.clone());

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
			task.status = Status::CustomerRatingPending;
			// Inserting updated task into storage
			<TaskStorage<T>>::insert(&task_id, task.clone());
			// Notify user
			Self::deposit_event(
				Event::TaskApproved(
					task_id.clone(), 
					publisher.clone()
				));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn provide_customer_rating(
			origin: OriginFor<T>,
			task_id: u128,
			rating_for_customer: u8,
		) -> DispatchResult {
			// User authentication
			let bidder = ensure_signed(origin)?;
			// Does task exist?
			ensure!(<TaskStorage<T>>::contains_key(task_id.clone()), <Error<T>>::TaskDoesNotExist);
			// Getting task details from storage
			let mut task = Self::task(task_id.clone());
			// Accessing status
			let status = task.status;
			// Is rating pending from worker to publisher?
			ensure!(status == Status::CustomerRatingPending, <Error<T>>::TaskIsNotPendingRating);
			// Get worker id
			let worker = task.worker_id.clone().ok_or(<Error<T>>::WorkerNotSet)?;
			// Is worker the bidder?
			ensure!(worker == bidder.clone(), <Error<T>>::UnauthorisedToProvideCustomerRating);
			// Accessing reference of the publisher
			let customer = &task.publisher;

			task.final_customer_rating = Some(rating_for_customer.clone());


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

			task.status = Status::CustomerRatingProvided;
			<TaskStorage<T>>::insert(&task_id, task.clone());

			// Notify the user about the task being closed
			Self::deposit_event(
				Event::CustomerRatingProvided(
					task_id.clone(),
					bidder.clone(),
					rating_for_customer,
					customer.clone()
				));

			Ok(())
		}

		#[pallet::weight(100)]
		pub fn close_task(
			origin: OriginFor<T>,
			task_id: u128,	
		) -> DispatchResult {

			// User authentication
			let publisher = ensure_signed(origin)?;
			// Does task exist?
			ensure!(<TaskStorage<T>>::contains_key(task_id.clone()), <Error<T>>::TaskDoesNotExist);
			// Getting task details from storage
			let mut task_details = Self::task(task_id.clone());
			// Accessing task status
			let status = task_details.status;
			// Is approval pending?
			ensure!(status == Status::CustomerRatingProvided, <Error<T>>::CustomerRatingNotProvided);
			// Accessing publisher
			let approver = task_details.publisher.clone();
			// Is publisher the approver?
			ensure!(publisher == approver.clone(), <Error<T>>::UnauthorisedToApprove);

			let worker_id = task_details.worker_id.clone().unwrap();
			let escrow_id = Self::escrow_account_id(task_id.clone() as u32);
			let transfer_amount = T::Currency::free_balance(&escrow_id);
			T::Currency::transfer(
				&escrow_id,
				&worker_id,
				transfer_amount,
				ExistenceRequirement::AllowDeath,
			)?;
			task_details.status = Status::Completed;

			TaskStorage::<T>::insert(&task_id, task_details);

			Self::deposit_event(Event::AmountTransfered(
				publisher,
				worker_id,
				transfer_amount.clone(),
			));
			Self::deposit_event(Event::TaskClosed(task_id.clone()));
			Ok(())

		}

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

	// Helper functions

	impl<T: Config> Pallet<T> {

		pub fn update_account_storage(task_id: u128) {}

		pub fn adjourn_court(task_id: u128) -> Option<bool> {
			// ----- Initializations
			let mut total_publisher_rating: u8 = 0;
			let mut total_worker_rating: u8 = 0;
			let mut winner_account_id: Vec<T::AccountId> = Vec::new();
			let mut task_details = Self::task(task_id.clone());
			let mut dispute_details = task_details.dispute.clone().unwrap();
			let worker_id = task_details.worker_id.clone().unwrap();
			let publisher_id = task_details.publisher.clone();
			let escrow_id = Self::escrow_account_id(task_id.clone() as u32);
			let votes_for_customer: u8 = dispute_details.votes_for_customer.unwrap_or(0);
			let votes_for_worker: u8 = dispute_details.votes_for_worker.unwrap_or(0);
			let mut is_active = true;
			// * To keep track of the juror participation in voting
			// NOTE: Disabling no votes from juror check
			// let total_votes_cast: u8 = votes_for_customer + votes_for_worker;
			let final_jurors_details: BTreeMap<T::AccountId, JurorDecisionDetails> = 
				dispute_details.clone().final_jurors.into_iter().filter(|(_, value)| value.voted_for != None).collect();
			let final_jurors_count: u8 = final_jurors_details.len() as u8;
			// -----

			if final_jurors_count > 0 && votes_for_customer != votes_for_worker {

				// ----- Calculating total rating for publisher and worker from jurors
				for juror_decision in final_jurors_details.values() {
					total_publisher_rating += juror_decision.publisher_rating.unwrap_or(0);
					total_worker_rating += juror_decision.worker_rating.unwrap_or(0);
				}
				// -----

				// Calculating average publisher rating
				let avg_publisher_rating = Self::roundoff(total_publisher_rating, final_jurors_count.clone());
				// Updating average publisher rating
				dispute_details.avg_publisher_rating = Some(avg_publisher_rating);
				// Calculating average worker rating
				let avg_worker_rating = Self::roundoff(total_worker_rating, final_jurors_count.clone());
				// Updating average worker rating
				dispute_details.avg_worker_rating = Some(avg_worker_rating);

				// ----- Deciding the winner based on votes
				if votes_for_customer > votes_for_worker {
					dispute_details.winner = Some(UserType::Customer);
				} else if votes_for_customer < votes_for_worker {
					dispute_details.winner = Some(UserType::Worker);
				} else {
					// * If votes are even and if no one votes
					dispute_details.winner = None;
				}
				// -----

				// ----- Updating the winner a/c id vector with respective publisher & worker ids
				match dispute_details.winner.clone() {
					Some(UserType::Customer) => {
						winner_account_id.push(worker_id.clone());
						winner_account_id.push(publisher_id.clone());
					}
					Some(UserType::Worker) => {
						winner_account_id.push(worker_id.clone());
					}
					// * If no one wins, publisher and worker should get half
					None => {
						winner_account_id.push(worker_id.clone());
						winner_account_id.push(publisher_id.clone());
					}
				};
				// -----

				// Accessing the task cost 
				let task_cost = task_details.cost;
				// Converting task cost to u128
				let task_cost_converted = task_cost.saturated_into::<u128>();
				// Initializing placeholder
				let remaining_amount;

				// ----- Only calculate court fees if jurors have voted (Not used)
				// if total_votes_cast != 0 {
				let court_fee = (task_cost_converted * 60) / 100 as u128;
				let juror_fee: u32 = (court_fee as u32) / (final_jurors_count as u32);
				let juror_account_ids: Vec<_> = final_jurors_details.keys().cloned().collect();
				// * Transfer to all jurors their respective fees
				for juror_account_id in juror_account_ids {
					T::Currency::transfer(
						&escrow_id,
						&juror_account_id,
						juror_fee.into(),
						ExistenceRequirement::KeepAlive,
					).ok()?;
				}
				remaining_amount = (task_cost_converted * 140) / 100 as u128;

				// Convert remaining amount to u32
				let mut remaining_amount_converted = remaining_amount as u32;

				// ----- Checking if winner is customer or no one
				if dispute_details.winner == Some(UserType::Customer) || dispute_details.winner == None
				{
					// NOTE: AccountMap value should ideally be task cost & bidder cost and not remaining amount/2
					let remaining_amount_for_customer = remaining_amount / 2;
					let remaining_amount_converted_for_customer = remaining_amount_for_customer as u32;
					remaining_amount_converted = remaining_amount_converted_for_customer;
					// * Transfering to winner account
					T::Currency::transfer(
						&escrow_id,
						&winner_account_id[1],
						remaining_amount_converted_for_customer.into(),
						ExistenceRequirement::KeepAlive,
					).ok()?;
				}
				// -----

				// Transfering to winner account
				T::Currency::transfer(
					&escrow_id,
					&winner_account_id[0],
					remaining_amount_converted.into(),
					ExistenceRequirement::AllowDeath,
				).ok()?;
				// Updating the task details structure
				task_details.final_worker_rating = dispute_details.avg_publisher_rating.clone();
				task_details.final_customer_rating = dispute_details.avg_worker_rating.clone();
				task_details.dispute = Some(dispute_details);
				task_details.status = Status::Completed;
				// Updating the task details storage
				<TaskStorage<T>>::insert(&task_id, task_details);
				

				AccountDetails::<pallet::AccountDetails<_> as Trait>::BalanceOf::update_rating::<T>(publisher_id.clone(), task_details.final_customer_rating.unwrap().clone());
				AccountDetails::<pallet::AccountDetails<_> as Trait>::BalanceOf::update_rating::<T>(worker_id.clone(), task_details.final_worker_rating.unwrap().clone());

				Self::deposit_event(
					Event::CourtAdjourned(
						task_id.clone()
					)
				);
				// } 
				// -----

			} else {
				dispute_details.sudo_juror = Some(Self::pick_sudo_juror(task_details.publisher, task_details.worker_id.unwrap()));
				task_details.dispute = Some(dispute_details);
				is_active = false;
			}
 	
			Some(is_active)
		}

		pub fn register_case(
			task_id: u128,
			mut task_details: TaskDetails<T::AccountId, BalanceOf<T>, BlockNumberOf<T>>,
		) {
			// Getting the jury acceptance period and total case period
			let case_period = Self::calculate_case_period(task_details.clone());
			// Updating the status when dispute is raised
			task_details.status = Status::DisputeRaised;
			// Getting all the potential jurors
			let potential_jurors = Self::potential_jurors(task_details.clone());
			// Creating the court dispute structure
			let dispute = CourtDispute {
				potential_jurors,
				final_jurors: BTreeMap::new(),
				winner: None,
				votes_for_worker: None,
				votes_for_customer: None,
				avg_worker_rating: None,
				avg_publisher_rating: None,
				jury_acceptance_period: case_period.0,
				total_case_period: case_period.1,
				sudo_juror: None
			};
			// Updating task details structure
			task_details.dispute = Some(dispute);
			// Updating the task details storage 
			<TaskStorage<T>>::insert(task_id, task_details);
		}

		pub fn collect_cases(block_number: BlockNumberOf<T>) {
			// Getting hearings vector from storage
			let mut hearings: Vec<Hearing<BlockNumberOf<T>>> = Self::get_hearings();
			// Only retain those hearings with case ending period >= current block number
			hearings.retain(|x| x.total_case_period >= block_number || x.is_active); 

			// ----- Validating jury acceptance period and total case period
			for hearing in hearings.iter_mut() {
				let mut task_details = Self::task(hearing.task_id.clone());
				let mut dispute_details = task_details.dispute.clone().unwrap();
				// For stopping unlimited court reinitiations
				if hearing.trial_number >= 3 {
					dispute_details.sudo_juror = Some(Self::pick_sudo_juror(task_details.publisher.clone(), task_details.worker_id.clone().unwrap()));
					task_details.dispute = Some(dispute_details);
					hearing.is_active = false;
					<TaskStorage<T>>::insert(&hearing.task_id, task_details);
				}
				// * For jury acceptance period
				else if block_number == hearing.jury_acceptance_period {
					if dispute_details.final_jurors.len() == 0 {
						hearing.jury_acceptance_period += 5u128.saturated_into();
						hearing.total_case_period += 5u128.saturated_into();
						hearing.trial_number += 1;
						task_details.status = Status::DisputeRaised;
						dispute_details.jury_acceptance_period = hearing.jury_acceptance_period.clone();
						dispute_details.total_case_period = hearing.total_case_period.clone();
						dispute_details.potential_jurors = Self::potential_jurors(task_details.clone());
						task_details.dispute = Some(dispute_details);
					} else {
						// * Change status when atleast 1 final juror accepted jury duty
						task_details.status = Status::VotingPeriod;
					}
					<TaskStorage<T>>::insert(&hearing.task_id, task_details);
				} 
				// * For total case period
				else if block_number == hearing.total_case_period {
					let total_votes = dispute_details.votes_for_worker.unwrap_or(0) + dispute_details.votes_for_customer.unwrap_or(0);
					if total_votes == 0 {
						hearing.jury_acceptance_period += 5u128.saturated_into();
						hearing.total_case_period += 5u128.saturated_into();
						hearing.trial_number += 1;
						task_details.status = Status::DisputeRaised;
						dispute_details.jury_acceptance_period = hearing.jury_acceptance_period.clone();
						dispute_details.total_case_period = hearing.total_case_period.clone();
						dispute_details.potential_jurors = Self::potential_jurors(task_details.clone());
						// * Clearing the list of final jurors as people may have accepted jury duty
						if dispute_details.final_jurors.len() != 0 {
							dispute_details.final_jurors.clear();
						}
						task_details.dispute = Some(dispute_details);
						<TaskStorage<T>>::insert(&hearing.task_id, task_details);
					} else {
						// * Adjourn court 
						let is_active = Self::adjourn_court(hearing.task_id).unwrap();
						if !is_active {
							dispute_details.sudo_juror = Some(Self::pick_sudo_juror(task_details.publisher.clone(), task_details.worker_id.clone().unwrap()));
							task_details.dispute = Some(dispute_details);
							hearing.is_active = false;
							<TaskStorage<T>>::insert(&hearing.task_id, task_details);
						}
					}
				}
			}
			// -----
			
			// Updating the hearings storage
			<Hearings<T>>::put(hearings);
		}

		pub fn potential_jurors(
			task_details: TaskDetails<T::AccountId, BalanceOf<T>, BlockNumberOf<T>>,
		) -> Vec<T::AccountId> {
			// Creating iterator of account map storage
			let all_account_details = <AccountMap<T>>::iter();
			// Initializing empty vector for storing potentials jurors
			let mut jurors: Vec<T::AccountId> = Vec::new();

			// ----- Collecting all potential jurors based on certain conditions
			for (acc_id, acc_details) in all_account_details {
				if acc_details.avg_rating >= Some(4) && !acc_details.sudo {
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
			// -----

			jurors
		}

		pub fn pick_sudo_juror(
			publisher_id: T::AccountId,
			worker_id: T::AccountId,
		) -> T::AccountId {
			// Creating iterator of account map storage
			let all_account_details = <AccountMap<T>>::iter();
			// Storage all sudo users
			let mut all_sudo_account_ids: Vec<T::AccountId> = Vec::new();

			// ----- Verify and collect sudo users
			for (acc_id, acc_details) in all_account_details {
				if acc_details.sudo && acc_id != worker_id && acc_id != publisher_id {
					all_sudo_account_ids.push(acc_id);
				}
			}
			// -----

			// Get current block number
			let block_number = <frame_system::Pallet<T>>::block_number();
			// Length of the acount id list
			let length = all_sudo_account_ids.len() as u32;
			// Calling the shuffling algorithm
			let random_vector = dot_shuffle::<Item<T>>(all_sudo_account_ids, block_number.saturated_into::<u32>(), length);
			
			random_vector.first().unwrap().clone()
		}

		pub fn escrow_account_id(id: u32) -> T::AccountId {
			// Creating and calling sub account
			T::PalletId::get().into_sub_account(id)
		}

		pub fn calculate_case_period(
			task_details: TaskDetails<T::AccountId, BalanceOf<T>, BlockNumberOf<T>>,
		) -> (BlockNumberOf<T>, BlockNumberOf<T>) {
			// One era is one day
			const ONE_ERA: u32 = 5;
			// Retrieving complete task details
			let task_id = task_details.task_id.clone();
			// Time span for participant to become jurors
			let jury_acceptance_period = <frame_system::Pallet<T>>::block_number() + ONE_ERA.into();
			// Total case time
			let total_case_period = jury_acceptance_period + (ONE_ERA * 2).into();
			// Initiate trial
			let trial_number = 1;
			// Court dispute status
			let is_active = true;
			// Structure for time frame storage
			let dispute_timeframe =
				Hearing { task_id, jury_acceptance_period, total_case_period, trial_number, is_active };
			// Get the time frame storage vector
			let mut dispute_timeframe_storage = Self::get_hearings();
			// Updating the timeframe storage vector
			dispute_timeframe_storage.push(dispute_timeframe);
			// Updating the timeframe storage
			<Hearings<T>>::put(dispute_timeframe_storage);

			(jury_acceptance_period, total_case_period)
		}

		pub fn roundoff(total_rating: u8, number_of_users: u8) -> u8 {
			// For carrying the result
			let output: u8;
			// Calculating the average rating in floating point value
			let avg_rating: f32 = total_rating as f32 / number_of_users as f32;
			// Converting floating point to integer
			let rounded_avg_rating: u8 = avg_rating as u8;
			// Removing the decimal from float
			let fraction = avg_rating.fract();

			// ----- Result at different conditions
			if rounded_avg_rating != 0 {
				if fraction >= 0.5 {
					output = rounded_avg_rating + 1;
				} else if fraction == 0.0 {
					output = rounded_avg_rating;
				} else {
					output = rounded_avg_rating - 1;
				}
			} else {
				output = 0;
			}
			// -----

			output
		}
	}
}
