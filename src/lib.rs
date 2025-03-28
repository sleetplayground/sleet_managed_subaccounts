use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise, PublicKey};
use std::str::FromStr;










#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// The account ID of the contract owner
    owner_id: AccountId,
    /// Set of approved users who can create subaccounts
    approved_users: UnorderedSet<AccountId>,
    /// Set of public keys to be added to all new subaccounts
    default_public_keys: UnorderedSet<PublicKey>,
    /// Set of created subaccounts
    created_subaccounts: UnorderedSet<AccountId>,
    /// Set of restricted subaccount names that cannot be created
    restricted_names: UnorderedSet<String>,
}















#[near_bindgen]
impl Contract
 // opening bracket
{







    /// 👋 init
    #[init]
    pub fn new(owner_id: AccountId, initial_public_key: Option<PublicKey>) -> Self {
        let mut contract = Self {
            owner_id,
            approved_users: UnorderedSet::new(b"a"),
            default_public_keys: UnorderedSet::new(b"k"),
            created_subaccounts: UnorderedSet::new(b"s"),
            restricted_names: UnorderedSet::new(b"r"),
        };

        if let Some(key) = initial_public_key {
            contract.default_public_keys.insert(&key);
        }

        contract
    }















    /// 🚊 sub
    /// Create a new subaccount with optional specific public key and contract deployment
    #[payable]
    pub fn sub_create(
        &mut self,
        name: String,
        public_key: Option<PublicKey>,
        contract_code: Option<Vec<u8>>
    ) -> Promise {
        // Ensure caller is owner or approved user
        assert!(
            env::predecessor_account_id() == self.owner_id
                || self.approved_users.contains(&env::predecessor_account_id()),
            "Only owner or approved users can create subaccounts"
        );

        // Validate subaccount name
        assert!(!name.contains("."), "Subaccount name cannot contain dots");
        assert!(!self.restricted_names.contains(&name), "This subaccount name is restricted");

        // Construct the full subaccount name
        let subaccount_id = format!("{}.{}", name, env::current_account_id());
        let subaccount = AccountId::from_str(&subaccount_id).unwrap();

        // Store the created subaccount
        self.created_subaccounts.insert(&subaccount);

        // Create the new account promise
        let mut promise = Promise::new(subaccount.clone())
            .create_account()
            .transfer(env::attached_deposit());

        // Add the provided public key if any
        if let Some(key) = public_key {
            promise = promise.add_full_access_key(key);
        }

        // Add all default public keys
        for key in self.default_public_keys.iter() {
            promise = promise.add_full_access_key(key);
        }

        // Deploy contract if provided
        if let Some(code) = contract_code {
            promise = promise.deploy_contract(code);
        }

        promise
    }

    /// List all subaccounts created through this contract
    pub fn sub_list(&self) -> Vec<AccountId> {
        self.created_subaccounts.to_vec()
    }

    /// Remove a subaccount from the created subaccounts list (owner only)
    pub fn sub_remove(&mut self, account_id: AccountId) {
        self.assert_owner();
        self.created_subaccounts.remove(&account_id);
    }

    /// Add a subaccount to the created subaccounts list (owner only)
    pub fn sub_add(&mut self, account_id: AccountId) {
        self.assert_owner();
        self.created_subaccounts.insert(&account_id);
    }

    /// Add a name to the restricted list (owner only)
    pub fn sub_restrict(&mut self, name: String) {
        self.assert_owner();
        self.restricted_names.insert(&name);
    }

    /// Remove a name from the restricted list (owner only)
    pub fn sub_unrestrict(&mut self, name: String) {
        self.assert_owner();
        self.restricted_names.remove(&name);
    }

    /// List all restricted subaccount names
    pub fn sub_list_restricted(&self) -> Vec<String> {
        self.restricted_names.to_vec()
    }

    /// Add an account to the list of approved subaccount creators
    pub fn manage_add_user(&mut self, account_id: AccountId) {
        self.assert_owner();
        self.approved_users.insert(&account_id);
    }

    /// Remove an account from the list of approved creators
    pub fn manage_remove_user(&mut self, account_id: AccountId) {
        self.assert_owner();
        self.approved_users.remove(&account_id);
    }

    /// View all accounts approved for subaccount creation
    pub fn manage_list_users(&self) -> Vec<AccountId> {
        self.approved_users.to_vec()
    }

    /// Add a public key to be included in all new subaccounts
    pub fn manage_add_key(&mut self, public_key: PublicKey) {
        self.assert_owner();
        self.default_public_keys.insert(&public_key);
    }

    /// Remove a public key from the default set
    pub fn manage_remove_key(&mut self, public_key: PublicKey) {
        self.assert_owner();
        self.default_public_keys.remove(&public_key);
    }

    /// View all public keys that will be added to new subaccounts
    pub fn manage_list_keys(&self) -> Vec<PublicKey> {
        self.default_public_keys.to_vec()
    }

    /// Helper method to check if caller is owner
    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only owner can call this method"
        );
    }














    

} // closing bracket