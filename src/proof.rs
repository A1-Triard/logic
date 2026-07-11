use arena_container::Arena;
use crate::axioms::*;
use crate::prop::Prop;
use hashbrown::HashMap;
use std::fmt::{self, Display, Formatter, Debug};
use std::rc::Rc;

#[derive(Eq, PartialEq, Clone)]
pub enum Reason {
    Axiom(u8),
    Hypothesis,
    ModusPonens(isize, isize),
}

pub struct Proof {
    props: Arena<isize, (Rc<Prop>, Reason)>,
    list: Vec<isize>,
}

pub struct ProofDisplay<'p, 'a, A: Display> {
    proof: &'p Proof,
    atoms: &'a [A],
}

impl Proof {
    pub fn display<'p, 'a, A: Display>(&'p self, atoms: &'a [A]) -> ProofDisplay<'p, 'a, A> {
        ProofDisplay { proof: self, atoms }
    }

    pub fn new() -> Self {
        Proof {
            props: Arena::new(),
            list: Vec::new(),
        }
    }

    pub fn prove_axiom(&mut self, a: Rc<Prop>, n: u8) -> isize {
        let id = self.props.insert(|id| ((a, Reason::Axiom(n)), id));
        self.list.push(id);
        id
    }

    pub fn prove_hypothesis(&mut self, h: Rc<Prop>) -> isize {
        let id = self.props.insert(|id| ((h, Reason::Hypothesis), id));
        self.list.push(id);
        id
    }

    pub fn modus_ponens(&mut self, a: isize, a_to_b: isize) -> isize {
        let b = self.props[a_to_b].0.conclusion().expect("invalid modus ponens").clone();
        let id = self.props.insert(|id| ((b, Reason::ModusPonens(a, a_to_b)), id));
        self.list.push(id);
        id
    }

    pub fn prove_a_to_a(&mut self, a: Rc<Prop>) -> isize {
        let a_to_a = Rc::new(Prop::To(a.clone(), a.clone()));
        let p = self.prove_axiom(Rc::new(axiom_2(a.clone(), a_to_a.clone(), a.clone())), 2);
        let q = self.prove_axiom(Rc::new(axiom_1(a.clone(), a_to_a.clone())), 1);
        let r = self.modus_ponens(q, p);
        let s = self.prove_axiom(Rc::new(axiom_1(a.clone(), a)), 1);
        self.modus_ponens(s, r)
    }

    pub fn deduct(&mut self, proof: &Proof, h: &Rc<Prop>) -> Option<isize> {
        let mut map = HashMap::new();
        for &prop in &proof.list {
            let mapped_prop = match &proof.props[prop].1 {
                Reason::Hypothesis if &proof.props[prop].0 == h => {
                    self.prove_a_to_a(h.clone())
                },
                Reason::Hypothesis | Reason::Axiom(_) => {
                    let prop = &proof.props[prop];
                    let p = self.props.insert(|id| (prop.clone(), id));
                    self.list.push(p);
                    let q = self.prove_axiom(Rc::new(axiom_1(prop.0.clone(), h.clone())), 1);
                    self.modus_ponens(p, q)
                },
                &Reason::ModusPonens(a, a_to_b) => {
                    let a_st = &proof.props[a].0;
                    let b = proof.props[a_to_b].0.conclusion().expect("invalid MP");
                    let h_to_a = map[&a];
                    let h_to_a_to_b = map[&a_to_b];
                    let p = self.prove_axiom(Rc::new(axiom_2(h.clone(), a_st.clone(), b.clone())), 2);
                    let q = self.modus_ponens(h_to_a_to_b, p);
                    self.modus_ponens(h_to_a, q)
                },
            };
            map.insert(prop, mapped_prop);
        }
        self.list.last().copied()
    }
}

impl Debug for Proof {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut indices = HashMap::new();
        for (n, prop) in self.list.iter().copied().enumerate() {
            let n = n.wrapping_add(1);
            indices.insert(prop, n);
            let prop = &self.props[prop];
            write!(f, "{n}. {:?} (", prop.0)?;
            match prop.1 {
                Reason::Axiom(a) => write!(f, "A{a}")?,
                Reason::Hypothesis => write!(f, "H")?,
                Reason::ModusPonens(a, b) => write!(f, "MP {} {}", indices[&a], indices[&b])?,
            }
            writeln!(f, ")")?;
        }
        Ok(())
    }
}

impl<'p, 'a, A: Display> Display for ProofDisplay<'p, 'a, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut indices = HashMap::new();
        for (n, prop) in self.proof.list.iter().copied().enumerate() {
            let n = n.wrapping_add(1);
            indices.insert(prop, n);
            let prop = &self.proof.props[prop];
            write!(f, "{n}. {} (", prop.0.display(self.atoms))?;
            match prop.1 {
                Reason::Axiom(a) => write!(f, "A{a}")?,
                Reason::Hypothesis => write!(f, "H")?,
                Reason::ModusPonens(a, b) => write!(f, "MP {} {}", indices[&a], indices[&b])?,
            }
            writeln!(f, ")")?;
        }
        Ok(())
    }
}
