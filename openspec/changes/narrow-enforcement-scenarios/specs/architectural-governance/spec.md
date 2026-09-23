# Spec Delta

## MODIFIED Requirements

### Requirement: The contract performs no I/O

`lengkap-contract` SHALL NOT use source paths under `std::io`, `std::fs`,
`std::net`, or `std::process`, and SHALL NOT introduce public async or direct
wall-clock observation. The governance check observes an inline call into one
of those paths, an inline `std::time` call ending in `now`, and a declared
public `async fn`; I/O reached through a method on a value or inside a macro
body, a clock read through a method on a value (such as `Instant::elapsed`),
and a written `-> impl Future` are review-governed.

#### Scenario: An I/O path is introduced

- **WHEN** contract source makes an inline call into a `std::io`, `std::fs`,
  `std::net`, or `std::process` path
- **THEN** the governance check fails with the source location and reason

#### Scenario: Public async is introduced

- **WHEN** contract source declares a public `async fn`
- **THEN** the governance check fails with the source location and reason

#### Scenario: The pure contract is checked

- **WHEN** contract source contains only deterministic in-memory adjudication
- **THEN** every sans-I/O boundary passes

### Requirement: Serialization policy stays outside the contract

`lengkap-contract` SHALL NOT derive or implement `Serialize` or `Deserialize`.
The governance check observes a derive or impl written in contract source; an
impl generated inside a macro, or one whose self type the scan cannot resolve
(such as a glob-imported type), is review-governed.

#### Scenario: A serialization marker is introduced

- **WHEN** contract source contains a written derive or impl of `Serialize` or
  `Deserialize`
- **THEN** the governance check fails with the source location and reason

#### Scenario: Domain values remain opaque

- **WHEN** the contract stores generic values without serialization markers
- **THEN** the serialization boundary passes

### Requirement: The governor remains independent

`lengkap-governance` SHALL depend only on Tianheng and SHALL NOT depend on any
workspace crate it judges. The governance check observes the normal dependency
table; a dev or build dependency is review-governed.

#### Scenario: The governor depends on a judged crate

- **WHEN** the governor manifest declares a normal dependency on the contract or
  facade
- **THEN** the governance check fails with the boundary name and reason

#### Scenario: The governor depends only on Tianheng

- **WHEN** the governor manifest has Tianheng as its sole normal dependency
- **THEN** the governor-independence boundary passes
