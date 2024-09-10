# Sink

A contract that burns tokens (forked from
https://github.com/noislabs/nois-contracts).

## Production build

This is a regular CosmWasm workspace. Use the latest version of
[cosmwasm/optimizer](https://github.com/CosmWasm/optimizer) to build it.

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.16.0
```
