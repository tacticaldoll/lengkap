# lengkap-contract

The isolated core of Lengkap: a zero-dependency, `no_std + alloc`, sans-I/O
mechanism for deciding when every required evidence slot has produced a value,
when evidence is still pending, or when completion has become impossible.

The contract provides:

- fixed ordered slots with monotonic first-value capture;
- deterministic pending, ready, and impossible decisions;
- recovery of accumulated state from impossible decisions;
- atomic recovery of both inputs after structural errors;
- ordered optional-slot transfer for caller-owned checkpoint adapters;
- captured, remaining, and unresolved-slot progress inspection; and
- equivalent free-function, batch-method, and single-finding entrypoints.

It does not serialize or persist the transferred slots. Callers own evidence
truth, encoding, storage, I/O, scheduling, and reactions.

Most applications should depend on the `lengkap` facade. Depend directly on
this crate only when the contract boundary itself is the desired dependency.

This repository is currently pre-release. See the root `README.md` and
`PROJECT.md` for the full contract and maturity status.

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at
your option.
