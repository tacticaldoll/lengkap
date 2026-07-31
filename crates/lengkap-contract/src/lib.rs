//! A `no_std`, sans-I/O all-of evidence completion core.
//!
//! [`Assembly`] owns a fixed ordered set of slots. [`adjudicate`] captures
//! [`Finding::Produced`] values monotonically, reports progress without I/O,
//! and returns owned state at recovery boundaries. The core does not obtain,
//! verify, persist, serialize, or react to evidence.

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

    /// Restore an assembly from its ordered optional slots.
    ///
    /// The vector is an in-memory ownership boundary, not a durable encoding.
    /// Callers own any serialization, versioning, or persistence policy.
    #[must_use]
    pub fn from_slots(values: Vec<Option<Value>>) -> Self {
        Self { values }
    }

    /// Export this assembly as ordered optional slots.
    ///
    /// Passing the result to [`Assembly::from_slots`] preserves slot count,
    /// order, resolved positions, and owned values.
    #[must_use]
    pub fn into_slots(self) -> Vec<Option<Value>> {
        self.values
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

    /// Return the number of slots that have captured values.
    #[must_use]
    pub fn captured_len(&self) -> usize {
        self.values.iter().filter(|value| value.is_some()).count()
    }

    /// Return the number of unresolved slots.
    #[must_use]
    pub fn remaining_len(&self) -> usize {
        self.len() - self.captured_len()
    }

    /// Iterate over unresolved slots in stable ascending order.
    pub fn unresolved_slots(&self) -> impl Iterator<Item = Slot> + '_ {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(index, value)| value.is_none().then_some(Slot::new(index)))
    }

    /// Fold a finite set of located findings into this assembly.
    ///
    /// This method is equivalent to the free [`adjudicate`] function.
    pub fn adjudicate<Cause>(
        self,
        findings: impl IntoIterator<Item = LocatedFinding<Value, Cause>>,
    ) -> Result<Decision<Value, Cause>, AdjudicationError<Value, Cause>> {
        adjudicate(self, findings)
    }

    /// Fold one located finding into this assembly.
    ///
    /// This is the ownership-first convenience for repeated caller-driven
    /// observation loops.
    pub fn adjudicate_one<Cause>(
        self,
        finding: LocatedFinding<Value, Cause>,
    ) -> Result<Decision<Value, Cause>, AdjudicationError<Value, Cause>> {
        adjudicate(self, core::iter::once(finding))
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
        /// The assembly after all produced findings in this call were captured.
        assembly: Assembly<Value>,
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

/// A structural error paired with both unchanged adjudication inputs.
#[derive(Debug, PartialEq, Eq)]
pub struct AdjudicationError<Value, Cause> {
    assembly: Assembly<Value>,
    findings: Vec<LocatedFinding<Value, Cause>>,
    kind: StructuralError,
}

impl<Value, Cause> AdjudicationError<Value, Cause> {
    /// Borrow the unchanged assembly supplied to adjudication.
    #[must_use]
    pub const fn assembly(&self) -> &Assembly<Value> {
        &self.assembly
    }

    /// Borrow the complete finding batch supplied to adjudication.
    #[must_use]
    pub fn findings(&self) -> &[LocatedFinding<Value, Cause>] {
        &self.findings
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

    /// Recover the complete finding batch supplied to adjudication.
    #[must_use]
    pub fn into_findings(self) -> Vec<LocatedFinding<Value, Cause>> {
        self.findings
    }

    /// Split this error into both unchanged inputs and its error kind.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        Assembly<Value>,
        Vec<LocatedFinding<Value, Cause>>,
        StructuralError,
    ) {
        (self.assembly, self.findings, self.kind)
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
) -> Result<Decision<Value, Cause>, AdjudicationError<Value, Cause>> {
    let findings: Vec<_> = findings.into_iter().collect();
    let mut seen = vec![false; assembly.len()];
    let mut structural_error = None;

    for located in &findings {
        let slot = located.slot();
        let Some(was_seen) = seen.get_mut(slot.index()) else {
            structural_error = Some(StructuralError::SlotOutOfRange {
                slot,
                slot_count: assembly.len(),
            });
            break;
        };
        if *was_seen {
            structural_error = Some(StructuralError::DuplicateFinding { slot });
            break;
        }
        *was_seen = true;
    }

    if let Some(kind) = structural_error {
        return Err(AdjudicationError {
            assembly,
            findings,
            kind,
        });
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
        return Ok(Decision::Impossible {
            assembly,
            slot,
            cause,
        });
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
                assembly: Assembly::from_slots(vec![None, Some("second")]),
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
                assembly: Assembly::new(4),
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
        assert_eq!(
            error.findings(),
            &[produced(1, "not captured"), produced(2, "outside")]
        );
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
        assert_eq!(
            error.findings(),
            &[produced(0, "ignored"), impossible(0, "also ignored")]
        );
    }

    #[test]
    fn impossible_returns_prior_and_same_call_progress() {
        let Decision::Pending(assembly) =
            adjudicate(Assembly::new(3), [produced(0, "first")]).unwrap()
        else {
            panic!("two slots remain unresolved");
        };

        let Decision::Impossible {
            assembly,
            slot,
            cause,
        } = assembly
            .adjudicate([produced(2, "third"), impossible(1, "gone")])
            .unwrap()
        else {
            panic!("one unresolved slot is impossible");
        };

        assert_eq!(slot, Slot::new(1));
        assert_eq!(cause, "gone");
        assert_eq!(
            assembly.into_slots(),
            vec![Some("first"), None, Some("third")]
        );
    }

    #[test]
    fn slot_transfer_round_trips_empty_partial_and_complete_assemblies() {
        let cases = [
            vec![],
            vec![Some("first"), None, Some("third")],
            vec![Some("first"), Some("second")],
        ];

        for slots in cases {
            let expected = slots.clone();
            let assembly = Assembly::from_slots(slots);
            let expected_len = assembly.len();
            let expected_captured = assembly.captured_len();

            let restored = Assembly::from_slots(assembly.into_slots());

            assert_eq!(restored.len(), expected_len);
            assert_eq!(restored.captured_len(), expected_captured);
            assert_eq!(restored.into_slots(), expected);
        }
    }

    #[test]
    fn progress_reports_counts_and_stable_unresolved_slots() {
        let assembly = Assembly::from_slots(vec![None, Some("second"), None, Some("fourth")]);

        assert_eq!(assembly.captured_len(), 2);
        assert_eq!(assembly.remaining_len(), 2);
        assert_eq!(
            assembly.unresolved_slots().collect::<Vec<_>>(),
            vec![Slot::new(0), Slot::new(2)]
        );
        assert_eq!(assembly.value(Slot::new(1)), Some(&"second"));

        let empty = Assembly::<&str>::new(0);
        assert_eq!(empty.captured_len(), 0);
        assert_eq!(empty.remaining_len(), 0);
        assert_eq!(empty.unresolved_slots().next(), None);
    }

    #[test]
    fn method_and_free_function_batch_entrypoints_are_equivalent() {
        let findings = || [produced(1, "second")];

        assert_eq!(
            adjudicate(Assembly::new(2), findings()),
            Assembly::new(2).adjudicate(findings())
        );
    }

    #[test]
    fn single_finding_method_matches_batch_rules_and_recovers_invalid_input() {
        let Decision::Pending(assembly) = Assembly::new(2)
            .adjudicate_one(produced(0, "first"))
            .unwrap()
        else {
            panic!("one slot remains unresolved");
        };
        assert_eq!(assembly.value(Slot::new(0)), Some(&"first"));

        let error = assembly.adjudicate_one(produced(2, "outside")).unwrap_err();
        let (assembly, findings, kind) = error.into_parts();

        assert_eq!(assembly.value(Slot::new(0)), Some(&"first"));
        assert_eq!(findings, vec![produced(2, "outside")]);
        assert_eq!(
            kind,
            StructuralError::SlotOutOfRange {
                slot: Slot::new(2),
                slot_count: 2
            }
        );
    }
}
