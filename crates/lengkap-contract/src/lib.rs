//! A `no_std`, sans-I/O all-of evidence completion core.
//!
//! [`Assembly`] owns a fixed ordered set of slots. [`adjudicate`] captures
//! [`Finding::Produced`] values monotonically, returns [`Decision::Pending`]
//! while unresolved slots remain, and returns [`Decision::Impossible`] when an
//! unresolved slot can no longer produce a value. The core does not obtain,
//! verify, persist, or react to evidence.

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

/// The stable zero-based position of one required value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slot(usize);

impl Slot {
    /// Construct a slot from its zero-based index.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Return this slot's zero-based index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

/// Fixed ordered storage for values captured so far.
#[derive(Debug, PartialEq, Eq)]
pub struct Assembly<Value> {
    values: Vec<Option<Value>>,
}

impl<Value> Assembly<Value> {
    /// Construct an unresolved assembly with `slot_count` fixed slots.
    #[must_use]
    pub fn new(slot_count: usize) -> Self {
        Self {
            values: (0..slot_count).map(|_| None).collect(),
        }
    }

    /// Return the fixed number of slots in this assembly.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Return whether this assembly has no required slots.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Borrow the captured value at `slot`, or `None` if it is unresolved or
    /// outside this assembly.
    #[must_use]
    pub fn value(&self, slot: Slot) -> Option<&Value> {
        self.values.get(slot.index()).and_then(Option::as_ref)
    }

    /// Return whether every slot has captured a value.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.values.iter().all(Option::is_some)
    }
}

/// Domain-supplied evidence about one unresolved slot.
#[derive(Debug, PartialEq, Eq)]
pub enum Finding<Value, Cause> {
    /// This slot produced its value.
    Produced(Value),
    /// This slot can no longer produce a value.
    Impossible(Cause),
}

/// A finding associated with its stable slot.
#[derive(Debug, PartialEq, Eq)]
pub struct LocatedFinding<Value, Cause> {
    slot: Slot,
    finding: Finding<Value, Cause>,
}

impl<Value, Cause> LocatedFinding<Value, Cause> {
    /// Associate `finding` with `slot`.
    #[must_use]
    pub const fn new(slot: Slot, finding: Finding<Value, Cause>) -> Self {
        Self { slot, finding }
    }

    /// Return the slot addressed by this finding.
    #[must_use]
    pub const fn slot(&self) -> Slot {
        self.slot
    }

    /// Borrow the domain-supplied finding.
    #[must_use]
    pub const fn finding(&self) -> &Finding<Value, Cause> {
        &self.finding
    }
}

/// The semantic result of structurally valid adjudication.
#[derive(Debug, PartialEq, Eq)]
pub enum Decision<Value, Cause> {
    /// At least one required slot remains unresolved.
    Pending(Assembly<Value>),
    /// Every required slot produced a value, returned in slot order.
    Ready(Vec<Value>),
    /// An unresolved slot can no longer produce a value.
    Impossible {
        /// The lowest unresolved slot reported as impossible.
        slot: Slot,
        /// The caller-supplied reason completion became impossible.
        cause: Cause,
    },
}

/// Why a set of located findings is structurally invalid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StructuralError {
    /// A finding addressed a slot outside the fixed assembly.
    SlotOutOfRange {
        /// The invalid slot.
        slot: Slot,
        /// The number of slots available.
        slot_count: usize,
    },
    /// More than one finding in the same call addressed one slot.
    DuplicateFinding {
        /// The duplicated slot.
        slot: Slot,
    },
}

/// A structural error paired with the unchanged input assembly.
#[derive(Debug, PartialEq, Eq)]
pub struct AdjudicationError<Value> {
    assembly: Assembly<Value>,
    kind: StructuralError,
}

impl<Value> AdjudicationError<Value> {
    /// Borrow the unchanged assembly supplied to adjudication.
    #[must_use]
    pub const fn assembly(&self) -> &Assembly<Value> {
        &self.assembly
    }

    /// Return the structural error kind.
    #[must_use]
    pub const fn kind(&self) -> StructuralError {
        self.kind
    }

    /// Recover the unchanged assembly supplied to adjudication.
    #[must_use]
    pub fn into_assembly(self) -> Assembly<Value> {
        self.assembly
    }

    /// Split this error into its unchanged assembly and structural error kind.
    #[must_use]
    pub fn into_parts(self) -> (Assembly<Value>, StructuralError) {
        (self.assembly, self.kind)
    }
}

/// Fold a finite set of located findings into an assembly.
///
/// All slots and same-call duplicates are validated before the assembly can
/// change. Existing values are never replaced or revoked. If multiple
/// unresolved slots are impossible, the lowest slot wins independently of
/// finding order.
pub fn adjudicate<Value, Cause>(
    mut assembly: Assembly<Value>,
    findings: impl IntoIterator<Item = LocatedFinding<Value, Cause>>,
) -> Result<Decision<Value, Cause>, AdjudicationError<Value>> {
    let findings: Vec<_> = findings.into_iter().collect();
    let mut seen = vec![false; assembly.len()];

    for located in &findings {
        let slot = located.slot();
        let Some(was_seen) = seen.get_mut(slot.index()) else {
            return Err(AdjudicationError {
                kind: StructuralError::SlotOutOfRange {
                    slot,
                    slot_count: assembly.len(),
                },
                assembly,
            });
        };
        if *was_seen {
            return Err(AdjudicationError {
                kind: StructuralError::DuplicateFinding { slot },
                assembly,
            });
        }
        *was_seen = true;
    }

    let mut impossible: Option<(Slot, Cause)> = None;
    for located in findings {
        let index = located.slot.index();
        if assembly.values[index].is_some() {
            continue;
        }

        match located.finding {
            Finding::Produced(value) => assembly.values[index] = Some(value),
            Finding::Impossible(cause) => {
                let candidate = (located.slot, cause);
                if impossible
                    .as_ref()
                    .is_none_or(|(slot, _)| candidate.0 < *slot)
                {
                    impossible = Some(candidate);
                }
            }
        }
    }

    if let Some((slot, cause)) = impossible {
        return Ok(Decision::Impossible { slot, cause });
    }

    if assembly.is_complete() {
        let values = assembly
            .values
            .into_iter()
            .map(|value| value.expect("complete assembly has no unresolved slot"))
            .collect();
        Ok(Decision::Ready(values))
    } else {
        Ok(Decision::Pending(assembly))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn produced(slot: usize, value: &str) -> LocatedFinding<&str, &str> {
        LocatedFinding::new(Slot::new(slot), Finding::Produced(value))
    }

    fn impossible(slot: usize, cause: &str) -> LocatedFinding<&str, &str> {
        LocatedFinding::new(Slot::new(slot), Finding::Impossible(cause))
    }

    #[test]
    fn partial_production_is_pending_and_monotonic() {
        let assembly = Assembly::new(2);
        let Decision::Pending(assembly) = adjudicate(assembly, [produced(1, "second")]).unwrap()
        else {
            panic!("one unresolved slot must remain pending");
        };

        let decision = adjudicate(
            assembly,
            [produced(1, "replacement"), impossible(0, "gone")],
        )
        .unwrap();

        assert_eq!(
            decision,
            Decision::Impossible {
                slot: Slot::new(0),
                cause: "gone"
            }
        );
    }

    #[test]
    fn ready_values_follow_slot_order() {
        let decision = adjudicate(
            Assembly::new(3),
            [
                produced(2, "third"),
                produced(0, "first"),
                produced(1, "second"),
            ],
        )
        .unwrap();

        assert_eq!(decision, Decision::Ready(vec!["first", "second", "third"]));
    }

    #[test]
    fn lowest_unresolved_impossible_slot_wins() {
        let decision = adjudicate(
            Assembly::new(4),
            [impossible(3, "later"), impossible(1, "earlier")],
        )
        .unwrap();

        assert_eq!(
            decision,
            Decision::Impossible {
                slot: Slot::new(1),
                cause: "earlier"
            }
        );
    }

    #[test]
    fn empty_assembly_is_ready() {
        let decision = adjudicate::<(), ()>(Assembly::new(0), core::iter::empty()).unwrap();
        assert_eq!(decision, Decision::Ready(vec![]));
    }

    #[test]
    fn structural_error_returns_unchanged_assembly() {
        let Decision::Pending(assembly) =
            adjudicate(Assembly::new(2), [produced(0, "kept")]).unwrap()
        else {
            panic!("one slot remains unresolved");
        };

        let error = adjudicate(
            assembly,
            [produced(1, "not captured"), produced(2, "outside")],
        )
        .unwrap_err();

        assert_eq!(
            error.kind(),
            StructuralError::SlotOutOfRange {
                slot: Slot::new(2),
                slot_count: 2
            }
        );
        assert_eq!(error.assembly().value(Slot::new(0)), Some(&"kept"));
        assert_eq!(error.assembly().value(Slot::new(1)), None);
    }

    #[test]
    fn same_call_duplicates_are_rejected_even_for_captured_slots() {
        let Decision::Pending(assembly) =
            adjudicate(Assembly::new(2), [produced(0, "kept")]).unwrap()
        else {
            panic!("one slot remains unresolved");
        };

        let error = adjudicate(
            assembly,
            [produced(0, "ignored"), impossible(0, "also ignored")],
        )
        .unwrap_err();

        assert_eq!(
            error.kind(),
            StructuralError::DuplicateFinding { slot: Slot::new(0) }
        );
        assert_eq!(error.assembly().value(Slot::new(0)), Some(&"kept"));
    }
}
