use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupSet, UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise, PublicKey};

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
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, initial_public_key: Option<PublicKey>) -> Self {
        let mut contract = Self {
            owner_id,
            approved_users: UnorderedSet::new(b"a"),
            default_public_keys: UnorderedSet::new(b"k"),
            created_subaccounts: UnorderedSet::new(b"s"),
        };

        // If an initial public key is provided, add it to the default keys
        if let Some(key) = initial_public_key {
            contract.default_public_keys.insert(&key);
        }

        contract
    }

    /// Create a new subaccount with optional specific public key
    #[payable]
    pub fn sub_create(&mut self, name: String, public_key: Option<PublicKey>) -> Promise {
        // Ensure caller is owner or approved user
        assert!(
            env::predecessor_account_id() == self.owner_id
                || self.approved_users.contains(&env::predecessor_account_id()),
            "Only owner or approved users can create subaccounts"
        );

        // Validate subaccount name
        assert!(!name.contains("."), "Subaccount name cannot contain dots");

        // Construct the full subaccount name
        let subaccount_id = format!("{}.{}", name, env::current_account_id());
        let subaccount = AccountId::new_unchecked(subaccount_id.clone());

        // Store the created subaccount
        self.created_subaccounts.insert(&subaccount);

        // Create the new account promise
        let mut promise = Promise::new(subaccount)
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

        promise
    }

    /// List all subaccounts created through this contract
    pub fn sub_list(&self) -> Vec<AccountId> {
        self.created_subaccounts.to_vec()
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        VMContextBuilder::new()
            .predecessor_account_id(predecessor_account_id)
            .build()
    }

    #[test]
    fn test_new() {
        let context = get_context(AccountId::new_unchecked("owner.near".to_string()));
        testing_env!(context);

        let contract = Contract::new(
            AccountId::new_unchecked("owner.near".to_string()),
            None,
        );
        assert_eq!(contract.owner_id.to_string(), "owner.near");
        assert_eq!(contract.manage_list_users().len(), 0);
        assert_eq!(contract.manage_list_keys().len(), 0);
    }

    #[test]
    fn test_manage_users() {
        let context = get_context(AccountId::new_unchecked("owner.near".to_string()));
        testing_env!(context);

        let mut contract = Contract::new(
            AccountId::new_unchecked("owner.near".to_string()),
            None,
        );

        contract.manage_add_user(AccountId::new_unchecked("user1.near".to_string()));
        assert_eq!(contract.manage_list_users().len(), 1);

        contract.manage_remove_user(AccountId::new_unchecked("user1.near".to_string()));
        assert_eq!(contract.manage_list_users().len(), 0);
    }
}
