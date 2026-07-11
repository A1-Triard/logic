use hashbrown::HashMap;
use std::io::stdin;
use std::process::ExitCode;
use std::rc::Rc;

mod prop;
use prop::Prop;

mod axioms;

mod proof;
use proof::Proof;

mod proofs;
use proofs::*;

fn read_prop<'a>(
    line: &'a mut String, atoms: (&mut Vec<&'a str>, &mut HashMap<&'a str, usize>)
) -> Result<Prop, ExitCode> {
    if let Err(e) = stdin().read_line(line) {
        eprintln!("{e}");
        return Err(ExitCode::from(2));
    }
    match Prop::from_str(line.trim_end(), atoms) {
        Ok(prop) => Ok(prop),
        Err(_) => {
            eprintln!("Invalid proposition");
            Err(ExitCode::from(1))
        },
    }
}

fn main() -> ExitCode {
    let mut line_1 = String::new();
    let mut line_2 = String::new();
    let mut line_3 = String::new();
    let mut atoms = Vec::new();
    let mut names = HashMap::new();
    let a = match read_prop(&mut line_1, (&mut atoms, &mut names)) {
        Ok(p) => Rc::new(p),
        Err(e) => return e,
    };
    let b = match read_prop(&mut line_2, (&mut atoms, &mut names)) {
        Ok(p) => Rc::new(p),
        Err(e) => return e,
    };
    let c = match read_prop(&mut line_3, (&mut atoms, &mut names)) {
        Ok(p) => Rc::new(p),
        Err(e) => return e,
    };
    let mut proof = Proof::new();
    let a_to_b = proof.prove_hypothesis(Rc::new(Prop::To(a.clone(), b.clone())));
    prove_not_a_or_b(&mut proof, a_to_b);
    print!("{}", proof.display(&atoms));
    ExitCode::SUCCESS
}
