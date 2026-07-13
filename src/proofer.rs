use crate::prop::Prop;
use crate::proofs::*;
use crate::truth_table::TruthTable;

fn calc(hypotheses: &[Rc<Prop>]) -> Option<bool> {
    let Some(h) = hypotheses.first() else { return None; };
    let mut prop = h;
    for h in hypotheses.iter().skip(1) {
        prop = Rc::new(Prop::And(prop, h.clone()));
    }
    let atoms_count = prop.atoms_count();
    let tt = TruthTable::new(atoms_count, |x| prop.calc(x));
    if tt.is_contradiction() {
        Some(false)
    } else if tt.is_tautology() {
        Some(true)
    } else {
        None
    }
}

fn prove_atom_checking_contradiction(
    proof: &mut Proof, atom: &Rc<Prop>, hypotheses: &[Rc<Prop>]
) -> Option<isize> {
    let hv = calc(hypotheses);
    if hv == Some(true) {
        return None;
    }
    if hv == Some(false) {
        return prove_from_contradiction(proof, atom, hypotheses);
    }
    prove_atom(proof, atom, hypothesis)
}

fn prove_atom(proof: &mut Proof, atom: &Rc<Prop>, hypotheses: &[Rc<Prop>]) -> Option<isize> {
    if hypotheses.iter().any(|x| x == atom) {
        return Some(proof.prove_hypothesis(atom.clone()));
    }
    let mut dnots = Vec::new();
    let mut ands = Vec::new();
    let mut nors = Vec::new();
    let mut ntos = Vec::new();
    let mut splitted_hypotheses = Vec::new();
    for h in hypotheses {
        match h {
            Prop::And(a, b) => {
                ands.push((a, b));
                splitted_hypotheses.push(a.clone());
                splitted_hypotheses.push(b.clone());
            },
            Prop::Not(a) => match a {
                Prop::Not(a) => {
                    dnots.push(a);
                    splitted_hypotheses.push(a.clone());
                },
                Prop::Or(a, b) => {
                    nors.push((a, b));
                    splitted_hypotheses.push(Rc::new(Prop::Not(a.clone())));
                    splitted_hypotheses.push(Rc::new(Prop::Not(b.clone())));
                },
                Prop::To(a, b) => {
                    ntos.push((a, b));
                    splitted_hypotheses.push(Rc::new(Prop::Not(Rc::new(Prop::Not(a.clone())))));
                    splitted_hypotheses.push(Rc::new(Prop::Not(b.clone())));
                },
                _ => splitted_hypotheses.push(h.clone()),
            },
            _ => splitted_hypotheses.push(h.clone()),
        }
    }
    if !dnots.is_empty() || !ands.is_empty() || !nors.is_empty() || !ntos.is_empty() {
        let mut hypotheses_proofs = HashMap::new();
        for dnot in dnots {
            let p = proof.prove_hypothesis(Rc::new(Prop::Not(Rc::new(Prop::Not(dnot.clone())))));
            let q = prove_a_from_not_not_a(proof, p);
            hypotheses_proofs.insert(dnot.clone(), q);
        }
        for (a, b) in ands {
            let p = proof.prove_hypothesis(Rc::new(Prop::And(a.clone(), b.clone())));
            let q = prove_a_from_a_and_b(proof, p);
            let r = prove_b_from_a_and_b(proof, p);
            hypotheses_proofs.insert(a.clone(), q);
            hypotheses_proofs.insert(b.clone(), r);
        }
        for (a, b) in nors {
            let p = proof.prove_hypothesis(Rc::new(Prop::Not(Rc::new(Prop::Or(a.clone(), b.clone())))));
            let q = prove_not_a_from_a_nor_b(proof, p);
            let r = prove_not_b_from_a_nor_b(proof, p);
            hypotheses_proofs.insert(Rc::new(Prop::Not(a.clone())), q);
            hypotheses_proofs.insert(Rc::new(Prop::Not(b.clone())), r);
        }
        for (a, b) in ntos {
            let p = proof.prove_hypothesis(Rc::new(Prop::Not(Rc::new(Prop::To(a.clone(), b.clone())))));
            let q = prove_not_not_a_from_a_nto_b(proof, p);
            let r = prove_not_b_from_a_nto_b(proof, p);
            hypotheses_proofs.insert(Rc::new(Prop::Not(Rc::new(Prop::Not(a.clone())))), q);
            hypotheses_proofs.insert(Rc::new(Prop::Not(b.clone())), r);
        }
        let mut new_proof = Proof::new();
        prove_atom(&mut new_proof, atom, &splitted_hypotheses)?;
        return proof.extend(&new_proof, |x| hypotheses_proofs.get(x).copied());
    }
    if let Some(h) = hypotheses.find(|x| matches!(x.as_ref(), Prop::Or(_, _))) {
        let reduced_hypotheses = hypotheses.iter().filter(|x| x != h).collect();
        let mut reduced_proof = Proof::new();
        if prove_atom(&mut reduced_proof, atom, &reduced_hypotheses).is_some() {
            return proof.extend(&reduced_proof, |_| None);
        }
        let mut hypotheses_a = reduced_hypotheses.clone();
        let mut hypotheses_b = reduced_hypotheses.clone();
        let a = h.or_left_arg().unwrap().clone();
        let b = h.or_right_arg().unwrap().clone();
        hypotheses_a.push(a.clone());
        hypotheses_b.push(b.clone());
        let mut proof_a = Proof::new();
        let mut proof_b = Proof::new();
        prove_atom_checking_contradiction(&proof_a, atom, &hypotheses_a)?;
        prove_atom_checking_contradiction(&proof_b, atom, &hypotheses_b)?;
        let p = proof.prove_hypothesis(h.clone());
        let q = proof.deduct(&proof_a, &a);
        let r = proof.deduct(&proof_b, &b);
        let s = proof.prove_axiom(Rc::new(axiom_8(a, b, atom.clone())), 8);
        let t = proof.modus_ponens(q, s);
        let u = proof.modus_ponens(r, t);
        return Some(proof.modus_ponens(p, u));
    }
    if let Some(h) = hypotheses.find(|x| matches!(x.as_ref(), Prop::To(_, _))) {
        let reduced_hypotheses = hypotheses.iter().filter(|x| x != h).collect();
        let mut reduced_proof = Proof::new();
        if prove_atom(&mut reduced_proof, atom, &reduced_hypotheses).is_some() {
            return proof.extend(&reduced_proof, |_| None);
        }
        a -> b
        ~a -> x
        b -> x
        (a -> x) -> (~a -> x) -> (a | ~a -> x)
        (a -> b -> x) -> (a -> b) -> (a -> x)

        ~a -> b
        a -> x
        b -> x
        (a -> x) -> (~a -> x) -> (a | ~a -> x)
        (~a -> b -> x) -> (~a -> b) -> (~a -> x)
        let mut hypotheses_a = reduced_hypotheses.clone();
        let mut hypotheses_b = reduced_hypotheses.clone();
        let a = h.premise().unwrap().clone();
        let b = h.conclusion().unwrap().clone();
        hypotheses_a.push(a.clone());
        hypotheses_b.push(b.clone());
        let mut proof_a = Proof::new();
        let mut proof_b = Proof::new();
        prove_atom_checking_contradiction(&proof_a, atom, &hypotheses_a)?;
        prove_atom_checking_contradiction(&proof_b, atom, &hypotheses_b)?;
        let p = proof.prove_hypothesis(h.clone());
        let q = proof.deduct(&proof_a, &a);
        let r = proof.deduct(&proof_b, &b);
        let s = proof.prove_axiom(Rc::new(axiom_8(a, b, atom.clone())), 8);
        let t = proof.modus_ponens(q, s);
        let u = proof.modus_ponens(r, t);
        return Some(proof.modus_ponens(p, u));
    }
}

pub fn prove(proof: &mut Proof, target: &Rc<Prop>, hypotheses: &[Rc<Prop>]) -> Option<isize> {
    let hv = calc(hypotheses);
    if hv == Some(true) {
        return prove(proof, target, &[]);
    }
    if hv == Some(false) {
        return prove_from_contradiction(proof, target, hypotheses);
    }
    match target.as_ref() {
        Prop::Atom(a) => prove_atom(proof, target, hypotheses),
        Prop::And(a, b) => {
            let a = prove(proof, a, hypotheses)?;
            let b = prove(proof, b, hypotheses)?;
            Some(prove_a_and_b(proof, a, b))
        },
        Prop::Or(a, b) => match a {
            Prop::Not(a) => {
                let a_to_b = prove(proof, &Rc::new(Prop::To(a.clone(), b.clone())), hypotheses)?;
                Some(prove_not_a_or_b(proof, a_to_b))
            },
            _ => match b {
                Prop::Not(b) => {
                    let b_to_a = prove(proof, &Rc::new(Prop::To(b.clone(), a.clone())), hypotheses)?;
                    Some(prove_a_or_not_b(proof, b_to_a))
                },
                _ => {
                    let not_a_to_b = prove(
                        proof, &Rc::new(Prop::To(Rc::new(Prop::Not(a.clone())), b.clone())), hypotheses
                    )?;
                    Some(prove_a_or_b(proof, not_a_to_b))
                }
            },
        },
        Prop::To(a, b) => {
            let mut new_hypotheses = hypotheses.iter().cloned().collect();
            new_hypotheses.push(a.clone());
            let mut new_proof = Proof::new();
            prove(&mut new_proof, b, &new_hypotheses)?;
            proof.deduct(&new_proof, a)
        },
        Prop::Not(a) => match a {
            Prop::Atom(_) => prove_atom(proof, target, hypotheses),
            Prop::Not(a) => {
                let a = prove(proof, a, hypotheses)?;
                Some(prove_not_not_a(proof, a))
            },
            Prop::Or(a, b) => match a {
                Prop::Not(a) => {
                    let a = prove(proof, a, hypotheses)?;
                    let not_b = prove(proof, &Rc::new(Prop::Not(b.clone())), hypotheses)?;
                    Some(prove_not_a_nor_b(proof, a, not_b))
                },
                _ => match b {
                    Prop::Not(b) => {
                        let not_a = prove(proof, &Rc::new(Prop::Not(a.clone())), hypotheses)?;
                        let b = prove(proof, b, hypotheses)?;
                        Some(prove_a_nor_not_b(proof, not_a, b))
                    },
                    _ => {
                        let not_a = prove(proof, &Rc::new(Prop::Not(a.clone())), hypotheses)?;
                        let not_b = prove(proof, &Rc::new(Prop::Not(b.clone())), hypotheses)?;
                        Some(prove_a_nor_b(proof, not_a, not_b))
                    },
                },
            },
            Prop::And(a, b) => match a {
                Prop::Not(a) => {
                    let b_to_a = prove(proof, &Rc::new(Prop::To(b.clone(), a.clone())), hypotheses)?;
                    Some(prove_not_a_nand_b(proof, b_to_a))
                },
                _ => match b {
                    Prop::Not(b) => {
                        let a_to_b = prove(proof, &Rc::new(Prop::To(a.clone(), b.clone())), hypotheses)?;
                        Some(prove_a_nand_not_b(proof, a_to_b))
                    },
                    _ => {
                        let a_to_not_b = prove(
                            proof, &Rc::new(Prop::To(a.clone(), Rc::new(Prop::Not(b.clone())))), hypotheses
                        )?;
                        Some(prove_a_nand_b(proof, a_to_not_b))
                    },
                },
            },
            Prop::To(a, b) => match b {
                Prop::Not(b) => {
                    let a = prove(proof, a, hypotheses)?;
                    let b = prove(proof, b, hypotheses)?;
                    Some(prove_a_nto_not_b(proof, a, b))
                },
                _ => {
                    let a = prove(proof, a, hypotheses)?;
                    let not_b = prove(proof, &Rc::new(Prop::Not(b.clone())), hypotheses)?;
                    Some(prove_a_nto_b(proof, a, not_b))
                },
            },
        },
    }
}
