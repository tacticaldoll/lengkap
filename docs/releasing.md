# Releasing Lengkap

Lengkap releases the complete workspace as one verified source state but
publishes only two product crates:

1. `lengkap-contract`
2. `lengkap`

`lengkap-governance` is repository tooling and is never published.

## Preparation

Prepare release content through a pull request whose squash title is
`chore(release): prepare X.Y.Z`. The changelog entry uses the release date and
links to `vX.Y.Z`. Before merging, run every repository gate, inspect both
product package archives, and run a publication dry-run for the contract.

After the squash commit reaches `main`, re-run the complete gates and verify
that the working tree is clean and exactly at that commit.

## Publication

Publish one package at a time:

```bash
cargo publish -p lengkap-contract
cargo publish --dry-run -p lengkap
cargo publish -p lengkap
```

The facade dry-run occurs after `lengkap-contract` is visible in the crates.io
index so that Cargo verifies the registry dependency rather than the workspace
path.

If publication times out or its result is uncertain, query crates.io for the
exact crate and version before retrying. Published versions cannot be
overwritten.

## External Verification

Create a fresh project with no path or patch override, depend on
`lengkap = "X.Y.Z"`, and exercise the facade API on the declared product MSRV.
Do not finalize the release if registry resolution, compilation, or execution
fails.

## Finalization

Only after both crates and the external consumer pass, create the annotated tag
and GitHub release:

```bash
git tag -a vX.Y.Z -m "release: X.Y.Z"
git push origin vX.Y.Z
gh release create vX.Y.Z
```

The tag names the verified release-preparation commit on `main`. No content
commit follows merely to finalize the release.
