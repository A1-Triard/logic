use hashbrown::HashMap;
use std::io::stdin;
use std::process::ExitCode;

mod prop;
use prop::*;

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
    let mut atoms = Vec::new();
    let mut names = HashMap::new();
    let a = match read_prop(&mut line_1, (&mut atoms, &mut names)) {
        Ok(p) => p,
        Err(e) => return e,
    };
    println!("{}", a.display(&atoms));
    ExitCode::SUCCESS
}
