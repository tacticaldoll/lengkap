use lengkap_contract::{Assembly, Decision, Finding, LocatedFinding, Slot, adjudicate};

struct ValueWithoutPolicyTraits(u8);
struct CauseWithoutPolicyTraits;

#[test]
fn user_types_need_no_clone_eq_hash_or_serialization_traits() {
    let result = adjudicate(
        Assembly::new(1),
        [LocatedFinding::<_, CauseWithoutPolicyTraits>::new(
            Slot::new(0),
            Finding::Produced(ValueWithoutPolicyTraits(7)),
        )],
    );
    let decision = match result {
        Ok(decision) => decision,
        Err(_) => panic!("the only slot is structurally valid"),
    };

    let Decision::Ready(values) = decision else {
        panic!("the assembly must be ready");
    };
    assert_eq!(values[0].0, 7);
}
