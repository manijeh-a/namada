//! Default addresses and keys.

#[cfg(any(test, feature = "testing", feature = "benches"))]
pub use dev::{
    addresses, albert_address, albert_keypair, bertha_address, bertha_keypair,
    christel_address, christel_keypair, daewon_address, daewon_keypair,
    ester_address, ester_keypair, keys, tokens, validator_account_keypair,
    validator_address, validator_keypair, validator_keys,
};

#[cfg(any(test, feature = "testing", feature = "benches"))]
mod dev {
    use std::collections::HashMap;

    use lazy_static::lazy_static;
    use namada::ledger::{governance, pgf, pos};
    use namada::types::address::{
        apfel, btc, dot, eth, kartoffel, nam, schnitzel, Address,
    };
    use namada::types::key::*;
    use namada_sdk::wallet::alias::Alias;
    use namada_sdk::wallet::pre_genesis::ValidatorWallet;
    use namada_sdk::wallet::Wallet;

    use crate::wallet::CliWalletUtils;

    /// Get protocol, eth_bridge, and dkg keys from the validator pre-genesis
    /// wallet
    pub fn validator_keys() -> (common::SecretKey, common::SecretKey) {
        let protocol_key = VALIDATOR_WALLET
            .store
            .validator_keys
            .protocol_keypair
            .clone();
        let eth_bridge_key = VALIDATOR_WALLET.eth_hot_key.clone();
        (protocol_key, eth_bridge_key)
    }

    /// The default keys with their aliases.
    pub fn keys() -> HashMap<Alias, common::SecretKey> {
        vec![
            ("albert".into(), albert_keypair()),
            ("bertha".into(), bertha_keypair()),
            ("christel".into(), christel_keypair()),
            ("daewon".into(), daewon_keypair()),
            ("ester".into(), ester_keypair()),
            ("validator".into(), validator_keypair()),
        ]
        .into_iter()
        .collect()
    }

    /// The default tokens with their aliases.
    pub fn tokens() -> HashMap<Address, &'static str> {
        vec![
            (nam(), "NAM"),
            (btc(), "BTC"),
            (eth(), "ETH"),
            (dot(), "DOT"),
            (schnitzel(), "Schnitzel"),
            (apfel(), "Apfel"),
            (kartoffel(), "Kartoffel"),
        ]
        .into_iter()
        .collect()
    }

    /// The default addresses with their aliases.
    pub fn addresses() -> HashMap<Alias, Address> {
        let mut addresses: HashMap<Alias, Address> = vec![
            ("pos".into(), pos::ADDRESS),
            ("pos_slash_pool".into(), pos::SLASH_POOL_ADDRESS),
            ("governance".into(), governance::ADDRESS),
            ("governance".into(), pgf::ADDRESS),
            ("validator".into(), validator_address()),
            ("albert".into(), albert_address()),
            ("bertha".into(), bertha_address()),
            ("christel".into(), christel_address()),
            ("daewon".into(), daewon_address()),
            ("ester".into(), ester_address()),
            ("masp".into(), namada::types::address::MASP),
        ]
        .into_iter()
        .collect();
        let token_addresses = tokens()
            .into_iter()
            .map(|(addr, alias)| (alias.into(), addr));
        addresses.extend(token_addresses);
        addresses
    }

    /// An established user address for testing & development
    pub fn albert_address() -> Address {
        PREGENESIS_WALLET
            .find_address("albert")
            .expect("Albert's address should be in the pre-genesis wallet")
            .into_owned()
    }

    /// An established user address for testing & development
    pub fn bertha_address() -> Address {
        PREGENESIS_WALLET
            .find_address("bertha")
            .expect("Bertha's address should be in the pre-genesis wallet")
            .into_owned()
    }

    /// An established user address for testing & development
    pub fn christel_address() -> Address {
        PREGENESIS_WALLET
            .find_address("christel")
            .expect("Christel's address should be in the pre-genesis wallet")
            .into_owned()
    }

    /// An implicit user address for testing & development
    pub fn daewon_address() -> Address {
        (&daewon_keypair().ref_to()).into()
    }

    /// An implicit user address for testing & development
    pub fn ester_address() -> Address {
        (&ester_keypair().ref_to()).into()
    }

    /// An established validator address for testing & development
    pub fn validator_address() -> Address {
        PREGENESIS_WALLET
            .find_address("validator-0")
            .expect(
                "The zeroth validator's address should be in the pre-genesis \
                 wallet",
            )
            .into_owned()
    }

    /// Get an unecrypted keypair from the pre-genesis wallet.
    pub fn get_unencrypted_keypair(name: &str) -> common::SecretKey {
        let sk = match PREGENESIS_WALLET.get_secret_keys().get(name).unwrap().0
        {
            namada_sdk::wallet::StoredKeypair::Encrypted(_) => {
                panic!("{name}'s keypair should not be encrypted")
            }
            namada_sdk::wallet::StoredKeypair::Raw(sk) => sk,
        };
        sk.clone()
    }

    /// Get albert's keypair from the pre-genesis wallet.
    pub fn albert_keypair() -> common::SecretKey {
        get_unencrypted_keypair("albert-key")
    }

    /// Get bertha's keypair from the pre-genesis wallet.
    pub fn bertha_keypair() -> common::SecretKey {
        get_unencrypted_keypair("bertha-key")
    }

    /// Get christel's keypair from the pre-genesis wallet.
    pub fn christel_keypair() -> common::SecretKey {
        get_unencrypted_keypair("christel-key")
    }

    /// Get daewon's keypair from the pre-genesis wallet.
    pub fn daewon_keypair() -> common::SecretKey {
        get_unencrypted_keypair("daewon")
    }

    /// Get ester's keypair from the pre-genesis wallet.
    pub fn ester_keypair() -> common::SecretKey {
        get_unencrypted_keypair("ester")
    }

    /// Get the validator consensus keypair from the wallet.
    pub fn validator_keypair() -> common::SecretKey {
        VALIDATOR_WALLET.consensus_key.clone()
    }

    /// Get the validator account keypair from the wallet.
    pub fn validator_account_keypair() -> common::SecretKey {
        get_unencrypted_keypair("validator-0-account-key")
    }

    /// The name of a file that is unique to the project's root directory.
    const PROJECT_ROOT_UNIQUE_FILE: &str = "rust-toolchain.toml";

    /// The pre-genesis directory of `validator-0`.
    const VALIDATOR_0_PREGENESIS_DIR: &str =
        "genesis/localnet/src/pre-genesis/validator-0";

    lazy_static! {
        static ref PREGENESIS_WALLET: Wallet<CliWalletUtils> = {
            let mut root_dir = std::env::current_dir()
                .expect("Current directory should exist")
                .canonicalize()
                .expect("Current directory should exist");
            // Find the project root dir
            while !root_dir.join(PROJECT_ROOT_UNIQUE_FILE).exists() {
                root_dir.pop();
            }
            let path = root_dir.join("genesis/localnet/src/pre-genesis");
            crate::wallet::load(&path).unwrap()
        };

        static ref VALIDATOR_WALLET: ValidatorWallet = {
            let mut root_dir = std::env::current_dir()
                .expect("Current directory should exist")
                .canonicalize()
                .expect("Current directory should exist");
            // Find the project root dir
            while !root_dir.join(PROJECT_ROOT_UNIQUE_FILE).exists() {
                root_dir.pop();
            }
            let path =
                root_dir.join(VALIDATOR_0_PREGENESIS_DIR);
            crate::wallet::pre_genesis::load(&path).unwrap()
        };
    }
}
