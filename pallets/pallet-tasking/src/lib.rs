#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod utils;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::PalletId;
	use frame_system::pallet_prelude::*;
	use frame_support::{
		sp_runtime,
		sp_runtime::traits::{AccountIdConversion, SaturatedConversion, Hash},
		traits::{
			tokens::ExistenceRequirement, Currency, LockableCurrency,
		},
		transactional,
	};

	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};
	use codec::{Decode, Encode};
	use sp_std::collections::btree_map::BTreeMap;
	use sp_std::vec::Vec;
	use crate::utils::{dot_shuffle,roundoff, create_milestone_id, get_milestone_and_project_id};

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

	// Enum for the status of the Project
	#[derive(Encode, Decode, PartialEq, Eq, Debug, Clone, TypeInfo)]
	pub enum ProjectStatus {
		/// Project is ready to be submitted into the marketplace
		Ready,
		/// Project is Open
		Open,
		/// Project has been completed or closed
		Closed,
	}

	// Default for the project status
	impl Default for ProjectStatus {
		fn default() -> Self {
			ProjectStatus::Ready
		}
	}


	#[derive(Encode, Decode, PartialEq, Eq, Debug, Clone, TypeInfo)]
	pub enum Reason {
		DisapproveTask,
		UnsatisfiedWorkerRating,
		UnsatisfiedPublisherRating,
		AgainstPublisher,
		AgainstWorker,
	}

	// a struct for the input of milestones
	#[derive(Encode, Decode, PartialEq, Eq, Debug, Clone, TypeInfo)]
	pub struct MilestoneHelper<Balance> {
		name: Vec<u8>,
		cost: Balance,
		tags: Vec<TaskTypeTags>,
		publisher_attachments: Vec<Vec<u8>>
	}

	// Struct for Project Description
	#[derive(Encode, Decode, Default, Debug, PartialEq, Clone, Eq, TypeInfo)]
	pub struct ProjectDetails<AccountId, Balance, BlockNumber> {
		pub project_id: u128,
		pub publisher: AccountId,
		pub project_name: Vec<u8>,
		pub tags: Vec<TaskTypeTags>, // other approach possible
		pub publisher_name: Option<Vec<u8>>,
		pub milestones: Option<Vec<Milestone<AccountId, Balance, BlockNumber>>>,
		pub overall_customer_rating: Option<u8>,
		pub status: ProjectStatus,
	}

	// Implementation for the Project Struct
	impl<AccountId, Balance, BlockNumber> ProjectDetails<AccountId, Balance, BlockNumber> {
		pub fn new(project_id:u128, project_name: Vec<u8>, tags: Vec<TaskTypeTags>, publisher: AccountId, publisher_name: Vec<u8>) -> Self {
			ProjectDetails{
				project_id: project_id,
				project_name: project_name,
				tags: tags,
				publisher: publisher,
				publisher_name: Some(publisher_name),
				milestones: None,
				overall_customer_rating: None,
				status: Default::default(),
			}
		}
	}


	// Milestone struct
	#[derive(Encode, Decode, Default, Debug, PartialEq, Clone, Eq, TypeInfo)]
	pub struct Milestone<AccountId, Balance, BlockNumber> {
		pub milestone_id: Vec<u8>,
		pub milestone_name: Vec<u8>,
		pub tags: Vec<TaskTypeTags>,
		pub cost: Balance,
		pub status: Status,
		pub worker_id: Option<AccountId>,
		pub worker_name: Option<Vec<u8>>,
		pub publisher_attachments: Option<Vec<Vec<u8>>>,
		pub worker_attachments: Option<Vec<Vec<u8>>>,
		pub dispute: Option<CourtDispute<AccountId, BlockNumber>>,
		pub final_worker_rating: Option<u8>,
		pub final_customer_rating: Option<u8>,
	}

	// Implementation of the milestone struct
	impl<AccountId, Balance, BlockNumber> Milestone<AccountId, Balance, BlockNumber>{
		fn new(
			milestone_id: Vec<u8>,
			milestone_name: Vec<u8>,
			tags: Vec<TaskTypeTags>,
			cost: Balance,
			publisher_attachments: Vec<Vec<u8>>,
		) -> Self {
			Milestone{
				milestone_id: milestone_id,
				milestone_name: milestone_name,
				tags: tags,
				cost: cost,
				status: Default::default(),
				worker_id: None,
				worker_name: None,
				publisher_attachments: Some(publisher_attachments),
				worker_attachments: None,
				dispute: None,
				final_worker_rating: None,
				final_customer_rating: None
			}
		}
	}

	// Bid struct
	#[derive(Encode, Decode, Default, Debug, PartialEq, Clone, Eq, TypeInfo)]
	pub struct Bid<Balance, AccountId>{
		pub bid_number: u32,
		pub bidder_id: AccountId,
		pub bidder_name: Vec<u8>,
		pub account: AccountDetails<Balance>,
	}

	impl<Balance, AccountId> Bid<Balance, AccountId> {
		pub fn new(
			bid_number: u32, 
			bidder_id: AccountId,
			bidder_name: Vec<u8>, 
			account: AccountDetails<Balance>
		) -> Self {
			Bid {
				bid_number,
				bidder_id,
				bidder_name,
				account
			}
		}
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

	#[derive(Encode, Decode, Default, Debug, PartialEq, Clone, Eq, TypeInfo)]
	pub struct Hearing<BlockNumber> {
		milestone_id: Vec<u8>,
		start_case_period: BlockNumber,
		total_case_period: BlockNumber,
		trial_number: u8,
		is_active: bool,
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
		start_case_period: BlockNumber,
		total_case_period: BlockNumber,
		sudo_juror: Option<AccountId>,
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
			let average = roundoff(total_sum, list_len);
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

		#[pallet::constant]
		type MaxMilestoneLimit: Get<u8>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn accounts)]
	pub type AccountMap<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, AccountDetails<BalanceOf<T>>, ValueQuery>;

	// * Genesis configuration for accounts
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub account_map: Vec<(T::AccountId, AccountDetails<BalanceOf<T>>)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> GenesisConfig<T> {
		/// Direct implementation of `GenesisBuild::build_storage`.
		///
		/// Kept in order not to break dependency.
		pub fn build_storage(&self) -> Result<sp_runtime::Storage, String> {
			<Self as GenesisBuild<T>>::build_storage(self)
		}

		/// Direct implementation of `GenesisBuild::assimilate_storage`.
		///
		/// Kept in order not to break dependency.
		pub fn assimilate_storage(&self, storage: &mut sp_runtime::Storage) -> Result<(), String> {
			<Self as GenesisBuild<T>>::assimilate_storage(self, storage)
		}
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
	#[pallet::getter(fn get_project_count)]
	pub type ProjectCount<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_project)]
	pub(super) type ProjectStorage<T: Config> =
	    StorageMap<_, Blake2_128Concat, u128, ProjectDetails<T::AccountId, BalanceOf<T>, BlockNumberOf<T>>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_bidder_list)]
	pub type BidderList<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Vec<Bid<BalanceOf<T>, T::AccountId>>, ValueQuery>;

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
		// CourtSummoned(u128, UserType, Reason, T::AccountId),
		// NewJurorAdded(u128, T::AccountId),
		// CustomerRatingProvided(u128, T::AccountId, u8, T::AccountId),
		VoteRecorded(u128,T::AccountId),
		/// Court has been adjourned for [MilestoneId].
		CourtAdjourned(Vec<u8>),
		/// Court has been reinitiated for [MilestoneId]
		CourtReinitiated(Vec<u8>),
		/// Case has been closed by a sudo juror. \[MilestoneId, SudoJuror]
		CaseClosedBySudoJuror(Vec<u8>, T::AccountId),
		// phase 3 events here
		/// A project has been created. [ProjectId, ProjectName, Publisher].
		ProjectCreated(u128,Vec<u8>,T::AccountId),
		/// A milestone has been created for some project. \[MilestoneId, \Cost].
		MileStoneCreated(Vec<u8>, BalanceOf<T>),
		/// A project has been added to marketplace. \[ProjectId]
		ProjectAddedToMarketplace(u128),
		/// A bid has been placed for some milestone. \[MilestoneId, AccountId]
		BidSuccessful(Vec<u8>, T::AccountId),
		/// A bid has been accepted for some milestone. \[MilestoneId, BidNumber]
		BidAccepted(Vec<u8>, u32),
		/// A milestone has been completed. \[MilestoneId]
		MilestoneCompleted(Vec<u8>),
		/// A milestone has been approved and a worker rating has been provided. \[MilestoneId, rating]
		MilestoneApproved(Vec<u8>,u8),
		/// Customer rating has been provided for a milestone. \[MilestoneId, rating]
		CustomerRatingProvided(Vec<u8>,u8),
		/// A milestone has been closed. \[MilestoneId]
		MilestoneClosed(Vec<u8>),
		/// A project has been closed. \[ProjectId]
		ProjectClosed(u128),
		/// Court has been summoned. \[MilestoneId, UserType, Reason]
		CourtSummoned(Vec<u8>, UserType, Reason),
		/// New juror has been added. \[MilestoneId, AccountId]
		NewJurorAdded(Vec<u8>, T::AccountId),
		/// Collect cases has been called
		CollectCasesCalled,
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
		UnauthorisedToRaiseDispute,

		// Phase 3 errors start here
		/// Maximum limit of u128 hit
		CannotCreateFurtherProjects,
		/// To ensure that project exists
		ProjectDoesNotExist,
		/// To ensure that project does not have more than 5 milestones
		MilestoneLimitReached,
		/// To ensure that project has atleast 1 milestone
		MilestoneRequired,
		/// To ensure the publisher is making the transaction
		Unauthorised,
		/// The project should be ready before adding to the marketplace
		ProjectNotReady,
		/// Ensuring that the project is not closed
		ProjectClosed,
		/// Ensuring the milestone id is valid
		InvalidMilestoneId,
		/// Ensuring that project is open for bidding
		ProjectNotOpenForBidding,
		/// Ensuring that the milestone is open for bidding
		MilestoneNotOpenForBidding,
		/// Ensuring that the owner do not bid for the milestone
		PublisherCannotBid,
		/// Ensuring that the bid number is valid
		InvalidBidNumber,
		/// Something went wrong while transfering from escrow to account id
		FailedToTransferBack,
		/// Ensuring that the milestone is in progress
		MilestoneNotInProgress,
		/// Ensuring that the publisher does not attempt to complete the milestone
		PublisherCannotCompleteMilestone,
		/// Ensuring that the project is open before submission of milestone
		ProjectNotOpenForMilestoneCompletion,
		/// Project should be open while providing rating
		ProjectNotOpenToProvideRating,
		/// Publisher should not rate themself
		PublisherCannotRateSelf,
		/// The project cannot be closed for the following reasons : One or more milestones are not either open or completed
		CannotCloseProject,
		/// Project not open
		ProjectNotOpen,
		/// Raising a dispute for a milestone that has just been open
		MilestoneJustOpened,
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
			milestone_id: Vec<u8>,
			voted_for: UserType,
			customer_rating: u8,
			worker_rating: u8,
		) -> DispatchResult {
			// authentication
			let sender = ensure_signed(origin)?;
			let mut milestone_id_clone = milestone_id.clone();
			let (milestone_number, project_id) = get_milestone_and_project_id(&mut milestone_id_clone).map_err(|_| <Error<T>>::InvalidMilestoneId)?;
			<ProjectStorage<T>>::try_mutate(&project_id, |option_project| {
				let res;
				match option_project {
					None => res = Err(<Error<T>>::ProjectDoesNotExist),
					Some(project) => {
						let publisher_id = project.publisher.clone();
						match project.status {
							ProjectStatus::Open => {
								match &mut project.milestones {
									None => res = Err(<Error<T>>::InvalidMilestoneId),
									Some(vector_of_milestones) => {
										let mut call_adjourn_court = false;
										if milestone_number >= vector_of_milestones.len() as u8 {
											res = Err(<Error<T>>::InvalidMilestoneId);
										}else{
											match vector_of_milestones[milestone_number as usize].status {
												Status::DisputeRaised => {
													match &mut vector_of_milestones[milestone_number as usize].dispute {
														None => res = Err(<Error<T>>::DisputeDoesNotExist),
														Some(dispute) => {
															match &dispute.sudo_juror {
																None => res = Err(<Error<T>>::DisputeDoesNotExist),
																Some(sudo_juror) => {
																	if *sudo_juror != sender {
																		res = Err(<Error<T>>::UnauthorisedToComplete);
																	}else{
																		let sudo_juror_details = JurorDecisionDetails {
																			voted_for: Some(voted_for.clone()),
																			publisher_rating: Some(customer_rating),
																			worker_rating: Some(worker_rating),
																		};
																		dispute.final_jurors.insert(sender.clone(), sudo_juror_details);
																		let mut votes_for_customer = dispute.votes_for_customer.clone().unwrap_or(0);
																		let mut votes_for_worker = dispute.votes_for_worker.clone().unwrap_or(0);
																		match voted_for {
																			UserType::Customer => {
																				votes_for_customer += 1;
																				dispute.votes_for_customer = Some(votes_for_customer);
																			},
																			UserType::Worker => {
																				votes_for_worker += 1;
																				dispute.votes_for_worker = Some(votes_for_worker);
																			}
																		}
																		// Self::adjourn_court();
																		call_adjourn_court = true;
																		res = Ok(());
																		Self::deposit_event(Event::CaseClosedBySudoJuror(milestone_id, sender));
																	}
																}
															}
														}
													}
												},
												_ => res = Err(<Error<T>>::CaseClosed)
											}
										}
										if call_adjourn_court {
											Self::adjourn_court(publisher_id, &mut vector_of_milestones[milestone_number as usize]);
										}
									}
								}
							},
							_ => res = Err(<Error<T>>::ProjectNotOpen)
						}
					}
				}
				res
			})?;
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn raise_dispute(
			origin: OriginFor<T>,
			milestone_id: Vec<u8>,
			user_type: UserType,
		) -> DispatchResult {
			// function body starts here
			// user authentication
			let sender = ensure_signed(origin)?;
			let mut milestone_id_clone = milestone_id.clone();
			let (milestone_number, project_id) = get_milestone_and_project_id(&mut milestone_id_clone).map_err(|_| <Error<T>>::InvalidMilestoneId)?;
			<ProjectStorage<T>>::try_mutate(&project_id, |option_project|{
				let mut res = Ok(());
				match option_project {
					Some(project) => {
						if project.status != ProjectStatus::Open {
							res = Err(<Error<T>>::ProjectNotOpen);
						}else if project.publisher != sender{
							res = Err(<Error<T>>::Unauthorised);
						}else{
							match &mut project.milestones {
								Some(vector_of_milestones) => {
									if milestone_number >= vector_of_milestones.len() as u8{
										res = Err(<Error<T>>::InvalidMilestoneId);
									}else{
										match vector_of_milestones[milestone_number as usize].status {
											Status::Open => res = Err(<Error<T>>::MilestoneJustOpened),
											Status::InProgress => res = Err(<Error<T>>::TaskInProgress),
											Status::Completed => res = Err(<Error<T>>::TaskIsNotOpen),
											Status::DisputeRaised => res = Err(<Error<T>>::DisputeAlreadyRaised),
											Status::VotingPeriod => res = Err(<Error<T>>::DisputeAlreadyRaised),
											_ => {
												// register the cse 
												Self::register_case(milestone_id.clone(), &mut vector_of_milestones[milestone_number as usize]);
												let against = match user_type {
													UserType::Customer => Reason::AgainstWorker,
													UserType::Worker => Reason::AgainstPublisher,
												};
												// notify event
												Self::deposit_event(Event::CourtSummoned(milestone_id, user_type, against));
											}
										}
									}
								},
								None => res = Err(<Error<T>>::InvalidMilestoneId)
							}
						}
					},
					None => res = Err(<Error<T>>::ProjectDoesNotExist)
				}
				res
			})?;
			Ok(())
			// function body ends here
		}

		#[pallet::weight(10_000)]
		pub fn disapprove_rating(
			origin: OriginFor<T>,
			milestone_id: Vec<u8>,
			user_type: UserType
		) -> DispatchResult {
			// user authentication
			let sender = ensure_signed(origin)?;
			let mut milestone_id_clone = milestone_id.clone();
			let (milestone_number, project_id) = get_milestone_and_project_id(&mut milestone_id_clone).map_err(|_| <Error<T>>::InvalidMilestoneId)?;
			<ProjectStorage<T>>::try_mutate(&project_id, |option_project|{
				let mut res = Ok(());
				// checking if project exists
				match option_project{
					// project exists
					Some(project) => {
						// checking the status of the project
						match project.status{
							// project should not be closed
							ProjectStatus::Closed => res = Err(<Error<T>>::ProjectClosed),
							_ => {
								// checking whether the milestone exists
								match &mut project.milestones {
									// milestone exists
									Some(vector_of_milestones) => {
										if milestone_number >= vector_of_milestones.len() as u8{
											res = Err(<Error<T>>::InvalidMilestoneId);
										}else{
											let reason;
											match user_type{
												UserType::Customer => {
													reason = Reason::UnsatisfiedPublisherRating;
													if project.publisher != sender {
														res = Err(<Error<T>>::Unauthorised);
													}else if vector_of_milestones[milestone_number as usize].status != Status::CustomerRatingProvided{
														res = Err(<Error<T>>::CustomerRatingNotProvided);
													}
												},
												UserType::Worker => {
													reason = Reason::UnsatisfiedWorkerRating;
													if vector_of_milestones[milestone_number as usize].worker_id != Some(sender) {
														res = Err(<Error<T>>::Unauthorised);
													}else if vector_of_milestones[milestone_number as usize].status != Status::CustomerRatingPending{
														res = Err(<Error<T>>::TaskIsNotPendingRating);
													}
												}
											}
											res = match res {
												Ok(()) => {
													Self::register_case(milestone_id.clone(), &mut vector_of_milestones[milestone_number as usize]);
													Self::deposit_event(Event::CourtSummoned(milestone_id, user_type, reason));
													res
												},
												Err(_) => res
											}
										}
									},
									// milestone does not exist
									None => res = Err(<Error<T>>::InvalidMilestoneId)
								}
							}
						}
					},
					// project does not exist
					None => res = Err(<Error<T>>::ProjectDoesNotExist)
				}
				res
			})?;
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn disapprove_milestone(
			origin: OriginFor<T>,
			milestone_id: Vec<u8>
		) -> DispatchResult {
			// authentication
			let sender = ensure_signed(origin)?;
			let mut milestone_id_clone = milestone_id.clone();
			let (milestone_number, project_id) = get_milestone_and_project_id(&mut milestone_id_clone).map_err(|_| <Error<T>>::InvalidMilestoneId)?;
			<ProjectStorage<T>>::try_mutate(&project_id, |option_project| {
				let mut res = Ok(());
				match option_project {
					Some(project) => {
						if project.status != ProjectStatus::Open{
							res = Err(<Error<T>>::ProjectNotOpen);
						}else if project.publisher != sender {
							res = Err(<Error<T>>::Unauthorised);
						}else{
							match &mut project.milestones {
								Some(vector_of_milestones) => {
									if milestone_number >= vector_of_milestones.len() as u8 {
										res = Err(<Error<T>>::InvalidMilestoneId);
									}else{
										match vector_of_milestones[milestone_number as usize].status {
											Status::PendingApproval => {
												Self::register_case(milestone_id.clone(), &mut vector_of_milestones[milestone_number as usize]);
												Self::deposit_event(Event::CourtSummoned(
													milestone_id,
													UserType::Customer,
													Reason::DisapproveTask
												));
											},
											_ => res = Err(<Error<T>>::TaskIsNotPendingApproval)
										}
									}
								},
								None => res = Err(<Error<T>>::InvalidMilestoneId)
							}
						}
					},
					None => res = Err(<Error<T>>::ProjectDoesNotExist)
				}
				res
			})?;
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn accept_jury_duty(
			origin: OriginFor<T>,
			milestone_id: Vec<u8>,
			voted_for: UserType,
			customer_rating: u8,
			worker_rating: u8,
		) -> DispatchResult {
			// authentication
			let sender = ensure_signed(origin)?;
			let mut milestone_id_clone = milestone_id.clone();
			let (milestone_number, project_id) = get_milestone_and_project_id(&mut milestone_id_clone).map_err(|_| <Error<T>>::InvalidMilestoneId)?;
			<ProjectStorage<T>>::try_mutate(&project_id, |option_project|{
				let res;
				match option_project {
					Some(project) => {
						match &mut project.milestones {
							Some(vector_of_milestones) => {
								if milestone_number >= vector_of_milestones.len() as u8 {
									res = Err(<Error<T>>::InvalidMilestoneId);
								}else{
									match vector_of_milestones[milestone_number as usize].status {
										Status::DisputeRaised => {
											match &mut vector_of_milestones[milestone_number as usize].dispute {
												Some(dispute) => {
													let current_period = <frame_system::Pallet<T>>::block_number();
													if current_period >= dispute.total_case_period {
														res = Err(<Error<T>>::JurySelectionPeriodElapsed);
													}else{
														if dispute.potential_jurors.contains(&sender) {
															if dispute.final_jurors.len() >= 2 {
																res = Err(<Error<T>>::CannotAddMoreJurors);
																// call adjourn court here
																// Self::adjourn_court();
															}else{
																let juror_details = JurorDecisionDetails {
																	voted_for: Some(voted_for.clone()),
																	publisher_rating: Some(customer_rating),
																	worker_rating: Some(worker_rating),
																};
																// inserting juror into final juror
																dispute.final_jurors.insert(sender.clone(), juror_details);
																// updating the vote count
																let mut votes_for_customer = dispute.votes_for_customer.unwrap_or(0);
																let mut votes_for_worker = dispute.votes_for_worker.unwrap_or(0);
																match voted_for {
																	UserType::Customer => {
																		votes_for_customer += 1;
																		dispute.votes_for_customer = Some(votes_for_customer);
																	},
																	UserType::Worker => {
																		votes_for_worker += 1;
																		dispute.votes_for_worker = Some(votes_for_worker);
																	}
																}
																Self::deposit_event(Event::NewJurorAdded(milestone_id, sender));
																res = Ok(());
															}
														}else{
															res = Err(<Error<T>>::NotPotentialJuror);
														}
													}
												},
												None => res = Err(<Error<T>>::DisputeDoesNotExist)
											}
										},
										_ => res = Err(<Error<T>>::CaseClosed)
									}
								}
							},
							None => res = Err(<Error<T>>::InvalidMilestoneId)
						}
					},
					None => res = Err(<Error<T>>::ProjectDoesNotExist)
				}
				res
			})?;
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn create_project(
			origin: OriginFor<T>,
			publisher_name: Vec<u8>,
			project_name: Vec<u8>,
			tags: Vec<TaskTypeTags>,
			milestone_one: MilestoneHelper<BalanceOf<T>>,
			add_milestones: Vec<MilestoneHelper<BalanceOf<T>>>
		) -> DispatchResult {
			//function body starts here
			// ensuring that the transaction is signed and getting the account id of the transactor
			let publisher = ensure_signed(origin)?;
			ensure!(add_milestones.len() < 5, <Error<T>>::MilestoneLimitReached);
			let project_id = Self::get_project_count() + 1;
			<ProjectCount<T>>::set(project_id.clone());
			let mut project = ProjectDetails::new(project_id.clone(), project_name.clone(), tags, publisher.clone(), publisher_name);
			let mid = create_milestone_id(project_id, 0);
			let milestone1: Milestone<T::AccountId, BalanceOf<T>, BlockNumberOf<T>> = Milestone::new(mid, milestone_one.name, milestone_one.tags, milestone_one.cost, milestone_one.publisher_attachments);
			let mut vector_of_milestones = Vec::new();
			vector_of_milestones.push(milestone1);
			for milestone_helper in add_milestones {
				let mid = create_milestone_id(project_id, vector_of_milestones.len() as u8);
				let milestone: Milestone<T::AccountId, BalanceOf<T>, BlockNumberOf<T>> = Milestone::new(mid.clone(), milestone_helper.name, milestone_helper.tags, milestone_helper.cost, milestone_helper.publisher_attachments);
				vector_of_milestones.push(milestone);
				Self::deposit_event(Event::MileStoneCreated(mid,milestone_helper.cost));
			}
			project.milestones = Some(vector_of_milestones);
			<ProjectStorage<T>>::insert(&project_id, project);
			Self::deposit_event(Event::ProjectCreated(project_id, project_name, publisher));
			Ok(())
			// function body ends here
		}

		#[pallet::weight(10_000)]
		pub fn add_milestones_to_project(
			origin: OriginFor<T>,
			project_id: u128,
			milestones: Vec<MilestoneHelper<BalanceOf<T>>>
		) -> DispatchResult {
			// function body starts here

			//authentication
			let sender = ensure_signed(origin)?;
			<ProjectStorage<T>>::try_mutate(&project_id, |option_project| {
				let mut res = Ok(());
				match option_project {
					None => res = Err(<Error<T>>::ProjectDoesNotExist),
					Some(project) => {
						if project.publisher != sender {
							res = Err(<Error<T>>::Unauthorised);
						}else{
							match project.status {
								ProjectStatus::Closed => res = Err(<Error<T>>::ProjectClosed),
								_ => {
									match &mut project.milestones {
										None => {
											let mut vector_of_milestones = Vec::new();
											for milestone_helper in milestones {
												let mid = create_milestone_id(project_id, vector_of_milestones.len() as u8);
												let milestone: Milestone<T::AccountId, BalanceOf<T>, BlockNumberOf<T>> = Milestone::new(mid.clone(), milestone_helper.name, milestone_helper.tags, milestone_helper.cost, milestone_helper.publisher_attachments);
												vector_of_milestones.push(milestone);
												Self::deposit_event(Event::MileStoneCreated(mid,milestone_helper.cost));
											}
											project.milestones = Some(vector_of_milestones);
										},
										Some(vector_of_milestones) => {
											if milestones.len() + vector_of_milestones.len() > 5{
												res = Err(<Error<T>>::MilestoneLimitReached);
											}else{
												for milestone_helper in milestones {
													let mid = create_milestone_id(project_id, vector_of_milestones.len() as u8);
													let milestone: Milestone<T::AccountId, BalanceOf<T>, BlockNumberOf<T>> = Milestone::new(mid.clone(), milestone_helper.name, milestone_helper.tags, milestone_helper.cost.clone(), milestone_helper.publisher_attachments);
													vector_of_milestones.push(milestone);
													Self::deposit_event(Event::MileStoneCreated(mid,milestone_helper.cost));
												}	
											}
										}
									}
								}
							}
						}
					}
				}
				res
			})?;
			Ok(())
		}

		#[pallet::weight(1000)]
		pub fn add_project_to_marketplace(
			origin: OriginFor<T>,
			project_id: u128,
		) -> DispatchResult {
			// function body starts here

			// authenticating
			let sender = ensure_signed(origin)?;

			<ProjectStorage<T>>::try_mutate(&project_id, |option| {
				let mut res = Ok(());
				match option {
					Some(project) => {
						if project.publisher != sender {
							res = Err(<Error<T>>::Unauthorised);
						}else if project.status != ProjectStatus::Ready{
							res = Err(<Error<T>>::ProjectNotReady);
						}else{
							project.status = ProjectStatus::Open;
							Self::deposit_event(Event::ProjectAddedToMarketplace(project_id));
						}
					},
					None => res = Err(<Error<T>>::ProjectDoesNotExist)
				}
				res
			})?;

			Ok(())
			// function body ends here
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn bid_for_milestone(
			origin: OriginFor<T>,
			milestone_id: Vec<u8>,
			worker_name: Vec<u8>
		) -> DispatchResult {
			// function body starts here

			// authentication
			let sender = ensure_signed(origin)?;
			let mut milestone_cost: BalanceOf<T> = 0u8.saturated_into();
			let mut milestone_id_clone = milestone_id.clone();
			let (milestone_number, project_id) = get_milestone_and_project_id(&mut milestone_id_clone).map_err(|_| <Error<T>>::InvalidMilestoneId)?;
			// ensure that the project and milestone exists
			<ProjectStorage<T>>::try_mutate(&project_id, |option_project| {
				let mut res = Ok(());
				match option_project{
					Some(project) => {
						if project.status != ProjectStatus::Open {
							res = Err(<Error<T>>::ProjectNotOpenForBidding);
						}else if project.publisher == sender{
							res = Err(<Error<T>>::PublisherCannotBid);
						}else{
							match &mut project.milestones {
								Some(milestone_vector) => {
									if milestone_number >= milestone_vector.len() as u8{
										res = Err(<Error<T>>::InvalidMilestoneId);
									}else{
										if milestone_vector[milestone_number as usize].status != Status::Open {
											res = Err(<Error<T>>::MilestoneNotOpenForBidding);
										}else{
											milestone_cost = milestone_vector[milestone_number as usize].cost.clone();
										}
									}
								},
								None => res = Err(<Error<T>>::InvalidMilestoneId)
							};
						}
					},
					None => res = Err(<Error<T>>::ProjectDoesNotExist)
				}
				res
			})?;
			let account = Self::accounts(sender.clone());
			let milestone_key = T::Hashing::hash_of(&milestone_id);
			<BidderList<T>>::mutate(&milestone_key, |bidder_vector| { // bid vector
				let bid_number = bidder_vector.len() as u32 + 1;
				let bid = Bid::new(bid_number, sender.clone(), worker_name, account);
				bidder_vector.push(bid);
			});
			let escrow_id = Self::get_escrow(milestone_id.clone());
			T::Currency::transfer(
				&sender,
				&escrow_id,
				milestone_cost,
				ExistenceRequirement::KeepAlive,
			)?;
			Self::deposit_event(Event::BidSuccessful(milestone_id,sender));
			Ok(())
			// function body ends here
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn accept_bid(
			origin: OriginFor<T>,
			milestone_id: Vec<u8>,
			bid_number: u32
		) -> DispatchResult {
			// function body starts
			// authentication
			let sender = ensure_signed(origin)?;
			let mut milestone_cost: BalanceOf<T> = 0u8.saturated_into();
			let mut milestone_id_clone = milestone_id.clone();
			let (milestone_number, project_id) = get_milestone_and_project_id(&mut milestone_id_clone).map_err(|_| <Error<T>>::InvalidMilestoneId)?;
			let bid_number = bid_number - 1 as u32;
			<ProjectStorage<T>>::try_mutate(&project_id, |option_project| {
				let mut res = Ok(());
				// ensuring that the project exist
				match option_project {
					Some(project) => {
						// ensuring that the project publisher is making the request
						if project.publisher != sender {
							res = Err(<Error<T>>::Unauthorised);
						}else if project.status != ProjectStatus::Open {
							res = Err(<Error<T>>::ProjectNotOpenForBidding);
						}else{
							// ensuring that the project milestone exists and is open for bidding
							match &mut project.milestones{
								Some(milestone_vector) => {
									if milestone_number >= milestone_vector.len() as u8 {
										res = Err(<Error<T>>::InvalidMilestoneId);
									}else{
										if milestone_vector[milestone_number as usize].status != Status::Open{
											res = Err(<Error<T>>::MilestoneNotOpenForBidding);
										}else{
											let milestone_key = T::Hashing::hash_of(&milestone_id);
											let bidder_list = Self::get_bidder_list(&milestone_key);
											if bidder_list.len() == 0 as usize || bid_number >= bidder_list.len() as u32{
												
												res = Err(<Error<T>>::InvalidBidNumber);
											}else{
												// changing the status of the milestone to in progress
												milestone_vector[milestone_number as usize].status = Status::InProgress;
												// updating the worker id in the milestone
												milestone_vector[milestone_number as usize].worker_id = Some(bidder_list[bid_number as usize].bidder_id.clone());
												// updating the worker name in th milestone
												milestone_vector[milestone_number as usize].worker_name = Some(bidder_list[bid_number as usize].bidder_name.clone());
												milestone_cost = milestone_vector[milestone_number as usize].cost.clone();
											}
										}
									}
								},
								None => res = Err(<Error<T>>::InvalidMilestoneId)
							}
						}
						
					},
					None => res = Err(<Error<T>>::ProjectDoesNotExist),
				};

				res
			})?;
			// locking the tokens of the publisher
			let escrow_id = Self::get_escrow(milestone_id.clone());
			T::Currency::transfer(
				&sender,
				&escrow_id,
				milestone_cost.clone(),
				ExistenceRequirement::KeepAlive
			)?;
			let milestone_key = T::Hashing::hash_of(&milestone_id);
			// rejecting all the other bidders
			Self::reject_all(milestone_key, escrow_id, milestone_cost, bid_number)?;
			Self::deposit_event(Event::BidAccepted(milestone_id, bid_number));
			Ok(())
			// function body ends
		}

		#[pallet::weight(10_000)]
		pub fn milestone_completed(
			origin: OriginFor<T>,
			milestone_id: Vec<u8>,
			worker_attachments: Vec<Vec<u8>>,
		) -> DispatchResult {
			// function body starts here
			// authentication
			let sender = ensure_signed(origin)?;
			let mut milestone_id_clone = milestone_id.clone();
			let (milestone_number, project_id) = get_milestone_and_project_id(&mut milestone_id_clone).map_err(|_| <Error<T>>::InvalidMilestoneId)?;
			<ProjectStorage<T>>::try_mutate(&project_id, |option_project| {
				let mut res = Ok(());
				match option_project{
					Some(project) => {
						if project.status != ProjectStatus::Open{
							res = Err(<Error<T>>::ProjectNotOpenForMilestoneCompletion); 
						}else if sender == project.publisher{
							res = Err(<Error<T>>::PublisherCannotCompleteMilestone);
						}else{
							// checking the milestones
							match &mut project.milestones {
								Some(milestone_vector) => {
									if milestone_number >= milestone_vector.len() as u8{
										res = Err(<Error<T>>::InvalidMilestoneId);
									}else{
										if milestone_vector[milestone_number as usize].status != Status::InProgress {
											res = Err(<Error<T>>::MilestoneNotInProgress);
										}else if milestone_vector[milestone_number as usize].worker_id.clone().unwrap() != sender {
											res = Err(<Error<T>>::UnauthorisedToComplete);
										}else{
											// updating the worker attachments
											milestone_vector[milestone_number as usize].worker_attachments = Some(worker_attachments);
											// updating the status to pending approval
											milestone_vector[milestone_number as usize].status = Status::PendingApproval;
											Self::deposit_event(Event::MilestoneCompleted(milestone_id));
										}
									}
								},
								None => res = Err(<Error<T>>::InvalidMilestoneId)
							}
						}
					},
					None => res = Err(<Error<T>>::ProjectDoesNotExist)
				};
				res
			})?;
			Ok(())
			// function body ends here
		}


		#[pallet::weight(10_000)]
		pub fn approve_milestone(
			origin: OriginFor<T>,
			milestone_id: Vec<u8>,
			rating_for_the_worker: u8,
		) -> DispatchResult {
			// function body starts here
			// authentication
			let sender = ensure_signed(origin)?;
			let mut milestone_id_clone = milestone_id.clone();
			let (milestone_number, project_id) = get_milestone_and_project_id(&mut milestone_id_clone).map_err(|_| <Error<T>>::InvalidMilestoneId)?;
			<ProjectStorage<T>>::try_mutate(&project_id, |option_project| {
				let mut res = Ok(());
				match option_project {
					Some(project) => {
						if project.publisher != sender {
							res = Err(<Error<T>>::UnauthorisedToApprove);
						}else if project.status != ProjectStatus::Open {
							res = Err(<Error<T>>::ProjectNotOpenToProvideRating); 
						}else {
							// checking for milestones
							match &mut project.milestones{
								Some(milestone_vector) => {
									if milestone_number >= milestone_vector.len() as u8 {
										res = Err(<Error<T>>::InvalidMilestoneId);
									}else{
										if milestone_vector[milestone_number as usize].status != Status::PendingApproval {
											res = Err(<Error<T>>::TaskIsNotPendingApproval); // create an error for this later
										}else{
											milestone_vector[milestone_number as usize].status = Status::CustomerRatingPending;
											milestone_vector[milestone_number as usize].final_worker_rating = Some(rating_for_the_worker);
											Self::deposit_event(Event::MilestoneApproved(milestone_id,rating_for_the_worker));
										}
									}
								},
								None => res = Err(<Error<T>>::InvalidMilestoneId)
							};
						}
					},
					None => res = Err(<Error<T>>::ProjectDoesNotExist)
				};
				res
			})?;
			Ok(())
			// function body ends here
		}

		#[pallet::weight(10_000)]
		pub fn provide_customer_rating(
			origin: OriginFor<T>,
			milestone_id: Vec<u8>,
			rating_for_customer: u8,
		) -> DispatchResult {
			// function body starts here
			// authentication
			let sender = ensure_signed(origin)?;
			let mut milestone_id_clone = milestone_id.clone();
			let (milestone_number, project_id) = get_milestone_and_project_id(&mut milestone_id_clone).map_err(|_| <Error<T>>::InvalidMilestoneId)?;
			<ProjectStorage<T>>::try_mutate(&project_id, |option_project| {
				let mut res = Ok(());
				match option_project {
					Some(project) => {
						if project.publisher == sender {
							res = Err(<Error<T>>::PublisherCannotRateSelf);
						}else if project.status != ProjectStatus::Open {
							res = Err(<Error<T>>::ProjectNotOpenToProvideRating);
						}else{
							// checking for milestone
							match &mut project.milestones {
								Some(vector_of_milestones) => {
									if milestone_number >= vector_of_milestones.len() as u8 {
										res = Err(<Error<T>>::InvalidMilestoneId);
									}else{
										if vector_of_milestones[milestone_number as usize].status != Status::CustomerRatingPending {
											res = Err(<Error<T>>::TaskIsNotPendingRating);
										}else{
											if vector_of_milestones[milestone_number as usize].worker_id.clone().unwrap() != sender {
												res = Err(<Error<T>>::UnauthorisedToProvideCustomerRating);
											}else{
												vector_of_milestones[milestone_number as usize].status = Status::CustomerRatingProvided;
												vector_of_milestones[milestone_number as usize].final_customer_rating = Some(rating_for_customer);
												Self::deposit_event(Event::CustomerRatingProvided(milestone_id, rating_for_customer));
											}
										}
									}
								},
								None => res = Err(<Error<T>>::InvalidMilestoneId)
							}
						}
					},
					None => res = Err(<Error<T>>::ProjectDoesNotExist)
				};
				res
			})?;
			Ok(())
			// function body ends here
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn close_milestone(
			origin: OriginFor<T>,
			milestone_id: Vec<u8>
		) -> DispatchResult {
			// function body starts here
			// authentication
			let sender = ensure_signed(origin)?;
			let mut milestone_id_clone = milestone_id.clone();
			let (milestone_number, project_id) = get_milestone_and_project_id(&mut milestone_id_clone).map_err(|_| <Error<T>>::InvalidMilestoneId)?;
			let worker_id = <ProjectStorage<T>>::try_mutate(&project_id, |option_project|{
				let res;
				match option_project {
					Some(project) => {
						if project.publisher != sender {
							res = Err(<Error<T>>::UnauthorisedToApprove); // perhaps create an error for this later
						}else if project.status != ProjectStatus::Open {
							res = Err(<Error<T>>::ProjectNotOpenForBidding); // create an error for this later
						}else{
							match &mut project.milestones {
								Some(vector_of_milestones) => {
									if milestone_number >= vector_of_milestones.len() as u8 {
										res = Err(<Error<T>>::InvalidMilestoneId);
									}else{
										if vector_of_milestones[milestone_number as usize].status != Status::CustomerRatingProvided {
											res = Err(<Error<T>>::CustomerRatingNotProvided);
										}else{
											vector_of_milestones[milestone_number as usize].status = Status::Completed;
											let worker_id = vector_of_milestones[milestone_number as usize].worker_id.clone().unwrap();
											// 	// ----- Update overall customer rating
											<AccountDetails<BalanceOf<T>>>::update_rating::<T>(
												worker_id.clone(),
												vector_of_milestones[milestone_number as usize].final_worker_rating.clone().unwrap()
											);
											// -----
											res = Ok(worker_id);
										}
									}
								},
								None => res = Err(<Error<T>>::InvalidMilestoneId)
							}
						}
					},
					None => res = Err(<Error<T>>::ProjectDoesNotExist)
				}
				res
			})?;
			let escrow_id = Self::get_escrow(milestone_id.clone());
			let transfer_amount = T::Currency::free_balance(&escrow_id);
			T::Currency::transfer(
				&escrow_id,
				&worker_id,
				transfer_amount,
				ExistenceRequirement::AllowDeath
			)?;

			Self::deposit_event(Event::MilestoneClosed(milestone_id));
			Ok(())
			// function body ends here
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn close_project(
			origin: OriginFor<T>,
			project_id: u128,
		) -> DispatchResult {
			// function body starts here
			// authentication
			let sender = ensure_signed(origin)?;
			<ProjectStorage<T>>::try_mutate(&project_id, |option_project| {
				let mut res = Ok(());
				match option_project {
					Some(project) => {
						if sender != project.publisher {
							res = Err(<Error<T>>::Unauthorised);
						}else{
							if project.status == ProjectStatus::Open {
								match &mut project.milestones {
									Some(vector_of_milestones) => {
										let mut flag = false;
										for milestone in vector_of_milestones {
											match milestone.status {
												Status::Completed => flag = false,
												Status::Open => {
													// reject all the bidders
													let milestone_key = T::Hashing::hash_of(&milestone.milestone_id);
													let escrow_id = Self::get_escrow(milestone.milestone_id.clone());
													let cost = milestone.cost.clone();
													let except = u32::MAX;
													res = Self::reject_all(
														milestone_key,
														escrow_id,
														cost,
														except
													);
													flag = false;
												},
												_ => {
													flag = true;
													break;
												}

											}
										}
										if flag {
											res = Err(<Error<T>>::CannotCloseProject);
										}else{
											let publisher_rating = Self::get_publisher_rating(project_id);
											// update publisher rating
											match publisher_rating {
												Some(rating) => {
													project.overall_customer_rating = Some(rating.clone());
													<AccountDetails<BalanceOf<T>>>::update_rating::<T>(sender, rating)
												},
												None => (),
											}
											project.status = ProjectStatus::Closed;
										}
									},
									None => {
										project.status = ProjectStatus::Closed;
										res = Ok(());
									}
								}
							}else{
								project.status = ProjectStatus::Closed;
								res = Ok(());
							}
						}
					},
					None => res = Err(<Error<T>>::ProjectDoesNotExist)
				}
				res
			})?;
			Self::deposit_event(Event::ProjectClosed(project_id));
			Ok(())
			// function body ends here
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
			// Notify event
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
		pub fn adjourn_court(
			publisher_id: T::AccountId,
			milestone: &mut Milestone<T::AccountId, BalanceOf<T>, BlockNumberOf<T>>
		) -> Option<bool> {
			// ------------- Initializations
			let mut total_publisher_rating : u8 = 0;
			let mut total_worker_rating: u8 = 0;
			let mut winner_account_id: Vec<T::AccountId> = Vec::new();
			let worker_id = milestone.worker_id.clone().unwrap();
			let milestone_id = milestone.milestone_id.clone();
			let escrow_id = Self::get_escrow(milestone_id.clone());
			let mut is_active = true;
			let task_cost = milestone.cost.clone();
			match &mut milestone.dispute {
				None => (),
				Some(dispute) => {
					let votes_for_customer: u8 = dispute.votes_for_customer.unwrap_or(0);
					let votes_for_worker: u8 = dispute.votes_for_worker.unwrap_or(0);
					let final_juror_details: BTreeMap<T::AccountId, JurorDecisionDetails> = dispute.final_jurors.clone().into_iter().filter(|(_, value)| value.voted_for != None).collect();
					let final_juror_count = final_juror_details.len() as u8;
					if final_juror_count > 0 && votes_for_customer != votes_for_worker {

						// ---------------- Calculating total rating for publisher and worker from jurors	
						for juror_decision in final_juror_details.values() {
							total_publisher_rating += juror_decision.publisher_rating.unwrap_or(0);
							total_worker_rating += juror_decision.worker_rating.unwrap_or(0);
						}

						// --------------- Calculating avg ratings
						let avg_publisher_rating = roundoff(total_publisher_rating, final_juror_count);
						dispute.avg_publisher_rating = Some(avg_publisher_rating);
						let avg_worker_rating = roundoff(total_worker_rating, final_juror_count);
						dispute.avg_worker_rating = Some(avg_worker_rating);

						// --------------- Deciding the winner based on votes
						if votes_for_customer > votes_for_worker {
							dispute.winner = Some(UserType::Customer);
						}else if votes_for_customer < votes_for_worker {
							dispute.winner = Some(UserType::Worker);
						}else {
							dispute.winner = None
						}

						// ----- Updating the winner a/c id vector with respective publisher & worker ids
						match dispute.winner.clone() {
							Some(UserType::Customer) => {
								winner_account_id.push(worker_id.clone());
								winner_account_id.push(publisher_id.clone());
							},
							Some(UserType::Worker) => {
								winner_account_id.push(worker_id.clone());
							},
							// * If no one wins, publisher and worker should get half
							None => {
								winner_account_id.push(worker_id.clone());
								winner_account_id.push(publisher_id.clone());
							}
						}

						// Converting the task cost to u128
						let task_cost_converted = task_cost.saturated_into::<u128>();
						// Initializing the placeholder
						let remaining_amount;

						// Court commision calculation
						let court_fee = (task_cost_converted * 60) / 100 as u128;
						// Individual juror fee calculation
						let juror_fee = (court_fee as u32) / (final_juror_count as u32);
						// Collecting juror accounts
						let juror_account_ids : Vec<_> = final_juror_details.keys().cloned().collect();
						// Transfer to all jurors their respective fees
						for juror_account_id in juror_account_ids {
							T::Currency::transfer(
								&escrow_id,
								&juror_account_id,
								juror_fee.into(),
								ExistenceRequirement::KeepAlive,
							).ok();
						}
						// Total amount excluding court fees
						remaining_amount = task_cost_converted - court_fee;
						// Convert remaining amount to u32
						let mut remaining_amount_converted = remaining_amount as u32;
						// ---------- Checking if winner is customer or no one
						if dispute.winner == Some(UserType::Customer) || dispute.winner == None {
							let remaining_amount_for_customer = remaining_amount / 2;
							let remaining_amount_converted_for_customer = remaining_amount_for_customer as u32;
							// * Transferring to winner account 
							T::Currency::transfer(
								&escrow_id,
								&winner_account_id[1],
								remaining_amount_converted_for_customer.into(),
								ExistenceRequirement::KeepAlive,
							).ok();
						}

						// Transferring to winner account
						T::Currency::transfer(
							&escrow_id,
							&winner_account_id[0],
							remaining_amount_converted.into(),
							ExistenceRequirement::AllowDeath,
						).ok();

						// Update final worker rating
						milestone.final_worker_rating = dispute.avg_worker_rating.clone();
						// Update final customer rating
						milestone.final_customer_rating = dispute.avg_publisher_rating.clone();

						// ------- Update overall customer and worker ratings
						<AccountDetails<BalanceOf<T>>>::update_rating::<T>(
							publisher_id.clone(),
							milestone.final_customer_rating.clone().unwrap()
						);
						<AccountDetails<BalanceOf<T>>>::update_rating::<T>(
							worker_id.clone(),
							milestone.final_worker_rating.clone().unwrap()
						);
						// Updating the status
						milestone.status = Status::Completed;
						// Notify event
						Self::deposit_event(
							Event::CourtAdjourned(
								milestone_id
							)
						);
					}else {
						// -------- Case handover to sudo juror
						dispute.sudo_juror = Some(
							Self::pick_sudo_juror(
								publisher_id.clone(),
								worker_id.clone(),
							)
						);
						is_active = false;
					}
				}
			};
			Some(is_active)
		}	
		
		pub fn register_case(
			milestone_id: Vec<u8>,
			milestone_details: &mut Milestone<T::AccountId, BalanceOf<T>, BlockNumberOf<T>>,
		) {
			// Getting the jury acceptance period and total case period
			let case_period = Self::calculate_case_period(milestone_id.clone()); 
			// Updating the status when dispute is raised
			milestone_details.status = Status::DisputeRaised;
			// Getting all the potential jurors
			let potential_jurors = Self::potential_jurors(milestone_id);
			// Creating the court dispute structure
			let dispute = CourtDispute {
				potential_jurors,
				final_jurors: BTreeMap::new(),
				winner: None,
				votes_for_worker: None,
				votes_for_customer: None,
				avg_worker_rating: None,
				avg_publisher_rating: None,
				start_case_period: case_period.0,
				total_case_period: case_period.1,
				sudo_juror: None
			};
			// Updating milestone details structure
			milestone_details.dispute = Some(dispute);
		}

		pub fn collect_cases(block_number: BlockNumberOf<T>) {
			Self::deposit_event(Event::CollectCasesCalled);
			// Getting hearings vector from storage
			let mut hearings = Self::get_hearings();
			// Only retain those hearings with case ending period >= current block number
			hearings.retain(|x| x.total_case_period >= block_number && x.is_active); // this
			// ---------- Validating jury acceptance period and total case period
			for hearing in hearings.iter_mut() {
				// For stopping unlimited court reinitiations
				if hearing.trial_number >= 3 {
					let mut milestone_id_clone = hearing.milestone_id.clone();
					let (milestone_number, project_id) = get_milestone_and_project_id(&mut milestone_id_clone).unwrap();
					<ProjectStorage<T>>::try_mutate(&project_id, |option_project| {
						let mut res = Ok(());
						match option_project {
							None => res = Err(<Error<T>>::ProjectDoesNotExist),
							Some(project) => {
								let publisher_id = project.publisher.clone();
								match &mut project.milestones {
									None => res = Err(<Error<T>>::InvalidMilestoneId),
									Some(vector_of_milestones) => {
										let worker_id = vector_of_milestones[milestone_number as usize].worker_id.clone().unwrap();
										match &mut vector_of_milestones[milestone_number as usize].dispute {
											None => res = Err(<Error<T>>::DisputeDoesNotExist),
											Some(dispute) => dispute.sudo_juror = Some(Self::pick_sudo_juror(
												publisher_id.clone(),
												worker_id.clone()
											))
										}
										hearing.is_active = false;
									}
								}
							}
						}
						res
					}).ok();
				}else if block_number == hearing.total_case_period {
					let mut milestone_id_clone = hearing.milestone_id.clone();
					let (milestone_number, project_id) = get_milestone_and_project_id(&mut milestone_id_clone).unwrap();
					<ProjectStorage<T>>::try_mutate(&project_id, |option_project|{
						let mut res = Ok(());
						match option_project {
							None => res = Err(<Error<T>>::ProjectDoesNotExist),
							Some(project) => {
								let publisher_id = project.publisher.clone();
								match &mut project.milestones {
									None => res = Err(<Error<T>>::InvalidMilestoneId),
									Some(vector_of_milestones) => {
										let worker_id = vector_of_milestones[milestone_number as usize].worker_id.clone().unwrap();
										let mut call_adjourn_court = false;
										match &mut vector_of_milestones[milestone_number as usize].dispute {
											None => res = Err(<Error<T>>::DisputeDoesNotExist),
											Some(dispute) => {
												let total_votes = dispute.votes_for_worker.clone().unwrap_or(0) + dispute.votes_for_customer.clone().unwrap_or(0);
												if total_votes == 0 {
													hearing.total_case_period += 5u128.saturated_into();
													hearing.trial_number += 1;
													//vector_of_milestones[milestone_number as usize].status = Status::DisputeRaised;
													dispute.total_case_period = hearing.total_case_period.clone();
													dispute.potential_jurors = Self::potential_jurors(hearing.milestone_id.clone());
													if dispute.final_jurors.len() != 0 {
														dispute.final_jurors.clear();
													}
													call_adjourn_court = false;
												}else {
													// Adjourn court 
													call_adjourn_court = true;
												}
											}
										}
										if call_adjourn_court {
											let is_active = Self::adjourn_court(publisher_id.clone(), &mut vector_of_milestones[milestone_number as usize]).unwrap();
											if !is_active {
												match &mut vector_of_milestones[milestone_number as usize].dispute{
													None => (),
													Some(dispute) => {
														dispute.sudo_juror = Some(Self::pick_sudo_juror(
															publisher_id.clone(),
															worker_id.clone()
														))
													}
												}
											}
										}
									}
								}
							}
						}
						res
					}).ok();
				}
			}
			<Hearings<T>>::put(hearings);
		}


		pub fn potential_jurors(
			milestone_id: Vec<u8>,
		) -> Vec<T::AccountId> {
			// Creating iterator of account map storage
			let all_account_details: Vec<(T::AccountId, AccountDetails<BalanceOf<T>>)> = <AccountMap<T>>::iter().collect();
			// Initializing empty vector for storing potential jurors
			let mut jurors: Vec<T::AccountId> = Vec::new();
			let mut milestone_id_clone = milestone_id.clone();
			let (milestone_number, project_id) = get_milestone_and_project_id(&mut milestone_id_clone).unwrap();
			let publisher = Self::get_project(&project_id).unwrap().publisher;
			let project_details = Self::get_project(project_id).unwrap();
			let milestones = project_details.milestones.unwrap();
			let milestone_details = milestones.get(milestone_number as usize).unwrap();
			// ----- Collecting all potential jurors based on certain conditions
			for (acc_id, acc_details) in all_account_details.into_iter() {
				if acc_details.avg_rating >= Some(4) && !acc_details.sudo {
					for milestone_tag in milestone_details.clone().tags.iter() {
						if acc_details.tags.contains(milestone_tag) 
						&& &acc_id != &publisher
						&& Some(&acc_id) != milestone_details.worker_id.as_ref() {
							jurors.push(acc_id);
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

		pub fn get_escrow(id: Vec<u8>) -> T::AccountId {
			T::PalletId::get().into_sub_account(id)
		}

		pub fn calculate_case_period(
			milestone_id: Vec<u8>
		) -> (BlockNumberOf<T>, BlockNumberOf<T>) {
			// One era is one day
			const ONE_ERA: u32 = 5;
			// Time span for participant to become jurors
			let start_case_period = <frame_system::Pallet<T>>::block_number();
			// Total case time
			let total_case_period = start_case_period + (ONE_ERA * 2).into();
			// Initiate trial
			let trial_number = 1;
			// Court dispute status
			let is_active = true;
			// Structure for time frame storage
			let dispute_timeframe =
				Hearing { milestone_id, start_case_period, total_case_period, trial_number, is_active };
			// Get the time frame storage vector
			let mut dispute_timeframe_storage = Self::get_hearings();
			// Updating the timeframe storage vector
			dispute_timeframe_storage.push(dispute_timeframe);
			// Updating the timeframe storage
			<Hearings<T>>::put(dispute_timeframe_storage);

			(start_case_period, total_case_period)
		}

		// helper function to reject the other biddings and transfer the locked funds back to the bidders
		pub fn reject_all(
			milestone_key: T::Hash,
			escrow_id: T::AccountId,
			cost: BalanceOf<T>,
			except: u32
		)-> Result<(), Error<T>>{
			let bidder_list = Self::get_bidder_list(milestone_key);
			for (index, bidder) in bidder_list.iter().enumerate() {
				if index as u32 == except {
					continue;
				}
				let bidder_id = &bidder.bidder_id;
				T::Currency::transfer(
					&escrow_id,
					bidder_id,
					cost,
					ExistenceRequirement::AllowDeath
				).map_err(|_| <Error<T>>::FailedToTransferBack)?;
			}
			<BidderList<T>>::remove(&milestone_key);
			Ok(())
		}

		// helper function to get publisher rating
		pub fn get_publisher_rating(
			project_id: u128
		) -> Option<u8> {
			let project = Self::get_project(project_id).unwrap();
			let res;
			match project.milestones {
				Some(vector_of_milestones) => {
					let mut number_of_rating = 0;
					let mut total_rating = 0;
					for milestone in vector_of_milestones {
						match milestone.final_customer_rating {
							Some(rating) => {
								number_of_rating += 1;
								total_rating += rating;
							},
							None => (),

						}
					}
					if number_of_rating > 0 {
						res = Some(roundoff(total_rating, number_of_rating));
					}else{
						res = None;
					}
				},
				None => res = None
			}
			res
		}
	}

}

