use crate::prop::Prop;
use std::rc::Rc;

pub fn axiom_1(a: Rc<Prop>, b: Rc<Prop>) -> Prop {
    Prop::To(
        a.clone(),
        Rc::new(Prop::To(b, a)),
    )
}

pub fn axiom_2(a: Rc<Prop>, b: Rc<Prop>, c: Rc<Prop>) -> Prop {
    Prop::To(
        Rc::new(Prop::To(a.clone(), Rc::new(Prop::To(b.clone(), c.clone())))),
        Rc::new(Prop::To(
            Rc::new(Prop::To(a.clone(), b)),
            Rc::new(Prop::To(a, c)),
        )),
    )
}

pub fn axiom_3(a: Rc<Prop>, b: Rc<Prop>) -> Prop {
    Prop::To(Rc::new(Prop::And(a.clone(), b)), a)
}

pub fn axiom_4(a: Rc<Prop>, b: Rc<Prop>) -> Prop {
    Prop::To(Rc::new(Prop::And(a, b.clone())), b)
}

pub fn axiom_5(a: Rc<Prop>, b: Rc<Prop>) -> Prop {
    Prop::To(
        a.clone(),
        Rc::new(Prop::To(b.clone(), Rc::new(Prop::And(a, b)))),
    )
}

pub fn axiom_6(a: Rc<Prop>, b: Rc<Prop>) -> Prop {
    Prop::To(a.clone(), Rc::new(Prop::Or(a, b)))
}

pub fn axiom_7(a: Rc<Prop>, b: Rc<Prop>) -> Prop {
    Prop::To(b.clone(), Rc::new(Prop::Or(a, b)))
}

pub fn axiom_8(a: Rc<Prop>, b: Rc<Prop>, c: Rc<Prop>) -> Prop {
    Prop::To(
        Rc::new(Prop::To(a.clone(), c.clone())),
        Rc::new(Prop::To(
            Rc::new(Prop::To(b.clone(), c.clone())),
            Rc::new(Prop::To(Rc::new(Prop::Or(a, b)), c)),
        )),
    )
}

pub fn axiom_9(a: Rc<Prop>, b: Rc<Prop>) -> Prop {
    Prop::To(
        Rc::new(Prop::Not(a.clone())),
        Rc::new(Prop::To(a, b)),
    )
}

pub fn axiom_10(a: Rc<Prop>, b: Rc<Prop>) -> Prop {
    Prop::To(
        Rc::new(Prop::To(a.clone(), b.clone())),
        Rc::new(Prop::To(
            Rc::new(Prop::To(a.clone(), Rc::new(Prop::Not(b)))),
            Rc::new(Prop::Not(a)),
        )),
    )
}

pub fn axiom_11(a: Rc<Prop>) -> Prop {
    Prop::Or(a.clone(), Rc::new(Prop::Not(a)))
}
