# ADR 0003: Isolate All-Of Evidence Completion

## Status

Accepted

## Context

Worklane fan-in needs a deterministic rule for accumulating ordered results and
recognizing terminal impossibility. That rule was entangled with broker reads,
async polling, storage, and callbacks even though it performs no I/O itself.

## Decision

Create Lengkap as an independent three-crate Rust workspace:

- `lengkap-contract` owns the zero-dependency `no_std + alloc` mechanism;
- `lengkap` is a complete logic-free re-export facade; and
- `lengkap-governance` enforces observable architectural boundaries.

The core uses fixed numeric slots, monotonic first-value capture, deterministic
lowest-slot impossibility, and atomic structural validation. All domain meaning
and effects remain caller obligations.

## Consequences

- The mechanism can be tested and evolved without a queue runtime.
- Worklane can assess adoption through a narrow adapter rather than an inward
  dependency from the core.
- A fixed all-of model deliberately excludes quorum, dynamic membership, and
  contradiction policy.
- The project remains Tier 2 until a real bridge consumer and public release
  jointly prove graduation.
