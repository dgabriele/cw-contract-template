#!/bin/bash

CMD=$1
NETWORK=$2
CONTRACT_ADDR=$(cat $3)
NODE=
CHAIN_ID=
FLAGS=

shift 3

case $NETWORK in
  testnet)
    NODE="https://rpc.uni.juno.deuslabs.fi:443"
    CHAIN_ID=uni-3
    DENOM=ujunox
    ;;
  mainnet)
    NODE="https://rpc-juno.itastakers.com",
    CHAIN_ID=juno-1
    DENOM=ujuno
    ;;
  devnet)
    NODE="http://localhost:26657"
    CHAIN_ID=testing
    DENOM=ujunox
    ;;
esac


do_something() {
  sender=$1
  value=$2
  msg='{"do_something":{"value":"'$value'"}}'
  flags="\
  --node $NODE \
  --gas-prices 0.025$DENOM \
  --chain-id $CHAIN_ID \
  --from $sender \
  --gas auto \
  --gas-adjustment 1.5 \
  --broadcast-mode block \
  --output json \
  -y \
  "
  echo junod tx wasm execute $CONTRACT_ADDR "$msg" "$flags"
  response=$(junod tx wasm execute "$CONTRACT_ADDR" "$msg" $flags)
  echo $response | ./bin/utils/base64-decode-attributes | jq
}


get_something() {
  query='{"get_something":{}}'
  flags="--chain-id $CHAIN_ID --output json --node $NODE"
  echo junod query wasm contract-state smart $CONTRACT_ADDR "$query" $flags
  response=$(junod query wasm contract-state smart $CONTRACT_ADDR "$query" $flags)
  echo $response | ./bin/utils/base64-decode-attributes | jq
}

set -e

echo $*
case $CMD in
  do-something)
    do_something $1 $2
    ;;
  get-something) 
    get_something
    ;;
esac