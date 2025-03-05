# sleet_managed_subaccounts

contract for creating and managing subaccounts.


use cases:
- an app that manages subaccounts for users.
- a meme token launchpad.


---

### Building and Testing

```sh
cargo near build
cargo near build non-reproducible-wasm
cargo near deploy build-reproducible-wasm


cargo check
cargo clippy
cargo test
cargo clean

# deploy
cargo near deploy build-reproducible-wasm <account-id>
```



---



copyright 2025 by sleet.near