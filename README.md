# Sleet Managed Subaccounts

A NEAR smart contract for creating and managing subaccounts with flexible public key management.


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
- Manual subaccount list management by owner
- Protected subaccounts that cannot be removed from the list

## Build and Test

```bash
# CARGO COMMANDS
cargo check
cargo test
cargo clean

# CUSTOM SCRIPTS
./build_cargo.sh
./build_reproducible.sh
./clean.sh # Just cleans the .wasm files and the custom build directories
```

###  How to Deploy?

```bash
# Deploy to testnet
near deploy <your-account>.testnet build_near/sleet_managed_subaccounts.wasm

# For mainnet deployment
near deploy <your-account>.near build_near/sleet_managed_subaccounts.wasm
```

## Contract Methods

### Initialization
- `new(owner_id: AccountId, initial_public_key: Option<PublicKey>)` - Initialize contract with owner and optional default public key

### Subaccount Management
- `sub_create(name: String, public_key: Option<PublicKey>, contract_code: Option<Vec<u8>>)` - Create a new subaccount with optional specific public key and contract deployment
- `sub_list()` - List all subaccounts created through this contract
- `sub_add(account_id: AccountId)` - Add an existing subaccount to the list (owner only)
- `sub_remove(account_id: AccountId)` - Remove a subaccount from the list (owner only)
- `sub_restrict(name: String)` - Add a name to the restricted list (owner only)
- `sub_unrestrict(name: String)` - Remove a name from the restricted list (owner only)
- `sub_list_restricted()` - View all restricted subaccount names

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
near call $CONTRACT new '{"owner_id": "owner.near", "initial_public_key": "ed25519:..."}' --accountId owner.near

# Add approved user
near call $CONTRACT manage_add_user '{"account_id": "approved.near"}' --accountId owner.near

# Add default public key
near call $CONTRACT manage_add_key '{"public_key": "ed25519:..."}' --accountId owner.near

# Create subaccount with default keys
near call $CONTRACT sub_create '{"name": "test"}' --accountId approved.near

# Create subaccount with additional key
near call $CONTRACT sub_create '{"name": "test2", "public_key": "ed25519:..."}' --accountId approved.near

# Create subaccount with contract deployment
near call $CONTRACT sub_create '{"name": "test3", "contract_code": "<base64-encoded-wasm-binary>"}' --accountId approved.near

# Create subaccount with both key and contract
near call $CONTRACT sub_create '{"name": "test4", "public_key": "ed25519:...", "contract_code": "<base64-encoded-wasm-binary>"}' --accountId approved.near

# View all subaccounts
near view $CONTRACT sub_list

# Add name to restricted list
near call $CONTRACT sub_restrict '{"name": "test"}' --accountId owner.near

# Remove name from restricted list
near call $CONTRACT sub_unrestrict '{"name": "test"}' --accountId owner.near

# View restricted names
near view $CONTRACT sub_list_restricted

# View approved users
near view $CONTRACT manage_list_users

# View default public keys
near view $CONTRACT manage_list_keys

# Note: The subaccount list is maintained automatically by the contract.
# Protected subaccounts cannot be removed from the list.
```

---

Copyright 2025 by sleet.near