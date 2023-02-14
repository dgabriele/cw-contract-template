# CosmWasm Project

## Project Structure

Contract are partitioned into _query_ and _execute_ functions. _Query_ functions
read contract state; whereas, _execute_ functions may mutate state. These groups
of functions are contained in distinct modules: `src/execute/` and `src/query/`.
If the contract implements any other entrypoint, like `reply`, one can create a
new `reply` module following the established pattern.

## Building, Deploying, Instantiating

TODO

## Execute Functions

### Change Ownership

This lets you change the "owner" address associated with the contract.

## Query Functions

### Select

Return one or more specified properties from state.
