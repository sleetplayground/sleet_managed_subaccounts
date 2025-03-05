use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Promise, NearToken, Gas, PublicKey};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    subaccounts: UnorderedMap<AccountId, AccountId>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            subaccounts: UnorderedMap::new(b"s"),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn sub_create(&mut self, name: String, public_key: Option<PublicKey>) -> Promise {
        let caller = env::predecessor_account_id();
        let subaccount_id = format!("{name}.{caller}").parse().unwrap();
        
        assert!(!self.subaccounts.get(&subaccount_id).is_some(), "Subaccount already exists");
        
        self.subaccounts.insert(&subaccount_id, &caller);
        
        Promise::new(subaccount_id.clone())
            .create_account()
            .transfer(env::attached_deposit())
            .add_full_access_key(env::signer_account_pk())
            .then(if let Some(key) = public_key {
                Promise::new(subaccount_id).add_access_key(
                    key,
                    NearToken::from_near(0),
                    env::current_account_id(),
                    "sub_action,sub_manage".to_string(),
                )
            } else {
                Promise::new(subaccount_id)
            })
    }

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
                let key: PublicKey = args.unwrap()[0].parse().unwrap();
                Promise::new(subaccount).add_access_key(
                    key,
                    NearToken::from_near(0),
                    env::current_account_id(),
                    "sub_action".to_string(),
                )
            }
            "remove_key" => {
                let key: PublicKey = args.unwrap()[0].parse().unwrap();
                Promise::new(subaccount).delete_key(key)
            }
            _ => env::panic_str("Invalid action"),
        }
    }

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
                let amount = NearToken::from_yoctonear(args[1].parse::<u128>().unwrap());
                Promise::new(receiver).transfer(amount)
            }
            "deploy" => {
                let code = env::input().expect("No input");
                Promise::new(subaccount).deploy_contract(code)
            }
            "call" => {
                let contract: AccountId = args[0].parse().unwrap();
                let method: String = args[1].parse().unwrap();
                let args_bytes = args[2].as_bytes().to_vec();
                let deposit = NearToken::from_yoctonear(args[3].parse::<u128>().unwrap());
                let gas: Gas = Gas::from_tgas(args[4].parse().unwrap());

                Promise::new(contract).function_call(
                    method,
                    args_bytes,
                    deposit,
                    gas,
                )
            }
            _ => env::panic_str("Invalid action"),
        }
    }

    pub fn sub_list(&self, from_index: u64, limit: u64) -> Vec<(AccountId, AccountId)> {
        self.subaccounts
            .iter()
            .skip(from_index as usize)
            .take(limit as usize)
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
