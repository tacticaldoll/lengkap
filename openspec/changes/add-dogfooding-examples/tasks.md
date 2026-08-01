## 1. `impossible_then_recovered` example

- [x] 1.1 Write `crates/lengkap-contract/examples/impossible_then_recovered.rs`: an unresolved slot receives `Impossible`, the returned assembly still has that slot as `None`, the caller resubmits a `Produced` finding for the same slot, and the final decision is `Ready`. Doc comment attributes the retry decision to caller policy.
- [x] 1.2 Run `cargo run --example impossible_then_recovered -p lengkap-contract` and confirm it exits successfully.

## 2. `readiness_gate` example

- [x] 2.1 Write `crates/lengkap-contract/examples/readiness_gate.rs`: a fixed three-slot startup gate (config loaded, database connected, cache warmed) modeled with locally-defined types, reaching `Ready` once every subsystem reports produced.
- [x] 2.2 Run `cargo run --example readiness_gate -p lengkap-contract` and confirm it exits successfully.

## 3. `resubmit_after_structural_error` example

- [x] 3.1 Write `crates/lengkap-contract/examples/resubmit_after_structural_error.rs`: an out-of-range finding from a simulated caller mapping bug, hitting an assembly with prior captured progress, produces a `SlotOutOfRange` `AdjudicationError`; the caller inspects `kind()`, recovers both the assembly and the batch via `into_parts()`, drops the invalid entry, and a corrected resubmission returns `Ready`.
- [x] 3.2 Run `cargo run --example resubmit_after_structural_error -p lengkap-contract` and confirm it exits successfully.

## 4. Documentation

- [x] 4.1 Add `README.md` references for all three new examples, in the same style as the existing `worklane_fan_in` and `checkpoint_restore` references.

## 5. Verification

- [x] 5.1 Run the complete Definition of Done from the workspace root (`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`, `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`, `cargo +1.85.0 check -p lengkap-contract -p lengkap --all-targets`, `cargo +1.88.0 check --workspace --all-targets`, `cargo semver-checks --package lengkap-contract --baseline-version 0.1.0`, `cargo semver-checks --package lengkap --baseline-version 0.1.0`, `cargo deny check`, `cargo run -p lengkap-governance -- check --manifest-path Cargo.toml`) and confirm every gate passes.
- [x] 5.2 Do not check off any task in sections 1-4 until 5.1 is green.

## 6. Sync and archive

- [ ] 6.1 Sync: promote the `example-documentation` delta spec into `openspec/specs/example-documentation/spec.md`.
- [ ] 6.2 Archive: once verified, remove `openspec/changes/add-dogfooding-examples/` directly; do not run `openspec archive`.
- [ ] 6.3 Update BACKLOG.md with the ✓ shipped status after archiving.
