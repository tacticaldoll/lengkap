# Domain Language

The canonical vocabulary of Lengkap. `PROJECT.md` states the contract these terms describe.

## Terms

- **Slot**: a stable zero-based position in the required all-of set.
- **Assembly**: fixed ordered storage for values captured so far.
- **Finding**: caller-supplied evidence that a slot produced a value or became
  impossible.
- **Located finding**: one finding associated with one slot.
- **Decision**: `Pending`, `Ready`, or `Impossible`.
- **Structural error**: malformed adjudication input, distinct from a valid
  domain-level impossible result.
