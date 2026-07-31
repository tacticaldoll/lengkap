//! A Worklane-shaped bridge without a Worklane dependency.

use lengkap_contract::{Assembly, Decision, Finding, LocatedFinding, Slot, adjudicate};

enum ObservedJob {
    Live,
    CompletedOrUnknown(Option<Vec<u8>>),
    DeadLettered,
}

#[derive(Debug, PartialEq, Eq)]
enum CannotComplete {
    MissingResult,
    DeadLettered,
}

fn finding(slot: Slot, job: ObservedJob) -> Option<LocatedFinding<Vec<u8>, CannotComplete>> {
    match job {
        ObservedJob::Live => None,
        ObservedJob::CompletedOrUnknown(Some(result)) => {
            Some(LocatedFinding::new(slot, Finding::Produced(result)))
        }
        ObservedJob::CompletedOrUnknown(None) => Some(LocatedFinding::new(
            slot,
            Finding::Impossible(CannotComplete::MissingResult),
        )),
        ObservedJob::DeadLettered => Some(LocatedFinding::new(
            slot,
            Finding::Impossible(CannotComplete::DeadLettered),
        )),
    }
}

fn main() {
    let observed = [
        ObservedJob::CompletedOrUnknown(Some(b"invoice".to_vec())),
        ObservedJob::Live,
    ];
    let findings = observed
        .into_iter()
        .enumerate()
        .filter_map(|(index, job)| finding(Slot::new(index), job));

    let Decision::Pending(assembly) =
        adjudicate(Assembly::new(2), findings).expect("slots are valid")
    else {
        panic!("one live job leaves fan-in pending");
    };

    let decision = adjudicate(assembly, finding(Slot::new(1), ObservedJob::DeadLettered))
        .expect("slot is valid");

    assert!(matches!(
        decision,
        Decision::Impossible {
            slot,
            cause: CannotComplete::DeadLettered
        } if slot == Slot::new(1)
    ));
}
