//! Transfer partial progress through a caller-owned checkpoint boundary.

use lengkap_contract::{Assembly, Decision, Finding, LocatedFinding, Slot};

fn main() {
    let Decision::Pending(partial) = Assembly::new(2)
        .adjudicate_one(LocatedFinding::<_, &str>::new(
            Slot::new(1),
            Finding::Produced("second"),
        ))
        .expect("the finding addresses an in-range slot")
    else {
        panic!("one slot remains unresolved");
    };

    // The caller owns what happens between these operations. Lengkap supplies
    // ordered owned slots, not an encoding, storage API, or I/O policy.
    let checkpoint = partial.into_slots();
    let restored = Assembly::from_slots(checkpoint);

    let decision = restored
        .adjudicate_one(LocatedFinding::<_, &str>::new(
            Slot::new(0),
            Finding::Produced("first"),
        ))
        .expect("the finding addresses an in-range slot");

    assert_eq!(decision, Decision::Ready(vec!["first", "second"]));
}
