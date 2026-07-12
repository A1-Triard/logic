pub struct TruthTable {
    atoms_count: usize,
    values: Vec<bool>,
}

struct TruthTableIndex(Vec<bool>);

impl TruthTableIndex {
    fn new(atoms_count: usize) -> Self {
        TruthTableIndex(vec![false; atoms_count])
    }

    fn next(&mut self) -> bool {
        for p in &mut self.0 {
            if *p {
                *p = false;
            } else {
                *p = true;
                return true;
            }
        }
        false
    }
}

impl TruthTable {
    pub fn new(atoms_count: usize, f: &impl Fn(&[bool]) -> bool) -> Self {
        let mut values = Vec::new();
        let mut index = TruthTableIndex::new(atoms_count);
        loop {
            values.push(f(&index.0));
            if !index.next() { break; }
        }
        TruthTable { atoms_count, values }
    }

    pub fn is_tautology(&self) -> bool {
        self.values.iter().copied().all(|x| x)
    }

    pub fn is_contradiction(&self) -> bool {
        self.values.iter().copied().all(|x| !x)
    }
}
