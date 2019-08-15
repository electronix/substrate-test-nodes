use primitives::{sr25519, Pair};
use test_node_runtime::{
	AccountId, GenesisConfig, AuraConfig, BalancesConfig,
	SudoConfig, IndicesConfig, SystemConfig, WASM_BINARY, AuraId, ContractConfig,
	currency::*
};
use aura_primitives::sr25519::AuthorityPair as AuraPair;
use contracts;
use substrate_service;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

/// The chain specification option. This is expected to come in from the CLI and
/// is little more than one of a number of alternatives which can easily be converted
/// from a string (`--chain=...`) into a `ChainSpec`.
#[derive(Clone, Debug)]
pub enum Alternative {
	/// Whatever the current runtime is, with just Alice as an auth.
	Development,
	/// Whatever the current runtime is, with simple Alice/Bob auths.
	LocalTestnet,
}

fn authority_key(s: &str) -> AuraId {
	AuraPair::from_string(&format!("//{}", s), None)
		.expect("static values are valid; qed")
		.public()
}

fn account_key(s: &str) -> AccountId {
	sr25519::Pair::from_string(&format!("//{}", s), None)
		.expect("static values are valid; qed")
		.public()
}

impl Alternative {
	/// Get an actual chain config from one of the alternatives.
	pub(crate) fn load(self) -> Result<ChainSpec, String> {
		Ok(match self {
			Alternative::Development => ChainSpec::from_genesis(
				"Development",
				"dev",
				|| testnet_genesis(vec![
					authority_key("Alice")
				], vec![
					account_key("Alice")
				],
					account_key("Alice")
				),
				vec![],
				None,
				None,
				None,
				None
			),
			Alternative::LocalTestnet => ChainSpec::from_genesis(
				"Local Testnet",
				"local_testnet",
				|| testnet_genesis(vec![
					authority_key("Alice"),
					authority_key("Bob"),
				], vec![
					account_key("Alice"),
					account_key("Bob"),
					account_key("Charlie"),
					account_key("Dave"),
					account_key("Eve"),
					account_key("Ferdie"),
				],
					account_key("Alice"),
				),
				vec![],
				None,
				None,
				None,
				None
			),
		})
	}

	pub(crate) fn from(s: &str) -> Option<Self> {
		match s {
			"dev" => Some(Alternative::Development),
			"" | "local" => Some(Alternative::LocalTestnet),
			_ => None,
		}
	}
}

fn testnet_genesis(initial_authorities: Vec<AuraId>, endowed_accounts: Vec<AccountId>, root_key: AccountId) -> GenesisConfig {
	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		aura: Some(AuraConfig {
			authorities: initial_authorities.clone(),
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.clone(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
			vesting: vec![],
		}),
		contracts: Some(ContractConfig {
			current_schedule: contracts::Schedule {
				enable_println: true, // this should only be enabled on development chains
				..Default::default()
			},
			gas_price: 1 * MILLICENTS,
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
	}
}