use std::{fmt::Debug, fs, vec};

struct Crate(String);

struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Moving {} crates from {} to {}",
            self.amount, self.from, self.to
        )
    }
}

impl Debug for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn read_initial_crates() -> Vec<Vec<Crate>> {
    let input: String = fs::read_to_string(String::from("cargo_initial.txt")).expect("Wrong path");
    let mut crates: Vec<Vec<Crate>> = Vec::new();

    for _ in 1..10 {
        crates.push(vec![]);
    }

    for line in input.lines() {
        let mut crate_index: usize = 0;

        for (i, character) in line.chars().enumerate() {
            if character != ' ' && character != '[' && character != ']' && !character.is_numeric() {
                match crates.get_mut(crate_index) {
                    Some(stack) => stack.push(Crate(character.to_string())),
                    None => {
                        let mut v = Vec::new();
                        v.push(Crate(character.to_string()));
                        crates[crate_index] = v;
                    }
                }
            }

            if i % 4 == 1 {
                crate_index += 1;
            }
        }
    }
    crates
}

fn read_instructions() -> Vec<Move> {
    let input: String =
        fs::read_to_string(String::from("moves.txt")).expect("Moves file not found");
    let mut moves: Vec<Move> = vec![];

    for line in input.lines() {
        let parsed_line: Vec<&str> = line.split(' ').collect();
        let new_move = Move {
            from: parsed_line[3].parse().unwrap_or(0),
            to: parsed_line[5].parse().unwrap_or(0),
            amount: parsed_line[1].parse().unwrap_or(0),
        };
        moves.push(new_move);
    }

    moves
}

fn execute_move(instr: Move, store: &mut Vec<Vec<Crate>>) {
    for _ in 0..instr.amount {
        let removed_value = store.get_mut(instr.from - 1).unwrap().remove(0);
        store
            .get_mut(instr.to - 1)
            .expect("Index out of bounds")
            .insert(0, removed_value);
    }
}

fn execute_better_move(instr: Move, store: &mut Vec<Vec<Crate>>) {
    let mut moved_crates: Vec<Crate> = vec![];

    for _ in 0..instr.amount {
        let removed_value = store.get_mut(instr.from - 1).unwrap().remove(0);
        moved_crates.insert(0, removed_value);
    }

    for c in moved_crates.into_iter() {
        store
            .get_mut(instr.to - 1)
            .expect("Wrong index")
            .insert(0, c);
    }
}

fn main() {
    let mut crates = read_initial_crates();
    let moves = read_instructions();

    for instr in moves {
        execute_better_move(instr, &mut crates);
    }

    for stack in crates.iter() {
        print!("{}", stack[0].0);
    }

    print!("{:?}", crates);
}
