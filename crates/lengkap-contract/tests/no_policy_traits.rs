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

#[test]
fn checkpoint_and_error_recovery_need_no_policy_traits() {
    let checkpoint = Assembly::from_slots(vec![Some(ValueWithoutPolicyTraits(3)), None]);
    let restored = Assembly::from_slots(checkpoint.into_slots());
    assert_eq!(restored.value(Slot::new(0)).map(|value| value.0), Some(3));

    let result = restored.adjudicate([
        LocatedFinding::new(Slot::new(1), Finding::Produced(ValueWithoutPolicyTraits(7))),
        LocatedFinding::new(Slot::new(2), Finding::Impossible(CauseWithoutPolicyTraits)),
    ]);
    let Err(error) = result else {
        panic!("the second finding is outside the assembly");
    };
    let (assembly, findings, kind) = error.into_parts();

    assert_eq!(assembly.value(Slot::new(0)).map(|value| value.0), Some(3));
    assert_eq!(assembly.value(Slot::new(1)).map(|value| value.0), None);
    let mut findings = findings.into_iter();
    let (slot, finding) = findings.next().expect("produced finding").into_parts();
    assert_eq!(slot, Slot::new(1));
    let Finding::Produced(value) = finding else {
        panic!("the first recovered finding must own its produced value");
    };
    assert_eq!(value.0, 7);

    let (slot, finding) = findings.next().expect("impossible finding").into_parts();
    assert_eq!(slot, Slot::new(2));
    let Finding::Impossible(CauseWithoutPolicyTraits) = finding else {
        panic!("the second recovered finding must own its cause");
    };
    assert!(findings.next().is_none());
    assert_eq!(
        kind,
        lengkap_contract::StructuralError::SlotOutOfRange {
            slot: Slot::new(2),
            slot_count: 2,
        }
    );
}
