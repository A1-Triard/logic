use crate::axioms::*;
use crate::proof::Proof;
use crate::prop::Prop;
use std::rc::Rc;

pub fn prove_a_to_a(a: Rc<Prop>, proof: &mut Proof) -> isize {
    let a_to_a = Rc::new(Prop::To(a.clone(), a.clone()));
    let p = proof.prove_axiom(Rc::new(axiom_2(a.clone(), a_to_a.clone(), a.clone())), 2);
    let q = proof.prove_axiom(Rc::new(axiom_1(a.clone(), a_to_a.clone())), 1);
    let r = proof.modus_ponens(q, p);
    let s = proof.prove_axiom(Rc::new(axiom_1(a.clone(), a)), 1);
    proof.modus_ponens(s, r)
}
