# Proposal

## Why

Three `architectural-governance` scenarios say the governance check fails on shapes its accepted
boundaries do not observe. The accepted boundary reasons and Tianheng's observation bounds say what
each boundary observes: the no-I/O boundaries see an inline call into a `std::io`, `std::fs`,
`std::net`, or `std::process` path, not I/O through a method on a value or inside a macro body; the
async boundary sees a public `async fn`, not a written `-> impl Future`; the serde boundary sees a
written derive or impl, not one a macro generates or one on a self type it cannot resolve; and the
dependency boundaries read only the normal dependency table. The scenarios still speak of "a
forbidden standard-library I/O path", "an async public function", "derives or implements", and "a
dependency", so they claim more than the gate proves. Two prose sentences outside the specs make
the same kind of claim.

## What Changes

- "The contract performs no I/O": the requirement states which part the governance check observes
  and that the remainder is review-governed; its two failing scenarios name an inline call into
  one of the four paths and a declared public `async fn`.
- "Serialization policy stays outside the contract": the requirement states that a macro-generated
  impl is review-governed; the failing scenario names a derive or impl written in contract source.
- "The governor remains independent": the requirement states that dev and build dependencies are
  review-governed; the failing scenario names a normal dependency.
- `AGENTS.md` Definition Of Done prose and `BACKLOG.md`'s "Governance states only observable
  facts" entry say "normal dependencies" and name inline path calls and inline `std::time` `now`
  calls instead of "direct dependencies", "selected source paths", and "clock reads".

The product intent in each requirement stays as written.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `architectural-governance`: three requirements narrow what their scenarios say the governance
  check catches.

## Impact

- `openspec/specs/architectural-governance/spec.md` after sync.
- `AGENTS.md` and `BACKLOG.md`: one sentence each.
- No code or law change: the constitution, `AGENTS.lengkap-law.md`, and `list --format json` stay
  as they are.
