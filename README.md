# CosmWasm Project

## Building, Deploying, Instantiating

```
make build
make deploy
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
