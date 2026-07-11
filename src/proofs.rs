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

pub fn prove_a_or_not_b(proof: &mut Proof, b_to_a: isize) -> isize {
    let a = proof.prop(b_to_a).conclusion().expect("invalid implication").clone();
    let b = proof.prop(b_to_a).premise().expect("invalid implication").clone();
    let not_b = Rc::new(Prop::Not(b.clone()));
    let a_or_not_b = Rc::new(Prop::Or(a.clone(), not_b.clone()));
    let p = proof.prove_axiom(Rc::new(axiom_6(a.clone(), not_b.clone())), 6);
    let q = proof.prove_axiom(Rc::new(axiom_1(
        Rc::new(Prop::To(a.clone(), a_or_not_b.clone())),
        b.clone(),
    )), 1);
    let r = proof.modus_ponens(p, q); // b -> a -> a | ~b
    let s = proof.prove_axiom(Rc::new(axiom_2(b.clone(), a.clone(), a_or_not_b.clone())), 2);
    let t = proof.modus_ponens(r, s); // (b -> a) -> (b -> a | ~b)
    let u = proof.modus_ponens(b_to_a, t);
    let v = proof.prove_axiom(Rc::new(axiom_7(a, not_b.clone())), 7);
    let w = proof.prove_axiom(Rc::new(axiom_8(b.clone(), not_b, a_or_not_b)), 8);
    let x = proof.modus_ponens(u, w);
    let y = proof.modus_ponens(v, x); // b | ~b -> a | ~b
    let z = proof.prove_axiom(Rc::new(axiom_11(b)), 11);
    proof.modus_ponens(z, y)
}

pub fn prove_a_or_b(proof: &mut Proof, not_a_to_b: isize) -> isize {
    let not_a = proof.prop(not_a_to_b).conclusion().expect("invalid implication").clone();
    let b = proof.prop(not_a_to_b).premise().expect("invalid implication").clone();
    let a = not_a.not_arg().expect("invalid negation");
    let a_or_b = Rc::new(Prop::Or(a.clone(), b.clone()));
    let p = proof.prove_axiom(Rc::new(axiom_7(a.clone(), b.clone())), 7);
    let q = proof.prove_axiom(Rc::new(axiom_1(
        Rc::new(Prop::To(b.clone(), a_or_b.clone())),
        not_a.clone(),
    )), 1);
    let r = proof.modus_ponens(p, q); // ~a -> b -> a | b
    let s = proof.prove_axiom(Rc::new(axiom_2(not_a.clone(), b.clone(), a_or_b.clone())), 2);
    let t = proof.modus_ponens(r, s); // (~a -> b) -> (~a -> a | b)
    let u = proof.modus_ponens(not_a_to_b, t);
    let v = proof.prove_axiom(Rc::new(axiom_6(a.clone(), b)), 6);
    let w = proof.prove_axiom(Rc::new(axiom_8(a.clone(), not_a.clone(), a_or_b)), 8);
    let x = proof.modus_ponens(v, w);
    let y = proof.modus_ponens(u, x); // a | ~a -> a | b
    let z = proof.prove_axiom(Rc::new(axiom_11(a.clone())), 11);
    proof.modus_ponens(z, y)
}

pub fn prove_not_not_a(proof: &mut Proof, a: isize) -> isize {
    let a_pr = proof.prop(a).clone();
    let not_a = Rc::new(Prop::Not(a_pr.clone()));
    let p = proof.prove_axiom(Rc::new(axiom_1(a_pr.clone(), not_a.clone())), 1);
    let q = proof.modus_ponens(a, p); // ~a -> a
    let r = proof.prove_a_to_a(not_a.clone());
    let s = proof.prove_axiom(Rc::new(axiom_10(not_a.clone(), a_pr.clone())), 10);
    let t = proof.modus_ponens(q, s);
    proof.modus_ponens(r, t)
}

pub fn prove_a_nor_b(proof: &mut Proof, not_a: isize, not_b: isize) -> isize {
    let not_a_pr = proof.prop(not_a).clone();
    let not_b_pr = proof.prop(not_b).clone();
    let a = not_a_pr.not_arg().expect("invalid negation");
    let b = not_b_pr.not_arg().expect("invalid negation");
    let a_or_b = Rc::new(Prop::Or(a.clone(), b.clone()));
    let p = proof.prove_axiom(Rc::new(axiom_9(a.clone(), a.clone())), 9);
    let q = proof.prove_axiom(Rc::new(axiom_9(b.clone(), a.clone())), 9);
    let r = proof.modus_ponens(not_a, p); // a -> a
    let s = proof.modus_ponens(not_b, q); // b -> a
    let t = proof.prove_axiom(Rc::new(axiom_8(a.clone(), b.clone(), a.clone())), 8);
    let u = proof.modus_ponens(r, t);
    let v = proof.modus_ponens(s, u); // a | b -> a
    let w = proof.prove_axiom(Rc::new(axiom_10(a_or_b.clone(), a.clone())), 10);
    let x = proof.modus_ponens(v, w);
    let y = proof.prove_axiom(Rc::new(axiom_1(not_a_pr, a_or_b)), 1);
    let z = proof.modus_ponens(not_a, y);
    proof.modus_ponens(z, x)
}

pub fn prove_not_a_nand_b(proof: &mut Proof, b_to_a: isize) -> isize {
    let b_to_a_pr = proof.prop(b_to_a).clone();
    let a = b_to_a_pr.conclusion().expect("invalid implication").clone();
    let b = b_to_a_pr.premise().expect("invalid implication").clone();
    let not_a = Rc::new(Prop::Not(a.clone()));
    let not_a_and_b = Rc::new(Prop::And(not_a.clone(), b.clone()));
    let p = proof.prove_axiom(Rc::new(axiom_1(b_to_a_pr.clone(), not_a_and_b.clone())), 1);
    let q = proof.modus_ponens(b_to_a, p); // ~a & b -> b -> a
    let r = proof.prove_axiom(Rc::new(axiom_2(not_a_and_b.clone(), b.clone(), a.clone())), 2);
    let s = proof.modus_ponens(q, r);
    let t = proof.prove_axiom(Rc::new(axiom_4(not_a.clone(), b.clone())), 4);
    let u = proof.modus_ponens(t, s); // ~a & b -> a
    let v = proof.prove_axiom(Rc::new(axiom_3(not_a.clone(), b.clone())), 3);
    let w = proof.prove_axiom(Rc::new(axiom_10(not_a_and_b, a)), 10);
    let x = proof.modus_ponens(u, w);
    proof.modus_ponens(v, x)
}

pub fn prove_a_nand_not_b(proof: &mut Proof, a_to_b: isize) -> isize {
    let a_to_b_pr = proof.prop(a_to_b).clone();
    let a = a_to_b_pr.premise().expect("invalid implication").clone();
    let b = a_to_b_pr.conclusion().expect("invalid implication").clone();
    let not_b = Rc::new(Prop::Not(b.clone()));
    let a_and_not_b = Rc::new(Prop::And(a.clone(), not_b.clone()));
    let p = proof.prove_axiom(Rc::new(axiom_1(a_to_b_pr.clone(), a_and_not_b.clone())), 1);
    let q = proof.modus_ponens(a_to_b, p); // a & ~b -> a -> b
    let r = proof.prove_axiom(Rc::new(axiom_2(a_and_not_b.clone(), a.clone(), b.clone())), 2);
    let s = proof.modus_ponens(q, r); // (a & ~b -> a) -> (a & ~b -> b)
    let t = proof.prove_axiom(Rc::new(axiom_3(a.clone(), not_b.clone())), 3);
    let u = proof.modus_ponens(t, s); // a & ~b -> b
    let v = proof.prove_axiom(Rc::new(axiom_4(a, not_b)), 4);
    let w = proof.prove_axiom(Rc::new(axiom_10(a_and_not_b, b)), 10);
    let x = proof.modus_ponens(u, w);
    proof.modus_ponens(v, x)
}

pub fn prove_a_nand_b(proof: &mut Proof, a_to_not_b: isize) -> isize {
    let a_to_not_b_pr = proof.prop(a_to_not_b).clone();
    let a = a_to_not_b_pr.premise().expect("invalid implication").clone();
    let not_b = a_to_not_b_pr.conclusion().expect("invalid implication").clone();
    let b = not_b.not_arg().expect("invalid negation");
    let a_and_b = Rc::new(Prop::And(a.clone(), b.clone()));
    let p = proof.prove_axiom(Rc::new(axiom_1(a_to_not_b_pr.clone(), a_and_b.clone())), 1);
    let q = proof.modus_ponens(a_to_not_b, p); // a & b -> a -> ~b
    let r = proof.prove_axiom(Rc::new(axiom_2(a_and_b.clone(), a.clone(), not_b.clone())), 2);
    let s = proof.modus_ponens(q, r); // (a & b -> a) -> (a & b -> ~b)
    let t = proof.prove_axiom(Rc::new(axiom_3(a.clone(), b.clone())), 3);
    let u = proof.modus_ponens(t, s); // a & b -> ~b
    let v = proof.prove_axiom(Rc::new(axiom_4(a, b.clone())), 4);
    let w = proof.prove_axiom(Rc::new(axiom_10(a_and_b, b.clone())), 10);
    let x = proof.modus_ponens(v, w);
    proof.modus_ponens(u, x)
}
