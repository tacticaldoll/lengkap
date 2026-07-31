use lengkap_contract::{Assembly, Decision, Finding, LocatedFinding, Slot};

fn produced(slot: usize, value: &str) -> LocatedFinding<&str, ()> {
    LocatedFinding::new(Slot::new(slot), Finding::Produced(value))
}

#[test]
fn complete_assembly_extracts_every_value_in_slot_order() {
    let decision = Assembly::new(3)
        .adjudicate([
            produced(2, "third"),
            produced(0, "first"),
            produced(1, "second"),
        ])
        .expect("all findings address distinct in-range slots");

    assert_eq!(decision, Decision::Ready(vec!["first", "second", "third"]));
}

#[test]
fn empty_assembly_extracts_an_empty_ready_value() {
    let decision = Assembly::<()>::new(0)
        .adjudicate(core::iter::empty::<LocatedFinding<(), ()>>())
        .expect("an empty finding batch is structurally valid");

    assert_eq!(decision, Decision::Ready(vec![]));
}

#[test]
fn incomplete_assembly_remains_available_as_pending() {
    let decision = Assembly::new(2)
        .adjudicate_one(produced(1, "second"))
        .expect("the finding addresses an in-range slot");

    assert_eq!(
        decision,
        Decision::Pending(Assembly::from_slots(vec![None, Some("second")]))
    );
}
