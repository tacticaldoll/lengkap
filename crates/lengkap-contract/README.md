# lengkap-contract

The isolated core of Lengkap: a zero-dependency, `no_std + alloc`, sans-I/O
mechanism for deciding when every required evidence slot has produced a value,
when evidence is still pending, or when completion has become impossible.

Most applications should depend on the `lengkap` facade. Depend directly on
this crate only when the contract boundary itself is the desired dependency.

This repository is currently pre-release. See the root `README.md` and
`PROJECT.md` for the full contract and maturity status.
