use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::Base58PublicKey;
use near_sdk::{env, log, near, AccountId, Promise};

#[derive(BorshDeserialize, BorshSerialize)]
#[near(contract_state)]
pub struct Contract {
    // Map of subaccount ID to its master account
    subaccounts: UnorderedMap<AccountId, AccountId>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            subaccounts: UnorderedMap::new(b"s"),
        }
    }
}

#[near]
impl Contract {
    /// Creates a new subaccount with the given name and optional public key
    /// The master account's key will automatically be added to the subaccount
    pub fn sub_create(&mut self, name: String, public_key: Option<Base58PublicKey>) -> Promise {
        let caller = env::predecessor_account_id();
        let subaccount_id = format!("{name}.{caller}").parse().unwrap();
        
        assert!(!self.subaccounts.contains_key(&subaccount_id), "Subaccount already exists");
        
        // Store the relationship between subaccount and master
        self.subaccounts.insert(&subaccount_id, &caller);
        
        // Create the subaccount
        Promise::new(subaccount_id)
            .create_account()
            .transfer(env::attached_deposit())
            .add_full_access_key(env::signer_account_pk())
            .then(if let Some(key) = public_key {
                Promise::new(subaccount_id).add_access_key(
                    key.into(),
                    0, // no allowance
                    env::current_account_id(),
                    "sub_action,sub_manage".to_string(),
                )
            } else {
                Promise::new(subaccount_id)
            })
    }

    /// Manages a subaccount by performing administrative actions
    pub fn sub_manage(
        &mut self,
        subaccount: AccountId,
        action: String,
        args: Option<Vec<String>>,
    ) -> Promise {
        let caller = env::predecessor_account_id();
        assert_eq!(
            self.subaccounts.get(&subaccount).unwrap(),
            caller,
            "Only master account can manage subaccount"
        );

        match action.as_str() {
            "delete" => Promise::new(subaccount).delete_account(caller),
            "add_key" => {
                let key: Base58PublicKey = args.unwrap()[0].parse().unwrap();
                Promise::new(subaccount).add_access_key(
                    key.into(),
                    0,
                    env::current_account_id(),
                    "sub_action".to_string(),
                )
            }
            "remove_key" => {
                let key: Base58PublicKey = args.unwrap()[0].parse().unwrap();
                Promise::new(subaccount).delete_key(key.into())
            }
            _ => env::panic_str("Invalid action"),
        }
    }

    /// Executes actions on behalf of a subaccount
    pub fn sub_action(
        &mut self,
        subaccount: AccountId,
        action: String,
        args: Vec<String>,
    ) -> Promise {
        let caller = env::predecessor_account_id();
        assert_eq!(
            self.subaccounts.get(&subaccount).unwrap(),
            caller,
            "Only master account can perform actions for subaccount"
        );

        match action.as_str() {
            "transfer" => {
                let receiver: AccountId = args[0].parse().unwrap();
                let amount: u128 = args[1].parse().unwrap();
                Promise::new(receiver).transfer(amount)
            }
            "call" => {
                let contract: AccountId = args[0].parse().unwrap();
                let method: String = args[1].parse().unwrap();
                let args: Vec<u8> = args[2].as_bytes().to_vec();
                Promise::new(contract).function_call(
                    method,
                    args,
                    env::attached_deposit(),
                    env::prepaid_gas() / 2,
                )
            }
            _ => env::panic_str("Invalid action"),
        }
    }

    /// Lists all subaccounts for a given master account
    pub fn sub_list(&self, master_account: AccountId) -> Vec<AccountId> {
        self.subaccounts
            .iter()
            .filter(|(_, master)| master == &master_account)
            .map(|(subaccount, _)| subaccount)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    fn get_context() -> VMContextBuilder {
        let mut context = VMContextBuilder::new();
        context.predecessor_account_id("master.near".parse().unwrap());
        context
    }

    #[test]
    fn test_create_subaccount() {
        let mut context = get_context();
        testing_env!(context.build());

        let mut contract = Contract::default();
        let result = contract.sub_create("sub".to_string(), None);
        // Note: In tests, we can't actually create accounts, but we can verify the Promise was created
        assert!(result.promise_indices().len() > 0);
    }
}
