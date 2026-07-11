use arena_container::Arena;
use crate::prop::Prop;
use hashbrown::HashMap;
use std::fmt::{self, Display, Formatter, Debug};
use std::rc::Rc;

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
