use node_template_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
	SystemConfig, WASM_BINARY, Balance, TaskingConfig, pallet_tasking::AccountDetails, 
	pallet_tasking::TaskTypeTags
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;


/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {

	const VALUE: Balance = 235813;

	// 12 accounts
	let accounts_to_map: Vec<AccountId> =
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		];

	// 12 account details
	let account_details: Vec<AccountDetails<Balance>> = vec![
		AccountDetails {
			balance: 1 << 60,
			ratings: [3, 5, 4, 2, 4].to_vec(),
			avg_rating: Some(4),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec()
		},
		AccountDetails {
			balance: 1 << 60,
			ratings: [3, 5, 5, 2, 5].to_vec(),
			avg_rating: Some(4),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec()
		},
		AccountDetails {
			balance: 1 << 60,
			ratings: [4, 4, 4, 4, 4].to_vec(),
			avg_rating: Some(4),
			tags: [TaskTypeTags::FullStackDevelopment, TaskTypeTags::WebDevelopment].to_vec()
		},
		AccountDetails {
			balance: 1 << 60,
			ratings: [3, 1, 3, 2, 4].to_vec(),
			avg_rating: Some(3),
			tags: [TaskTypeTags::CoreBlockchainDevelopment, TaskTypeTags::WebDevelopment].to_vec()
		},
		AccountDetails {
			balance: 1 << 60,
			ratings: [3, 5, 5, 2, 5].to_vec(),
			avg_rating: Some(4),
			tags: [TaskTypeTags::FullStackDevelopment, TaskTypeTags::WebDevelopment].to_vec()
		},
		AccountDetails {
			balance: 1 << 60,
			ratings: [3, 3, 4, 5, 4].to_vec(),
			avg_rating: Some(4),
			tags: [TaskTypeTags::MobileDevelopment].to_vec()
		},
		AccountDetails {
			balance: 1 << 60,
			ratings: [5, 5, 5, 5, 5].to_vec(),
			avg_rating: Some(5),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec()
		},
		AccountDetails {
			balance: 1 << 60,
			ratings: [3, 5, 3, 3, 3].to_vec(),
			avg_rating: Some(3),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec()
		},
		AccountDetails {
			balance: 1 << 60,
			ratings: [3, 5, 4, 2, 4].to_vec(),
			avg_rating: Some(4),
			tags: [TaskTypeTags::MachineLearning, TaskTypeTags::DeepLearning].to_vec()
		},
		AccountDetails {
			balance: 1 << 60,
			ratings: [4, 5, 5, 5, 4].to_vec(),
			avg_rating: Some(5),
			tags: [TaskTypeTags::FullStackDevelopment, TaskTypeTags::MobileDevelopment].to_vec()
		},
		AccountDetails {
			balance: 1 << 60,
			ratings: [3, 5, 4, 2, 4].to_vec(),
			avg_rating: Some(4),
			tags: [TaskTypeTags::CoreBlockchainDevelopment, TaskTypeTags::MachineLearning].to_vec()
		},
		AccountDetails {
			balance: 1 << 60,
			ratings: [3, 5, 1, 2, 3].to_vec(),
			avg_rating: Some(3),
			tags: [TaskTypeTags::CoreBlockchainDevelopment, TaskTypeTags::WebDevelopment].to_vec()
		},
	];

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
		tasking: TaskingConfig {
			// single_value: VALUE,
			account_map: accounts_to_map.iter().zip(account_details.iter()).map(|(x, acc_details)| (x.clone(), acc_details.clone())).collect(),
		},
		transaction_payment: Default::default(),
	}
}
