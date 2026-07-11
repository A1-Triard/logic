use hashbrown::HashMap;
use std::io::stdin;
use std::process::ExitCode;
use std::rc::Rc;

mod prop;
use prop::Prop;

mod axioms;
use axioms::*;

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
    println!("{}", axiom_1(a.clone(), b.clone()).display(&atoms));
    println!("{}", axiom_2(a.clone(), b.clone(), c.clone()).display(&atoms));
    println!("{}", axiom_3(a.clone(), b.clone()).display(&atoms));
    println!("{}", axiom_4(a.clone(), b.clone()).display(&atoms));
    println!("{}", axiom_5(a.clone(), b.clone()).display(&atoms));
    println!("{}", axiom_6(a.clone(), b.clone()).display(&atoms));
    println!("{}", axiom_7(a.clone(), b.clone()).display(&atoms));
    println!("{}", axiom_8(a.clone(), b.clone(), c).display(&atoms));
    println!("{}", axiom_9(a.clone(), b.clone()).display(&atoms));
    println!("{}", axiom_10(a.clone(), b).display(&atoms));
    println!("{}", axiom_11(a).display(&atoms));
    ExitCode::SUCCESS
}
