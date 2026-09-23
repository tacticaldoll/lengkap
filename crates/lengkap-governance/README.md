# lengkap-governance

The unpublished Tianheng constitution and reaction proofs for the Lengkap
workspace. It is an independent repository gate, not part of the public product
surface.

The accepted constitution's generated projection is `AGENTS.lengkap-law.md` at
the repository root. Regenerate it after a deliberate, reviewed law change:

```sh
BLESS=1 cargo test -p lengkap-governance law_projection_is_fresh
```

See the
[Lengkap repository](https://github.com/tacticaldoll/lengkap) for the accepted
architecture and local invocation.
