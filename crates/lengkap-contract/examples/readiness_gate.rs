//! An application-startup readiness gate, without queue vocabulary.
//!
//! The Worklane-shaped example models a fan-in of job observations.
//! This example models the same fixed all-of shape from an unrelated
//! domain: a fixed set of subsystems must each report ready before an
//! application serves traffic. `Assembly`, `Slot`, and `Finding` need no
//! adaptation to fit either domain.

use lengkap_contract::{Assembly, Decision, Finding, LocatedFinding, Slot, adjudicate};

const CONFIG: Slot = Slot::new(0);
const DATABASE: Slot = Slot::new(1);
const CACHE: Slot = Slot::new(2);

fn main() {
    let findings = [
        LocatedFinding::<_, &str>::new(CONFIG, Finding::Produced("config loaded")),
        LocatedFinding::new(DATABASE, Finding::Produced("database connected")),
        LocatedFinding::new(CACHE, Finding::Produced("cache warmed")),
    ];

    let decision = adjudicate(Assembly::new(3), findings).expect("every slot is in range");

    assert_eq!(
        decision,
        Decision::Ready(vec!["config loaded", "database connected", "cache warmed"])
    );
}
