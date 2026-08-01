//! Recovering from a structural error caused by a caller mapping bug.
//!
//! A caller's slot-mapping logic can have an off-by-one bug that addresses a
//! slot outside the assembly. Adjudication rejects the whole batch
//! atomically and returns both original inputs unchanged, including any
//! progress the assembly already held, so the caller can inspect the error,
//! drop the invalid entry, and resubmit a corrected batch.

use lengkap_contract::{
    Assembly, Decision, Finding, LocatedFinding, Slot, StructuralError, adjudicate,
};

fn main() {
    let Decision::Pending(assembly) = Assembly::new(2)
        .adjudicate_one(LocatedFinding::<_, &str>::new(
            Slot::new(0),
            Finding::Produced("first"),
        ))
        .expect("the finding addresses an in-range slot")
    else {
        panic!("the second slot remains unresolved");
    };

    // A caller-side mapping bug addresses slot 2, which is out of range for
    // a two-slot assembly.
    let buggy_findings = [
        LocatedFinding::<_, &str>::new(Slot::new(1), Finding::Produced("second")),
        LocatedFinding::new(Slot::new(2), Finding::Produced("mis-mapped")),
    ];

    let error = adjudicate(assembly, buggy_findings).expect_err("slot 2 is out of range");

    assert_eq!(
        error.kind(),
        StructuralError::SlotOutOfRange {
            slot: Slot::new(2),
            slot_count: 2
        }
    );

    let (assembly, findings, _kind) = error.into_parts();
    assert_eq!(assembly.value(Slot::new(0)), Some(&"first"));

    // Recover the batch, drop the invalid entry, keep the rest.
    let corrected: Vec<_> = findings
        .into_iter()
        .filter(|located| located.slot() != Slot::new(2))
        .collect();

    let decision = adjudicate(assembly, corrected).expect("the corrected batch is in range");

    assert_eq!(decision, Decision::Ready(vec!["first", "second"]));
}
