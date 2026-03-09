# Paruman Smart Contracts

ink! WASM contracts for MandalaChain (Polkadot parachain Para ID 4818).

## Contracts

- **kawenang** — Governance standing (non-transferable, seniority-weighted)
- **musyawarah** — Deliberation and outcome recording (two-track: Pasangkepan Rutin, Paruman Agung)
- **awig-awig** — Per-banjar customary law configuration

## Development

### Prerequisites

```bash
cargo install cargo-contract
```

### Build

```bash
cd contracts/kawenang
cargo contract build
```

### Test

```bash
cargo test
```

### Deploy to Paseo Testnet

```bash
cargo contract upload --suri //Alice
```

## References

- [MandalaChain](https://mandalachain.io)
- [ink! Documentation](https://docs.rs/ink/)
- [Polkadot Parachain Development](https://docs.substrate.io/)
