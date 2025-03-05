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
- sub_manage, for adding and removing keys, and other things to the subaccount
- sub_action, for doing actions as the subaccount
- sub_list





---

copyright 2025 by sleet.near