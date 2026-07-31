# Constitution: lengkap

## Static boundaries

### `lengkap-contract`

> lengkap-contract is the portable no_std mechanism root: its model and adjudication need no dependency, so it must declare none.

- **rule**: restrict dependencies to (only: )
- **kind**: crate · **severity**: enforce

### `lengkap`

> lengkap is the curated public entrypoint: its surface is the complete lengkap-contract re-export, so it may depend only on lengkap-contract.

- **rule**: restrict dependencies to (only: lengkap-contract)
- **kind**: crate · **severity**: enforce

### `lengkap-governance`

> the governance gate stays independent of the workspace it judges: lengkap-governance may depend only on tianheng.

- **rule**: restrict dependencies to (only: tianheng)
- **kind**: crate · **severity**: enforce

### `crate`

> evidence adjudication is an in-memory mechanism: lengkap-contract must not call std::io, std::fs, std::net, or std::process.

- **rule**: inline symbol path confined to module (confined_prefix: std::io)
- **kind**: module · **severity**: enforce · **crate**: lengkap-contract

### `crate`

> evidence adjudication is an in-memory mechanism: lengkap-contract must not call std::io, std::fs, std::net, or std::process.

- **rule**: inline symbol path confined to module (confined_prefix: std::fs)
- **kind**: module · **severity**: enforce · **crate**: lengkap-contract

### `crate`

> evidence adjudication is an in-memory mechanism: lengkap-contract must not call std::io, std::fs, std::net, or std::process.

- **rule**: inline symbol path confined to module (confined_prefix: std::net)
- **kind**: module · **severity**: enforce · **crate**: lengkap-contract

### `crate`

> evidence adjudication is an in-memory mechanism: lengkap-contract must not call std::io, std::fs, std::net, or std::process.

- **rule**: inline symbol path confined to module (confined_prefix: std::process)
- **kind**: module · **severity**: enforce · **crate**: lengkap-contract

### `crate`

> evidence adjudication is deterministic and caller-driven: lengkap-contract reads no ambient clock and exposes no public async API.

- **rule**: inline symbol path confined to module (confined_prefix: std::time; ending_with: now)
- **kind**: module · **severity**: enforce · **crate**: lengkap-contract

## Forbidden-marker boundaries

### `crate`

> lengkap-contract owns transient generic mechanism, not a wire format: Serialize and Deserialize remain the caller's responsibility.

- **rule**: must not acquire trait (forbidden: serde::Serialize, serde::Deserialize)
- **kind**: semantic · **severity**: enforce · **crate**: lengkap-contract

## Async-exposure boundaries

### `crate`

> evidence adjudication is deterministic and caller-driven: lengkap-contract reads no ambient clock and exposes no public async API.

- **rule**: must not expose async fn (including_submodules: true; scan_depth: subtree)
- **kind**: semantic · **severity**: enforce · **crate**: lengkap-contract
