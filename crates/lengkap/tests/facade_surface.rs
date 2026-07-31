use lengkap::{Assembly, Decision, Finding, LocatedFinding, Slot};

fn accepts_contract_type(_: lengkap_contract::Assembly<&'static str>) {}

#[test]
fn facade_exposes_identical_contract_types_and_methods() {
    accepts_contract_type(Assembly::new(1));

    let decision = Assembly::new(1)
        .adjudicate_one(LocatedFinding::<_, &str>::new(
            Slot::new(0),
            Finding::Produced("ready"),
        ))
        .expect("the facade exposes valid contract adjudication");

    assert_eq!(decision, Decision::Ready(vec!["ready"]));
}
