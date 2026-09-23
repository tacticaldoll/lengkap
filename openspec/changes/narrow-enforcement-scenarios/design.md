# Design

## Context

The accepted boundary reasons in `AGENTS.lengkap-law.md` state what each Tianheng boundary
observes. The earlier reason-correction review probed them against Tianheng 0.6.1: a dev dependency
of the contract and a build dependency of the governor leave the dependency boundaries clean,
`w.write_all(..)` leaves the no-I/O boundaries clean while `std::fs::read(..)` fails them, and a
written `-> impl Future` leaves the async boundary clean while a nested `pub async fn` fails it.
The Tianheng observation bounds also state that the forbidden-marker rule does not observe a
macro-generated impl or a hand impl whose self type it cannot resolve.

## Goals / Non-Goals

**Goals:**

- Every scenario whose outcome is a failing governance check describes only a shape the boundary
  observes.
- The unobserved remainder is named as review-governed rather than dropped.

**Non-Goals:**

- Changing any boundary, reason, or reaction test.
- Narrowing product intent. The requirement statements (`SHALL NOT use source paths under …`,
  `SHALL NOT derive or implement …`, `SHALL depend only on Tianheng`) and `PROJECT.md` stay as
  written, because judgment may be broader than its tooth.
- Adding a scenario for the ambient-clock boundary, which the requirement already names as intent.

## Decisions

- **Name the remainder in the requirement body, not in new scenarios.** One sentence per
  requirement says what the check observes and what review holds. Alternative considered: a
  "not observed" scenario per bound. Rejected, since the scenarios would assert a non-reaction
  that no repository test pins.
- **Use the reasons' wording.** "Inline call", "public `async fn`", and "normal dependency" match
  the accepted reasons, so prose and law read the same.

## Risks / Trade-offs

- [A future Tianheng may observe more] -> The scenarios then still hold; the review-governed
  sentence can narrow in a later change.
