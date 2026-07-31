# lengkap

The curated entrypoint for Lengkap, a sans-I/O all-of evidence completion core.
This crate adds no logic: it re-exports the complete `lengkap-contract` API.
That includes recoverable decisions and errors, caller-owned slot transfer,
progress inspection, and method-style adjudication.

Version 0.1.1 defines the current compatible maintenance release, and version
0.1.0 established the initial public release. Lengkap is Tier 1 after Worklane
adopted this registry facade without moving its I/O or domain policy into the
contract. See the root `README.md` and `PROJECT.md` in the
[Lengkap repository](https://github.com/tacticaldoll/lengkap) for the full
contract and maturity status.

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at
your option.
