use hashbrown::HashMap;
use hashbrown::hash_map::Entry;
use std::fmt::{self, Display, Formatter, Debug};
use std::rc::Rc;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Prop {
    Atom(usize),
    Not(Rc<Prop>),
    To(Rc<Prop>, Rc<Prop>),
    And(Rc<Prop>, Rc<Prop>),
    Or(Rc<Prop>, Rc<Prop>),
}

fn debug_unop(
    op: &'static str, el: i8, arg: &Prop, f: &mut Formatter<'_>
) -> fmt::Result {
    write!(f, "{}", op)?;
    let arg_el = arg.expr_level();
    if arg_el > el { write!(f, "(")?; }
    write!(f, "{arg:?}")?;
    if arg_el > el { write!(f, ")")?; }
    Ok(())
}

fn debug_binop(
    op: &'static str, el: i8, right_assoc: bool, arg_1: &Prop, arg_2: &Prop, f: &mut Formatter<'_>
) -> fmt::Result {
    let arg_1_el = arg_1.expr_level();
    let arg_2_el = arg_2.expr_level();
    let arg_1_p = right_assoc && arg_1_el >= el || !right_assoc && arg_1_el > el;
    let arg_2_p = right_assoc && arg_2_el > el || !right_assoc && arg_2_el >= el;
    if arg_1_p { write!(f, "(")?; }
    write!(f, "{arg_1:?}")?;
    if arg_1_p { write!(f, ")")?; }
    write!(f, " {} ", op)?;
    if arg_2_p { write!(f, "(")?; }
    write!(f, "{arg_2:?}")?;
    if arg_2_p { write!(f, ")")?; }
    Ok(())
}

impl Debug for Prop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            &Prop::Atom(a) => write!(f, "{a}"),
            Prop::Not(a) =>
                debug_unop("~", self.expr_level(), a.as_ref(), f),
            Prop::And(a, b) =>
                debug_binop("&", self.expr_level(), false, a.as_ref(), b.as_ref(), f),
            Prop::Or(a, b) =>
                debug_binop("|", self.expr_level(), false, a.as_ref(), b.as_ref(), f),
            Prop::To(a, b) =>
                debug_binop("->", self.expr_level(), true, a.as_ref(), b.as_ref(), f),
        }
    }
}

pub struct PropDisplay<'p, 'a, A: Display> {
    prop: &'p Prop,
    atoms: &'a [A],
}

impl Prop {
    pub fn display<'p, 'a, A: Display>(&'p self, atoms: &'a [A]) -> PropDisplay<'p, 'a, A> {
        PropDisplay { prop: self, atoms }
    }

    fn expr_level(&self) -> i8 {
        match self {
            Prop::Atom(_) => 0,
            Prop::Not(_) => 1,
            Prop::And(_, _) => 2,
            Prop::Or(_, _) => 3,
            Prop::To(_, _) => 4,
        }
    }

    pub fn from_str<'a>(
        s: &'a str, atoms: (&mut Vec<&'a str>, &mut HashMap<&'a str, usize>)
    ) -> Result<Self, InvalidPropError> {
        let mut parser = Parser {
            scanner: Scanner { s },
            next_token: None,
        };
        parser.skip_token()?;
        parser.parse_prop(atoms)
    }

    pub fn premise(&self) -> Option<&Rc<Prop>> {
        match self {
            Prop::To(x, _) => Some(x),
            _ => None,
        }
    }

    pub fn conclusion(&self) -> Option<&Rc<Prop>> {
        match self {
            Prop::To(_, x) => Some(x),
            _ => None,
        }
    }

    pub fn not_arg(&self) -> Option<&Rc<Prop>> {
        match self {
            Prop::Not(x) => Some(x),
            _ => None,
        }
    }
}

fn display_unop<A: Display>(
    op: &'static str, el: i8, arg: &Prop, atoms: &[A], f: &mut Formatter<'_>
) -> fmt::Result {
    write!(f, "{}", op)?;
    let arg_el = arg.expr_level();
    if arg_el > el { write!(f, "(")?; }
    write!(f, "{}", arg.display(atoms))?;
    if arg_el > el { write!(f, ")")?; }
    Ok(())
}

fn display_binop<A: Display>(
    op: &'static str, el: i8, right_assoc: bool, arg_1: &Prop, arg_2: &Prop, atoms: &[A], f: &mut Formatter<'_>
) -> fmt::Result {
    let arg_1_el = arg_1.expr_level();
    let arg_2_el = arg_2.expr_level();
    let arg_1_p = right_assoc && arg_1_el >= el || !right_assoc && arg_1_el > el;
    let arg_2_p = right_assoc && arg_2_el > el || !right_assoc && arg_2_el >= el;
    if arg_1_p { write!(f, "(")?; }
    write!(f, "{}", arg_1.display(atoms))?;
    if arg_1_p { write!(f, ")")?; }
    write!(f, " {} ", op)?;
    if arg_2_p { write!(f, "(")?; }
    write!(f, "{}", arg_2.display(atoms))?;
    if arg_2_p { write!(f, ")")?; }
    Ok(())
}

impl<'p, 'a, A: Display> Display for PropDisplay<'p, 'a, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.prop {
            &Prop::Atom(a) => write!(f, "{}", self.atoms[a]),
            Prop::Not(a) =>
                display_unop("~", self.prop.expr_level(), a.as_ref(), self.atoms, f),
            Prop::And(a, b) =>
                display_binop("&", self.prop.expr_level(), false, a.as_ref(), b.as_ref(), self.atoms, f),
            Prop::Or(a, b) =>
                display_binop("|", self.prop.expr_level(), false, a.as_ref(), b.as_ref(), self.atoms, f),
            Prop::To(a, b) =>
                display_binop("->", self.prop.expr_level(), true, a.as_ref(), b.as_ref(), self.atoms, f),
        }
    }
}

#[derive(Eq, PartialEq)]
enum Token<'a> {
    Atom(&'a str),
    Not,
    And,
    Or,
    To,
    LeftParen,
    RightParen,
}

pub struct InvalidPropError;

struct Scanner<'a> {
    s: &'a str,
}

impl<'a> Scanner<'a> {
    fn scan_to(&mut self) -> Result<Option<Token<'a>>, InvalidPropError> {
        let Some(c) = self.s.chars().next() else { return Err(InvalidPropError); };
        if c == '>' {
            self.s = &self.s[1 ..];
            Ok(Some(Token::To))
        } else {
            Err(InvalidPropError)
        }
    }

    fn scan_atom(&mut self) -> Result<Option<Token<'a>>, InvalidPropError> {
        let mut atom_len = 0;
        let mut chars = self.s.chars();
        loop {
            let Some(c) = chars.next() else { break; };
            match c {
                '~' | '&' | '|' | '-' | '(' | ')' => break,
                _ if c.is_whitespace() => break,
                _ => atom_len += c.len_utf8(),
            }
        }
        let res = Token::Atom(&self.s[.. atom_len]);
        self.s = &self.s[atom_len ..];
        Ok(Some(res))
    }

    fn scan_token(&mut self) -> Result<Option<Token<'a>>, InvalidPropError> {
        let c = loop {
            let Some(c) = self.s.chars().next() else { return Ok(None); };
            if !c.is_whitespace() { break c; }
            self.s = &self.s[c.len_utf8() ..];
        };
        match c {
            '~' => { self.s = &self.s[1 ..]; Ok(Some(Token::Not)) },
            '&' => { self.s = &self.s[1 ..]; Ok(Some(Token::And)) },
            '|' => { self.s = &self.s[1 ..]; Ok(Some(Token::Or)) },
            '-' => { self.s = &self.s[1 ..]; self.scan_to() },
            '(' => { self.s = &self.s[1 ..]; Ok(Some(Token::LeftParen)) },
            ')' => { self.s = &self.s[1 ..]; Ok(Some(Token::RightParen)) },
            _ => self.scan_atom(),
        }
    }
}

struct Parser<'a> {
    scanner: Scanner<'a>,
    next_token: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    fn skip_token(&mut self) -> Result<(), InvalidPropError> {
        self.next_token = self.scanner.scan_token()?;
        Ok(())
    }

    fn parse_negation(
        &mut self, atoms: (&mut Vec<&'a str>, &mut HashMap<&'a str, usize>)
    ) -> Result<Prop, InvalidPropError> {
        match &self.next_token {
            Some(Token::Not) => {
                self.skip_token()?;
                let a = self.parse_negation(atoms)?;
                Ok(Prop::Not(Rc::new(a)))
            },
            Some(Token::Atom(a)) => {
                let a = match atoms.1.entry(a) {
                    Entry::Occupied(e) => *e.get(),
                    Entry::Vacant(e) => {
                        let i = atoms.0.len();
                        atoms.0.push(a);
                        e.insert(i);
                        i
                    },
                };
                self.skip_token()?;
                Ok(Prop::Atom(a))
            },
            Some(Token::LeftParen) => {
                self.skip_token()?;
                let a = self.parse_implication(atoms)?;
                if self.next_token != Some(Token::RightParen) { return Err(InvalidPropError); }
                self.skip_token()?;
                Ok(a)
            },
            _ => Err(InvalidPropError),
        }
    }

    fn parse_conjunction(
        &mut self, atoms: (&mut Vec<&'a str>, &mut HashMap<&'a str, usize>)
    ) -> Result<Prop, InvalidPropError> {
        let mut a = self.parse_negation((atoms.0, atoms.1))?;
        loop {
            if self.next_token != Some(Token::And) { break; }
            self.skip_token()?;
            let b = self.parse_negation((atoms.0, atoms.1))?;
            a = Prop::And(Rc::new(a), Rc::new(b));
        }
        Ok(a)
    }

    fn parse_disjunction(
        &mut self, atoms: (&mut Vec<&'a str>, &mut HashMap<&'a str, usize>)
    ) -> Result<Prop, InvalidPropError> {
        let mut a = self.parse_conjunction((atoms.0, atoms.1))?;
        loop {
            if self.next_token != Some(Token::Or) { break; }
            self.skip_token()?;
            let b = self.parse_conjunction((atoms.0, atoms.1))?;
            a = Prop::Or(Rc::new(a), Rc::new(b));
        }
        Ok(a)
    }

    fn parse_implication(
        &mut self, atoms: (&mut Vec<&'a str>, &mut HashMap<&'a str, usize>)
    ) -> Result<Prop, InvalidPropError> {
        let a = self.parse_disjunction((atoms.0, atoms.1))?;
        if self.next_token != Some(Token::To) { return Ok(a); }
        self.skip_token()?;
        let b = self.parse_implication((atoms.0, atoms.1))?;
        Ok(Prop::To(Rc::new(a), Rc::new(b)))
    }

    fn parse_prop(
        &mut self, atoms: (&mut Vec<&'a str>, &mut HashMap<&'a str, usize>)
    ) -> Result<Prop, InvalidPropError> {
        let prop = self.parse_implication((atoms.0, atoms.1))?;
        if self.next_token.is_some() { return Err(InvalidPropError); }
        Ok(prop)
    }
}
