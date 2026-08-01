//! Recovering an `Impossible` slot: the core does not seal it.
//!
//! `Impossible` is a one-time decision, not a state stamped onto the slot.
//! The slot addressed by an impossible finding remains unresolved in the
//! returned assembly, so whether to accept that impossibility or retry the
//! same slot is entirely caller policy, per `PROJECT.md`'s User Obligations
//! ("whether contradictory evidence should be audited or rejected").

use lengkap_contract::{Assembly, Decision, Finding, LocatedFinding, Slot};

fn main() {
    let Decision::Impossible {
        assembly,
        slot,
        cause,
    } = Assembly::new(2)
        .adjudicate_one(LocatedFinding::<&str, &str>::new(
            Slot::new(0),
            Finding::Impossible("probe timed out"),
        ))
        .expect("the finding addresses an in-range slot")
    else {
        panic!("the only unresolved slot received an impossible finding");
    };

    assert_eq!(slot, Slot::new(0));
    assert_eq!(cause, "probe timed out");
    assert_eq!(assembly.value(Slot::new(0)), None);

    // Caller policy: a timeout is worth retrying. Nothing in the core
    // prevents resubmitting a produced finding for the same slot.
    let decision = assembly
        .adjudicate_one(LocatedFinding::<_, &str>::new(
            Slot::new(0),
            Finding::Produced("probe result"),
        ))
        .expect("the finding addresses an in-range slot");

    let Decision::Pending(assembly) = decision else {
        panic!("the second slot is still unresolved");
    };

    let decision = assembly
        .adjudicate_one(LocatedFinding::<_, &str>::new(
            Slot::new(1),
            Finding::Produced("second probe result"),
        ))
        .expect("the finding addresses an in-range slot");

    assert_eq!(
        decision,
        Decision::Ready(vec!["probe result", "second probe result"])
    );
}
