# sleet_managed_subaccounts

contract for creating and managing subaccounts.


use cases:
- an app that manages subaccounts for users.
- a meme token launchpad.
- trading with muli-accounts

> I orginaly ha the idea of a contract that could do and sign trasactions for subaccount, but that is beyond the scope of this project. the managed ascpec of this contract is the fact that the owner can set a public key that will be added to newly created accounts. currenlty only the owner and aprroved users can create subacounts.

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

new
- init, with owner info and public key to use for subaccount creation.
hello
- greeting_get
- greeting_set
subaccount creation
- sub_create
- sub_list, to list all subaccounts
owner management
- manage_add_user, for adding a user that can call the subaccount creation method
- manage_remove_user
-  manage_list_users
- manage_add_key, for ading a key to the list of keys that will be added to subacounts
- manage_remove_key, for removing a key from the list of keys that will be added to subacounts
- manage_list_keys, for listing keys


### CLI Usage Examples



---

copyright 2025 by sleet.near