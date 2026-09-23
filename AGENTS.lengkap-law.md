# Lengkap Tianheng Law Projection

This file is generated from `constitution()` in `crates/lengkap-governance/src/main.rs`.
The Rust declaration is authoritative; do not edit the projection by hand.
Regenerate it with `BLESS=1 cargo test -p lengkap-governance law_projection_is_fresh`.

# Constitution: lengkap

## Static boundaries

### `lengkap-contract` (crate)

> lengkap-contract is the portable no_std mechanism root: its model and adjudication need no dependency, so it declares no normal dependency.

- **rule**: restrict dependencies to (only: )
- **kind**: crate · **severity**: enforce

### `lengkap` (crate)

> lengkap is the curated public entrypoint: its surface is the complete lengkap-contract re-export, so its normal dependencies are limited to lengkap-contract.

- **rule**: restrict dependencies to (only: lengkap-contract)
- **kind**: crate · **severity**: enforce

### `lengkap-governance` (crate)

> the governance gate stays independent of the workspace it judges: lengkap-governance's normal dependencies are limited to tianheng.

- **rule**: restrict dependencies to (only: tianheng)
- **kind**: crate · **severity**: enforce

### `lengkap-contract::crate` (module)

> evidence adjudication is an in-memory mechanism: lengkap-contract makes no inline call into a std::io, std::fs, std::net, or std::process path. Coverage is partial by nature (a call through a method on a value, such as `write_all` on a writer, or macro-expanded I/O is invisible to a source scan).

- **rule**: inline symbol path confined to module (confined_prefix: std::io)
- **kind**: module · **severity**: enforce · **crate**: lengkap-contract

### `lengkap-contract::crate` (module)

> evidence adjudication is an in-memory mechanism: lengkap-contract makes no inline call into a std::io, std::fs, std::net, or std::process path. Coverage is partial by nature (a call through a method on a value, such as `write_all` on a writer, or macro-expanded I/O is invisible to a source scan).

- **rule**: inline symbol path confined to module (confined_prefix: std::fs)
- **kind**: module · **severity**: enforce · **crate**: lengkap-contract

### `lengkap-contract::crate` (module)

> evidence adjudication is an in-memory mechanism: lengkap-contract makes no inline call into a std::io, std::fs, std::net, or std::process path. Coverage is partial by nature (a call through a method on a value, such as `write_all` on a writer, or macro-expanded I/O is invisible to a source scan).

- **rule**: inline symbol path confined to module (confined_prefix: std::net)
- **kind**: module · **severity**: enforce · **crate**: lengkap-contract

### `lengkap-contract::crate` (module)

> evidence adjudication is an in-memory mechanism: lengkap-contract makes no inline call into a std::io, std::fs, std::net, or std::process path. Coverage is partial by nature (a call through a method on a value, such as `write_all` on a writer, or macro-expanded I/O is invisible to a source scan).

- **rule**: inline symbol path confined to module (confined_prefix: std::process)
- **kind**: module · **severity**: enforce · **crate**: lengkap-contract

### `lengkap-contract::crate` (module)

> evidence adjudication is deterministic and caller-driven: lengkap-contract makes no inline `std::time` `now` call and declares no public async fn anywhere in its module tree. Coverage is partial by nature (a clock read through a method on a value, such as `Instant::elapsed`, is invisible to a source scan, and a written `-> impl Future` is not an async fn).

- **rule**: inline symbol path confined to module (confined_prefix: std::time; ending_with: now)
- **kind**: module · **severity**: enforce · **crate**: lengkap-contract

## Forbidden-marker boundaries

### `lengkap-contract::crate` (semantic)

> lengkap-contract owns transient generic mechanism, not a wire format: Serialize and Deserialize remain the caller's responsibility.

- **rule**: must not acquire trait (forbidden: serde::Serialize, serde::Deserialize)
- **kind**: semantic · **severity**: enforce · **crate**: lengkap-contract

## Async-exposure boundaries

### `lengkap-contract::crate` (semantic)

> evidence adjudication is deterministic and caller-driven: lengkap-contract makes no inline `std::time` `now` call and declares no public async fn anywhere in its module tree. Coverage is partial by nature (a clock read through a method on a value, such as `Instant::elapsed`, is invisible to a source scan, and a written `-> impl Future` is not an async fn).

- **rule**: must not expose async fn (including_submodules: true; scan_depth: subtree)
- **kind**: semantic · **severity**: enforce · **crate**: lengkap-contract
