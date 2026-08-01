# Lengkap

**Tier 1 — consumer-proven.** Version 0.1.1 defines the current compatible
maintenance release, and version 0.1.0 established the initial public release.
Worklane consumes the crates.io facade while retaining broker access,
checkpoint persistence, polling, and reactions in its own adapter.

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
example shows the adopted bridge mapping without adding a Worklane dependency.
The domain-neutral
[`checkpoint_restore`](crates/lengkap-contract/examples/checkpoint_restore.rs)
example shows owned partial-state transfer without prescribing encoding,
storage, or I/O.

The
[`impossible_then_recovered`](crates/lengkap-contract/examples/impossible_then_recovered.rs)
example shows that an `Impossible` slot is not sealed by the core: a caller
may resubmit a produced finding for the same slot and still reach `Ready`.
The
[`readiness_gate`](crates/lengkap-contract/examples/readiness_gate.rs)
example re-derives the same fixed all-of shape from an application-startup
domain instead of a queue, showing that `Assembly`, `Slot`, and `Finding`
need no adaptation to fit either vocabulary. The
[`resubmit_after_structural_error`](crates/lengkap-contract/examples/resubmit_after_structural_error.rs)
example shows recovering both a partially captured assembly and its finding
batch after a caller mapping bug, then completing adjudication with a
corrected resubmission.

## Definition Of Done

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo +1.85.0 check -p lengkap-contract -p lengkap --all-targets
cargo +1.88.0 check --workspace --all-targets
cargo semver-checks --package lengkap-contract --baseline-version 0.1.0
cargo semver-checks --package lengkap --baseline-version 0.1.0
cargo deny check
cargo run -p lengkap-governance -- check --manifest-path Cargo.toml
```

The publishable `lengkap-contract` and `lengkap` crates support Rust 1.85.
Repository-only governance tooling uses Rust 1.88 and is checked separately.
Public API compatibility is compared with each crate's exact crates.io 0.1.0
baseline. The product crates are released together, contract first and facade
second;
`lengkap-governance` is never published. See
[`docs/releasing.md`](docs/releasing.md) for the transaction boundary.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or
[MIT](LICENSE-MIT), at your option.
