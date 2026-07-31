# Lengkap

**Tier 2 — release candidate.** Version 0.1.0 defines the initial public
release. The contract is implemented, adversarially tested, and governed, but
Tier 1 graduation still requires adoption of the registry artifact by a real
bridge consumer.

Lengkap ("complete; whole, with nothing missing" — Indonesian) is a
zero-dependency, `no_std + alloc`, sans-I/O core for all-of evidence completion.
A fixed ordered assembly becomes ready only after every slot has produced a
value, remains pending while evidence is absent, and becomes impossible when an
unresolved slot can no longer produce. Impossible and structurally invalid
outcomes return caller-owned inputs for recovery.

Worklane's fan-in lifecycle supplied the pressure that revealed the mechanism.
Lengkap does not depend on Worklane and contains no queue vocabulary: callers
own evidence truth, domain mapping, persistence, scheduling, I/O, and every
reaction to the decision.

## Example

```rust
use lengkap::{Assembly, Decision, Finding, LocatedFinding, Slot};

let decision = Assembly::new(2)
    .adjudicate([
        LocatedFinding::<_, &str>::new(
            Slot::new(1),
            Finding::Produced("second"),
        ),
        LocatedFinding::new(
            Slot::new(0),
            Finding::Produced("first"),
        ),
    ])
    .expect("slots are valid");

assert_eq!(decision, Decision::Ready(vec!["first", "second"]));
```

`Assembly` also exposes captured and remaining progress, stable unresolved-slot
iteration, and owned `into_slots` / `from_slots` transfer. Those slots are an
in-memory checkpoint seam; callers still own encoding, storage, and I/O.
Recovered located findings can be consumed back into their slot and owned
finding, while structural errors provide domain-neutral error messages.

## Workspace

- `crates/lengkap-contract` — the complete zero-dependency mechanism.
- `crates/lengkap` — the recommended, logic-free re-export facade.
- `crates/lengkap-governance` — the unpublished Tianheng constitution and
  reaction proofs.

Run the architecture gate with:

```bash
cargo run -p lengkap-governance -- check --manifest-path Cargo.toml
```

The non-toy
[`worklane_fan_in`](crates/lengkap-contract/examples/worklane_fan_in.rs)
example shows the intended bridge mapping without adding a Worklane dependency.

## Definition Of Done

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo +1.85.0 check -p lengkap-contract -p lengkap --all-targets
cargo +1.88.0 check --workspace --all-targets
cargo deny check
cargo run -p lengkap-governance -- check --manifest-path Cargo.toml
```

The publishable `lengkap-contract` and `lengkap` crates support Rust 1.85.
Repository-only governance tooling uses Rust 1.88 and is checked separately.
The product crates are released together, contract first and facade second;
`lengkap-governance` is never published. See
[`docs/releasing.md`](docs/releasing.md) for the transaction boundary.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or
[MIT](LICENSE-MIT), at your option.
