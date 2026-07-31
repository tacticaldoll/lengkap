# Worklane Bridge Reference

This document records the pressure that shaped Lengkap without creating a
dependency or committing Worklane to adoption.

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

## Adoption Test

Adoption is warranted only if Worklane can:

1. map its observations without leaking queue vocabulary into Lengkap;
2. preserve existing public lifecycle semantics;
3. persist or reconstruct pending assembly state safely; and
4. remove more lifecycle decision logic than the adapter introduces.

If those conditions do not hold, Lengkap remains a valid independent spike and
Worklane keeps its local mechanism.
