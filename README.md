# Discombobulator on Solana

**Shuffle state. On-chain.**

Discombobulator is an experimental Solana program that randomizes and re-encodes account data for testing and fuzz workflows. Useful for devnet experiments and ensuring clients handle non-deterministic layouts.

## Goals

- Provide a single instruction that takes an account and "discombobulates" its bytes (permute, xor with nonce, etc.)
- No mainnet deployment; devnet only.
- Anchor-based; minimal dependencies.

## Status

Early WIP. Devnet testing in progress.

## Repo

- `programs/discombobulator` — Anchor program (single `shuffle` instruction stub).
- Devnet-only; no mainnet plans.

## Setup

```bash
anchor build
anchor deploy --provider.cluster devnet
```

## License

MIT
# discombobulator-on-sol
