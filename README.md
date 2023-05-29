# CosmWasm Project

## Project Structure

Contract are partitioned into _query_ and _execute_ functions. _Query_ functions
read contract state; whereas, _execute_ functions may mutate state. These groups
of functions are contained in distinct modules: `src/execute/` and `src/query/`.
If the contract implements any other entrypoint, like `reply`, one can create a
new `reply` module following the established pattern.

## Building, Deploying, Instantiating

```
make build
make schemas
make deploy
make instantiate
```

---

## Local Development

### A. Docker Setup
1. Run local Juno node
```
make devnet
```

### B. Local System Setup

1. Initialize Juno & Import key again
```
junod init --chain-id testing localdev
```

2. Import key
```
junod keys add JunoWallet --recover

> Enter your bip39 mnemonic
clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose
```

Edit `$HOME/.juno/config/client.toml` and set `keyring-backend = "test"` to match the Juno node running in Docker.

### OSX Notes
* `grep` - Need to run `brew install grep`. Follow post-install instructions to add new location to PATH

---

## Execute Functions

### Change Ownership

This lets you change the "owner" address associated with the contract.

## Query Functions

### Select

Return one or more specified properties from state.
