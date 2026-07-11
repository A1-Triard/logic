use crate::axioms::*;
use crate::proof::Proof;
use crate::prop::Prop;
use std::rc::Rc;

pub fn prove_a_and_b(proof: &mut Proof, a: isize, b: isize) -> isize {
    let a_pr = proof.prop(a).clone();
    let b_pr = proof.prop(b).clone();
    let p = proof.prove_axiom(Rc::new(axiom_5(a_pr, b_pr)), 5);
    let q = proof.modus_ponens(a, p);
    proof.modus_ponens(b, q)
}

pub fn prove_not_a_or_b(proof: &mut Proof, a_to_b: isize) -> isize {
    let a = proof.prop(a_to_b).premise().expect("invalid implication").clone();
    let b = proof.prop(a_to_b).conclusion().expect("invalid implication").clone();
    let not_a = Rc::new(Prop::Not(a.clone()));
    let not_a_or_b = Rc::new(Prop::Or(not_a.clone(), b.clone()));
    let p = proof.prove_axiom(Rc::new(axiom_7(not_a.clone(), b.clone())), 7);
    let q = proof.prove_axiom(Rc::new(axiom_1(
        Rc::new(Prop::To(b.clone(), not_a_or_b.clone())),
        a.clone(),
    )), 1);
    let r = proof.modus_ponens(p, q); // a -> b -> ~a | b
    let s = proof.prove_axiom(Rc::new(axiom_2(a.clone(), b.clone(), not_a_or_b.clone())), 2);
    let t = proof.modus_ponens(r, s); // (a -> b) -> (a -> ~a | b)
    let u = proof.modus_ponens(a_to_b, t);
    let v = proof.prove_axiom(Rc::new(axiom_6(not_a.clone(), b)), 6);
    let w = proof.prove_axiom(Rc::new(axiom_8(a.clone(), not_a, not_a_or_b)), 8);
    let x = proof.modus_ponens(u, w);
    let y = proof.modus_ponens(v, x); // a | ~a -> ~a | b
    let z = proof.prove_axiom(Rc::new(axiom_11(a)), 11);
    proof.modus_ponens(z, y)
}
