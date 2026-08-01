## Why

`PROJECT.md` and the `evidence-completion-core` spec make several behavioral
promises in prose that no runnable example currently demonstrates: an
`Impossible` decision does not permanently seal its slot, the API vocabulary
stays domain-neutral outside a queue-shaped consumer, and a caller can recover
and resubmit after a structural error. The two existing examples
(`worklane_fan_in`, `checkpoint_restore`) only exercise the Worklane-shaped
Produced/Impossible/Ready path and checkpoint transfer. Per `PROJECT.md`'s
Change Prioritization item 4 ("Documentation and ergonomics around the
existing contract"), this is a zero-coupling way to turn those prose promises
into executable, README-linked evidence without touching the contract, its
dependencies, or its public API.

## What Changes

- Add `crates/lengkap-contract/examples/impossible_then_recovered.rs`:
  demonstrates that a slot reported `Impossible` remains unresolved (`None`)
  in the returned assembly, so a caller that judges the cause retryable can
  resubmit a `Produced` finding for the same slot and still reach `Ready`.
- Add `crates/lengkap-contract/examples/readiness_gate.rs`: re-derives the
  same Pending/Ready/Impossible flow using an application-startup readiness
  domain (config loaded, database connected, cache warmed) instead of
  queue/job vocabulary, to keep the `Assembly`/`Slot`/`Finding` naming
  demonstrably domain-neutral.
- Add `crates/lengkap-contract/examples/resubmit_after_structural_error.rs`:
  demonstrates recovering from an `AdjudicationError` (an out-of-range finding
  caused by a caller-side mapping bug, hitting an assembly that already has
  prior captured progress), inspecting `kind()`, recovering both the
  unchanged assembly and the finding batch with `into_parts()`, dropping the
  bad entry, and resubmitting a corrected batch that reaches `Ready`.
- Reference all three new examples from `README.md`, alongside the two
  existing ones, in the same style.
- No changes to `Cargo.toml`, crate dependencies, `lengkap-contract`/`lengkap`
  public API, or `lengkap-governance` boundaries.

## Capabilities

### New Capabilities
- `example-documentation`: requirements for the runnable, dependency-free
  examples under `crates/lengkap-contract/examples/` that demonstrate
  documented contract behaviors and are referenced from `README.md`.

### Modified Capabilities
<!-- none: no existing capability's requirements change -->

## Impact

- Adds three files under `crates/lengkap-contract/examples/`.
- Edits `README.md` to reference the new examples.
- No dependency, API, or governance-boundary changes; `cargo run --example
  <name> -p lengkap-contract` becomes the verification path for each.
