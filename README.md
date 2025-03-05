# sleet_managed_subaccounts

contract for creating and managing subaccounts.


use cases:
- an app that manages subaccounts for users.
- a meme token launchpad.
- trading with muli-accounts


---

### Building and Testing

```sh
cargo build --target wasm32-unknown-unknown --release
# use build sh
./build.sh

cargo check
cargo clippy
cargo test
cargo clean
```

### Methods and Actions

hello
- get_greeting
- set_greeting
subaccount managment
- sub_create
- sub_manage, for adding and removing keys, and other things to the subaccount like deleteing and depoying contracts
- sub_action, for doing actions as the subaccount
- sub_list, to list all subaccounts

### CLI Usage Examples

```bash
# Deploy the contract
near deploy --accountId YOUR_ACCOUNT.near --wasmFile target/wasm32-unknown-unknown/release/sleet_managed_subaccounts.wasm

# Create a new subaccount
near call YOUR_ACCOUNT.near sub_create '{"name": "myapp"}' --accountId YOUR_ACCOUNT.near

# Add full access key to subaccount
near call YOUR_ACCOUNT.near sub_manage '{
  "sub_account": "app001.YOUR_ACCOUNT.near",
  "action": "AddFullAccessKey",
  "public_key": "ed25519:XXXXX"
}' --accountId YOUR_ACCOUNT.near

# Execute action as subaccount (e.g., transfer NEAR)
near call YOUR_ACCOUNT.near sub_action '{
  "sub_account": "app001.YOUR_ACCOUNT.near",
  "action": {
    "Transfer": {
      "amount": "1000000000000000000000000",
      "receiver_id": "recipient.near"
    }
  }
}' --accountId YOUR_ACCOUNT.near

# List all subaccounts
near view YOUR_ACCOUNT.near sub_list '{"from_index": 0, "limit": 10}'
```

Note: Replace `YOUR_ACCOUNT.near` with your actual account name.

---

copyright 2025 by sleet.near