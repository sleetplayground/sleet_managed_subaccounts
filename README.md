# Sleet Managed Subaccounts

A NEAR smart contract for creating and managing subaccounts with flexible public key management.

> The contract focuses on subaccount creation and key management. While the original concept included transaction signing capabilities for subaccounts, that functionality is beyond the current scope. The "managed" aspect refers to the owner's ability to configure default public keys for new subaccounts and control who can create them.
> current limitaions include not being able to delpoy conrtact to the new account in the same command, or cleaning up the list of created subaccounts.


## Overview

This contract enables controlled creation and management of NEAR subaccounts with advanced public key management features. It allows approved users to create subaccounts and automatically adds predefined public keys to new subaccounts.

### Use Cases
- Applications that manage subaccounts for users
- Meme token launchpads
- Trading with multiple accounts
- Organizational account management

## Features

- Controlled subaccount creation (owner and approved users only)
- Optional public key specification during subaccount creation
- Automatic addition of predefined public keys to new subaccounts
- Flexible user management for subaccount creation permissions
- Comprehensive key management for default subaccount access

## Building and Testing

```sh
cargo build --target wasm32-unknown-unknown --release
# use build script
./build.sh

# development commands
cargo check
cargo clean

# deploy
near deploy --wasmFile dist/sleet_managed_subaccounts.wasm $CONTRACT_NAME
```

## Contract Methods

### Initialization
- `new(owner_id: AccountId, initial_public_key: Option<PublicKey>)` - Initialize contract with owner and optional default public key

### Subaccount Management
- `sub_create(name: String, public_key: Option<PublicKey>)` - Create a new subaccount with optional specific public key
- `sub_list()` - List all subaccounts created through this contract

### Access Control
- `manage_add_user(account_id: AccountId)` - Add an account to the list of approved subaccount creators
- `manage_remove_user(account_id: AccountId)` - Remove an account from the list of approved creators
- `manage_list_users()` - View all accounts approved for subaccount creation

### Key Management
- `manage_add_key(public_key: PublicKey)` - Add a public key to be included in all new subaccounts
- `manage_remove_key(public_key: PublicKey)` - Remove a public key from the default set
- `manage_list_keys()` - View all public keys that will be added to new subaccounts

## CLI Usage Examples

```bash
# Initialize contract
near call $CONTRACT new '{"owner_id": "owner.near", "initial_public_key": "ed25519:..."}'  --accountId owner.near

# Add approved user
near call $CONTRACT manage_add_user '{"account_id": "approved.near"}' --accountId owner.near

# Add default public key
near call $CONTRACT manage_add_key '{"public_key": "ed25519:..."}' --accountId owner.near

# Create subaccount with default keys
near call $CONTRACT sub_create '{"name": "test"}' --accountId approved.near

# Create subaccount with additional key
near call $CONTRACT sub_create '{"name": "test2", "public_key": "ed25519:..."}' --accountId approved.near

# View all subaccounts
near view $CONTRACT sub_list

# View approved users
near view $CONTRACT manage_list_users

# View default public keys
near view $CONTRACT manage_list_keys

# Note: The subaccount list is maintained automatically by the contract.
# If a subaccount is deleted, it will remain in the list

```

---

Copyright 2025 by sleet.near