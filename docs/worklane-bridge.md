# Worklane Bridge Reference

This document records both the pressure that shaped Lengkap and the completed
Worklane adoption without creating an inward dependency.

## Mapping

| Worklane observation | Lengkap input |
|---|---|
| Job is live | No finding; the slot remains unresolved |
| Completed or unknown, result present | `Finding::Produced(result_bytes)` |
| Completed or unknown, result absent | `Finding::Impossible(MissingResult)` |
| Dead-lettered | `Finding::Impossible(DeadLettered)` |

The adapter owns the required job order and maps each job to its `Slot`.
Worklane must persist the returned pending `Assembly` or reconstruct equivalent
state under its own lifecycle rules.

## Boundary

Lengkap does not call a broker, poll, deserialize job results, schedule another
check, emit callbacks, or decide whether a broker observation is trustworthy.
The adapter must reject an empty fan-in if Worklane's contract requires at least
one dependency; Lengkap correctly treats an empty all-of set as ready.

The executable example at
`crates/lengkap-contract/examples/worklane_fan_in.rs` compiles this mapping
without importing Worklane.

## Adoption Result

Worklane adopted `lengkap = "0.1.0"` from crates.io after satisfying the
original test:

1. its manifest declares the normal registry dependency and its lockfile
   records registry sources and checksums for both product crates;
2. its adapter maps observations without leaking queue vocabulary into
   Lengkap;
3. existing public lifecycle semantics remain owned and tested in Worklane;
4. pending assemblies cross Worklane's own checkpoint adapter as ordered
   optional slots; and
5. broker reads, async polling, persistence, and reactions remain outside
   Lengkap.

The adoption required no Lengkap dependency, public API, completion mode, or
user-obligation change. It therefore completes the Tier 1 graduation condition
without turning the originating Worklane pressure into product authority over
the core.
