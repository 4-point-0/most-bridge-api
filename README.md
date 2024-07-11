# MOST BRIDGE API

This repo serves as a backend service for most-bridge

- `/query-events` route uses `tx_digest` as parameter for calls to `suix_queryEvents` method on SUI RPC
- `/tx_digest` route uses `recipient` and `amount` for parameters calls to `unsafe_transferSui` method on SUI RPC
- `/execute-tx-block` route uses `tx_bytes` and `signature` for parameters calls to `sui_executeTransactionBlock` method on SUI RPC

# Run app

- Install shuttle cli: https://docs.shuttle.rs/getting-started/installation

- to run app use

```bash
cargo shuttle run
```
