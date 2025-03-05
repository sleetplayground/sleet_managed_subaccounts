use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Promise, NearToken, Gas};
use std::str::FromStr;

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

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SubAccountArgs {
    name: String,
    public_key: Option<String>,
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn sub_create(&mut self, name: String, public_key: Option<String>) -> Promise {
        let caller = env::predecessor_account_id();
        let subaccount_id = format!("{name}.{caller}").parse().unwrap();
        
        assert!(self.subaccounts.get(&subaccount_id).is_none(), "Subaccount already exists");
        
        // Store the relationship between subaccount and master
        self.subaccounts.insert(&subaccount_id, &caller);
        
        // Create the subaccount
        let mut promise = Promise::new(subaccount_id.clone())
            .create_account()
            .transfer(env::attached_deposit())
            .add_full_access_key(env::signer_account_pk());

        // Add limited access key if provided
        if let Some(key) = public_key {
            let pk = near_sdk::PublicKey::from_str(&key).unwrap();
            promise = promise.then(
                Promise::new(subaccount_id).add_access_key_allowance(
                    pk,
                    near_sdk::types::Allowance::default(),
                    env::current_account_id(),
                    "sub_action,sub_manage".to_string()
                )
            );
        }

        promise
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
                let pk = near_sdk::PublicKey::from_str(&args.unwrap()[0]).unwrap();
                Promise::new(subaccount).add_access_key_allowance(
                    pk,
                    near_sdk::types::Allowance::default(),
                    env::current_account_id(),
                    "sub_action".to_string()
                )
            }
            "remove_key" => {
                let pk = near_sdk::PublicKey::from_str(&args.unwrap()[0]).unwrap();
                Promise::new(subaccount).delete_key(pk)
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
