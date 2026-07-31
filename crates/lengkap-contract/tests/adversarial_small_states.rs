use lengkap_contract::{
    AdjudicationError, Assembly, Decision, Finding, LocatedFinding, Slot, StructuralError,
    adjudicate,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModelFinding {
    Produced { slot: usize },
    Impossible { slot: usize },
}

impl ModelFinding {
    fn slot(self) -> usize {
        match self {
            Self::Produced { slot } | Self::Impossible { slot } => slot,
        }
    }

    fn into_located(self) -> LocatedFinding<usize, usize> {
        let slot = Slot::new(self.slot());
        let finding = match self {
            Self::Produced { slot } => Finding::Produced(produced_value(slot)),
            Self::Impossible { slot } => Finding::Impossible(impossible_cause(slot)),
        };
        LocatedFinding::new(slot, finding)
    }
}

fn captured_value(slot: usize) -> usize {
    100 + slot
}

fn produced_value(slot: usize) -> usize {
    200 + slot
}

fn impossible_cause(slot: usize) -> usize {
    300 + slot
}

fn slots_for_mask(slot_count: usize, mask: usize) -> Vec<Option<usize>> {
    (0..slot_count)
        .map(|slot| (mask & (1 << slot) != 0).then(|| captured_value(slot)))
        .collect()
}

fn findings_for_assignment(slot_count: usize, mut assignment: usize) -> Vec<ModelFinding> {
    let mut findings = Vec::new();
    for slot in 0..slot_count {
        match assignment % 3 {
            0 => {}
            1 => findings.push(ModelFinding::Produced { slot }),
            2 => findings.push(ModelFinding::Impossible { slot }),
            _ => unreachable!("base-three digit is always zero, one, or two"),
        }
        assignment /= 3;
    }
    findings
}

fn permutations(items: &[ModelFinding]) -> Vec<Vec<ModelFinding>> {
    fn visit(items: &mut [ModelFinding], start: usize, permutations: &mut Vec<Vec<ModelFinding>>) {
        if start == items.len() {
            permutations.push(items.to_vec());
            return;
        }

        for index in start..items.len() {
            items.swap(start, index);
            visit(items, start + 1, permutations);
            items.swap(start, index);
        }
    }

    let mut items = items.to_vec();
    let mut result = Vec::new();
    visit(&mut items, 0, &mut result);
    result
}

fn expected_decision(
    mut slots: Vec<Option<usize>>,
    findings: &[ModelFinding],
) -> Decision<usize, usize> {
    let mut impossible = None;

    for finding in findings {
        let slot = finding.slot();
        if slots[slot].is_some() {
            continue;
        }

        match finding {
            ModelFinding::Produced { .. } => slots[slot] = Some(produced_value(slot)),
            ModelFinding::Impossible { .. } => {
                if impossible.is_none_or(|current: usize| slot < current) {
                    impossible = Some(slot);
                }
            }
        }
    }

    if let Some(slot) = impossible {
        Decision::Impossible {
            assembly: Assembly::from_slots(slots),
            slot: Slot::new(slot),
            cause: impossible_cause(slot),
        }
    } else if slots.iter().all(Option::is_some) {
        Decision::Ready(
            slots
                .into_iter()
                .map(|value| value.expect("the model checked completeness"))
                .collect(),
        )
    } else {
        Decision::Pending(Assembly::from_slots(slots))
    }
}

fn actual_findings(findings: &[ModelFinding]) -> Vec<LocatedFinding<usize, usize>> {
    findings
        .iter()
        .copied()
        .map(ModelFinding::into_located)
        .collect()
}

#[test]
fn every_small_valid_state_is_deterministic_across_permutations_and_entrypoints() {
    for slot_count in 0..=4 {
        let mask_limit = 1usize << slot_count;
        let assignment_limit = 3usize.pow(slot_count as u32);

        for mask in 0..mask_limit {
            let initial_slots = slots_for_mask(slot_count, mask);
            for assignment in 0..assignment_limit {
                let model_findings = findings_for_assignment(slot_count, assignment);
                let expected = expected_decision(initial_slots.clone(), &model_findings);

                for permutation in permutations(&model_findings) {
                    let free = adjudicate(
                        Assembly::from_slots(initial_slots.clone()),
                        actual_findings(&permutation),
                    )
                    .expect("unique in-range model findings are structurally valid");
                    let method = Assembly::from_slots(initial_slots.clone())
                        .adjudicate(actual_findings(&permutation))
                        .expect("the method receives the same valid model findings");

                    assert_eq!(
                        free, expected,
                        "free function diverged for slots={slot_count}, mask={mask}, \
                         assignment={assignment}, permutation={permutation:?}"
                    );
                    assert_eq!(
                        method, expected,
                        "method diverged for slots={slot_count}, mask={mask}, \
                         assignment={assignment}, permutation={permutation:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn every_small_checkpoint_mask_round_trips_with_exact_progress() {
    for slot_count in 0..=8 {
        for mask in 0..(1usize << slot_count) {
            let slots = slots_for_mask(slot_count, mask);
            let assembly = Assembly::from_slots(slots.clone());
            let expected_unresolved: Vec<_> = slots
                .iter()
                .enumerate()
                .filter_map(|(slot, value)| value.is_none().then_some(Slot::new(slot)))
                .collect();

            assert_eq!(assembly.captured_len(), mask.count_ones() as usize);
            assert_eq!(
                assembly.remaining_len(),
                slot_count - mask.count_ones() as usize
            );
            assert_eq!(
                assembly.unresolved_slots().collect::<Vec<_>>(),
                expected_unresolved
            );
            assert_eq!(
                Assembly::from_slots(assembly.into_slots()).into_slots(),
                slots
            );
        }
    }
}

#[test]
fn multi_round_capture_is_monotonic_and_impossible_state_can_be_recovered() {
    let Decision::Pending(assembly) = Assembly::new(3)
        .adjudicate_one(LocatedFinding::<_, usize>::new(
            Slot::new(2),
            Finding::Produced(22),
        ))
        .expect("slot two is valid")
    else {
        panic!("two slots remain");
    };

    let Decision::Pending(assembly) = assembly
        .adjudicate_one(LocatedFinding::new(Slot::new(2), Finding::Impossible(302)))
        .expect("a later finding for a captured slot remains structurally valid")
    else {
        panic!("impossibility cannot revoke a captured value");
    };

    let Decision::Impossible {
        assembly,
        slot,
        cause,
    } = assembly
        .adjudicate([
            LocatedFinding::new(Slot::new(1), Finding::Produced(21)),
            LocatedFinding::new(Slot::new(0), Finding::Impossible(300)),
        ])
        .expect("both slots are valid")
    else {
        panic!("slot zero is impossible");
    };
    assert_eq!(slot, Slot::new(0));
    assert_eq!(cause, 300);
    assert_eq!(assembly.value(Slot::new(1)), Some(&21));
    assert_eq!(assembly.value(Slot::new(2)), Some(&22));

    let decision = assembly
        .adjudicate_one(LocatedFinding::<_, usize>::new(
            Slot::new(0),
            Finding::Produced(20),
        ))
        .expect("the caller owns whether to resume recovered state");
    assert_eq!(decision, Decision::Ready(vec![20, 21, 22]));
}

fn invalid_batch() -> Vec<LocatedFinding<usize, usize>> {
    vec![
        LocatedFinding::new(Slot::new(1), Finding::Produced(21)),
        LocatedFinding::new(Slot::new(3), Finding::Impossible(303)),
    ]
}

#[test]
fn invalid_batches_are_atomic_and_equivalent_across_entrypoints() {
    let initial_slots = vec![Some(10), None, None];

    let free: AdjudicationError<_, _> =
        adjudicate(Assembly::from_slots(initial_slots.clone()), invalid_batch())
            .expect_err("slot three is outside the assembly");
    let method = Assembly::from_slots(initial_slots.clone())
        .adjudicate(invalid_batch())
        .expect_err("the method must reject the same batch");

    assert_eq!(free, method);
    let (assembly, findings, kind) = free.into_parts();
    assert_eq!(assembly.into_slots(), initial_slots);
    assert_eq!(findings, invalid_batch());
    assert_eq!(
        kind,
        StructuralError::SlotOutOfRange {
            slot: Slot::new(3),
            slot_count: 3,
        }
    );
}

fn batch_with_extra(
    slot_count: usize,
    position: usize,
    extra_slot: usize,
) -> Vec<LocatedFinding<usize, usize>> {
    let mut findings = Vec::new();
    for index in 0..=slot_count {
        if index == position {
            findings.push(LocatedFinding::new(
                Slot::new(extra_slot),
                Finding::Impossible(impossible_cause(extra_slot)),
            ));
        }
        if index < slot_count {
            findings.push(LocatedFinding::new(
                Slot::new(index),
                Finding::Produced(produced_value(index)),
            ));
        }
    }
    findings
}

#[test]
fn every_small_invalid_batch_returns_both_inputs_unchanged() {
    for slot_count in 0..=4 {
        for mask in 0..(1usize << slot_count) {
            let initial_slots = slots_for_mask(slot_count, mask);

            for position in 0..=slot_count {
                let invalid_slot = slot_count + 1;
                let error = Assembly::from_slots(initial_slots.clone())
                    .adjudicate(batch_with_extra(slot_count, position, invalid_slot))
                    .expect_err("the extra slot is out of range");
                let (assembly, findings, kind) = error.into_parts();

                assert_eq!(assembly.into_slots(), initial_slots);
                assert_eq!(
                    findings,
                    batch_with_extra(slot_count, position, invalid_slot)
                );
                assert_eq!(
                    kind,
                    StructuralError::SlotOutOfRange {
                        slot: Slot::new(invalid_slot),
                        slot_count,
                    }
                );
            }

            for duplicate_slot in 0..slot_count {
                for position in 0..=slot_count {
                    let error = Assembly::from_slots(initial_slots.clone())
                        .adjudicate(batch_with_extra(slot_count, position, duplicate_slot))
                        .expect_err("the extra finding duplicates an in-range slot");
                    let (assembly, findings, kind) = error.into_parts();

                    assert_eq!(assembly.into_slots(), initial_slots);
                    assert_eq!(
                        findings,
                        batch_with_extra(slot_count, position, duplicate_slot)
                    );
                    assert_eq!(
                        kind,
                        StructuralError::DuplicateFinding {
                            slot: Slot::new(duplicate_slot),
                        }
                    );
                }
            }
        }
    }
}
