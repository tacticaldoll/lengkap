# lengkap-contract

The isolated core of Lengkap: a zero-dependency, `no_std + alloc`, sans-I/O
mechanism for deciding when every required evidence slot has produced a value,
when evidence is still pending, or when completion has become impossible.

The contract provides:

- fixed ordered slots with monotonic first-value capture;
- deterministic pending, ready, and impossible decisions;
- recovery of accumulated state from impossible decisions;
- atomic recovery of both inputs after structural errors, including consuming
  located findings back into owned payloads;
- ordered optional-slot transfer for caller-owned checkpoint adapters;
- captured, remaining, and unresolved-slot progress inspection;
- equivalent free-function, batch-method, and single-finding entrypoints; and
- domain-neutral structural error formatting and standard error integration.

It does not serialize or persist the transferred slots. Callers own evidence
truth, encoding, storage, I/O, scheduling, and reactions.

Most applications should depend on the `lengkap` facade. Depend directly on
this crate only when the contract boundary itself is the desired dependency.

Version 0.1.0 defines the initial public release. Lengkap is Tier 1 after
Worklane adopted the registry facade without moving its I/O or domain policy
into the contract. See the root `README.md` and `PROJECT.md` in the
[Lengkap repository](https://github.com/tacticaldoll/lengkap) for the full
contract and maturity status.

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at
your option.
