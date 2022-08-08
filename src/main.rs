// we are trying to find a comm for F R U R' U' F'
#[derive(Debug, Clone, Copy, PartialEq)]
enum MoveType {
    R,
    U,
    F,
    D,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MoveDir {
    Clockwise,
    Double,
    Anticlockwise,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Move {
    ty: MoveType,
    dir: MoveDir,
}

#[allow(dead_code)]
const R: Move = Move {
    ty: MoveType::R,
    dir: MoveDir::Clockwise,
};
#[allow(dead_code)]
const R2: Move = Move {
    ty: MoveType::R,
    dir: MoveDir::Double,
};
const RP: Move = Move {
    ty: MoveType::R,
    dir: MoveDir::Anticlockwise,
};
#[allow(dead_code)]
const U: Move = Move {
    ty: MoveType::U,
    dir: MoveDir::Clockwise,
};
#[allow(dead_code)]
const U2: Move = Move {
    ty: MoveType::U,
    dir: MoveDir::Double,
};
#[allow(dead_code)]
const UP: Move = Move {
    ty: MoveType::U,
    dir: MoveDir::Anticlockwise,
};
#[allow(dead_code)]
const F: Move = Move {
    ty: MoveType::F,
    dir: MoveDir::Clockwise,
};
#[allow(dead_code)]
const F2: Move = Move {
    ty: MoveType::F,
    dir: MoveDir::Double,
};
#[allow(dead_code)]
const FP: Move = Move {
    ty: MoveType::F,
    dir: MoveDir::Anticlockwise,
};
#[allow(dead_code)]
const D: Move = Move {
    ty: MoveType::D,
    dir: MoveDir::Clockwise,
};
#[allow(dead_code)]
const D2: Move = Move {
    ty: MoveType::D,
    dir: MoveDir::Double,
};
#[allow(dead_code)]
const DP: Move = Move {
    ty: MoveType::D,
    dir: MoveDir::Anticlockwise,
};

impl Move {
    fn inverse(&self) -> Self {
        let dir = match self.dir {
            MoveDir::Clockwise => MoveDir::Anticlockwise,
            MoveDir::Double => MoveDir::Double,
            MoveDir::Anticlockwise => MoveDir::Clockwise,
        };
        Move { ty: self.ty, dir }
    }

    fn same_type(&self, mv: &Move) -> bool {
        self.ty == mv.ty
    }

    fn apply_same_type(&self, mv: &Move) -> Option<Move> {
        assert_eq!(self.ty, mv.ty);
        let dir = match self.dir {
            MoveDir::Clockwise => match mv.dir {
                MoveDir::Clockwise => MoveDir::Double,
                MoveDir::Double => MoveDir::Anticlockwise,
                MoveDir::Anticlockwise => return None,
            },
            MoveDir::Double => match mv.dir {
                MoveDir::Clockwise => MoveDir::Anticlockwise,
                MoveDir::Double => return None,
                MoveDir::Anticlockwise => MoveDir::Clockwise,
            },
            MoveDir::Anticlockwise => match mv.dir {
                MoveDir::Clockwise => return None,
                MoveDir::Double => MoveDir::Clockwise,
                MoveDir::Anticlockwise => MoveDir::Double,
            },
        };

        Some(Move { ty: self.ty, dir })
    }
}

const MOVES: [Move; 9] = [R, R2, RP, U, U2, UP, F, F2, FP];

// TODO write funky formatting
#[derive(Debug, Clone, PartialEq)]
struct Sequence {
    moves: Vec<Move>,
}

impl Sequence {
    fn empty() -> Self {
        Sequence { moves: vec![] }
    }

    fn from_comm(a: &Sequence, b: &Sequence) -> Self {
        let mut r = Sequence {
            moves: Vec::with_capacity(2 * a.moves.len() + 2 * b.moves.len()),
        };
        r.append(a);
        r.append(b);
        r.append_inverse(a);
        r.append_inverse(b);
        r.simplify();
        r
    }

    fn simplify(&mut self) {
        // use joey's implementation which creates a new vector maybe?
        let mut i = 0;
        while i + 1 < self.len() {
            let mut increment = true;
            if self.moves[i].same_type(&self.moves[i + 1]) {
                if let Some(new_move) = self.moves[i].apply_same_type(&self.moves[i + 1]) {
                    self.moves[i as usize] = new_move;
                    self.moves.remove(i + 1);
                } else {
                    self.moves.remove(i);
                    self.moves.remove(i);
                }
                if i > 1 {
                    i -= 1;
                }
                increment = false;
            }

            if increment {
                i += 1;
            }
        }
    }

    fn push(&mut self, mv: Move) {
        self.moves.push(mv)
    }

    fn append(&mut self, s: &Sequence) {
        for mv in &s.moves {
            self.push(*mv);
        }
    }

    fn append_inverse(&mut self, s: &Sequence) {
        for mv in s.moves.iter().rev() {
            self.push(mv.inverse());
        }
    }

    fn len(&self) -> usize {
        self.moves.len()
    }

    fn ends_with_type(&self, mv: &Move) -> bool {
        if self.len() == 0 {
            return false;
        }
        // dies
        let end = self.moves[self.len() - 1];
        end.same_type(mv)
    }
}

fn find_comm_search(
    sol: &Sequence,
    a: &mut Sequence,
    b: &mut Sequence,
    depth: usize,
    nodes: &mut usize,
    b_mode: bool,
) -> Option<(Sequence, Sequence)> {
    *nodes += 1;
    //if *nodes % 32768 == 0 {
    //println!("{} nodes", nodes);
    //}
    if depth == 0 {
        if b_mode && *sol == Sequence::from_comm(a, b) {
            return Some((a.clone(), b.clone()));
        } else {
            return None;
        }
    }
    for mv in MOVES {
        if !b_mode && !a.ends_with_type(&mv) {
            a.push(mv);
            if a.moves[0].ty != sol.moves[0].ty {
                a.moves.pop();
                continue;
            }
            if let Some((a, b)) = find_comm_search(sol, a, b, depth - 1, nodes, false) {
                return Some((a, b));
            }
            if let Some((a, b)) = find_comm_search(sol, a, b, depth - 1, nodes, true) {
                return Some((a, b));
            }
            a.moves.pop();
        } else if b_mode && !b.ends_with_type(&mv) {
            b.push(mv);
            if let Some((a, b)) = find_comm_search(sol, a, b, depth - 1, nodes, true) {
                return Some((a, b));
            }
            b.moves.pop();
        }
    }
    None
}

fn find_comm(sol: Sequence) -> (Sequence, Sequence) {
    let mut nodes = 0;
    for i in 1.. {
        println!("Depth {}", i);
        if let Some(sol) = find_comm_search(
            &sol,
            &mut Sequence::empty(),
            &mut Sequence::empty(),
            i,
            &mut nodes,
            false,
        ) {
            println!("Solved in {} nodes!", nodes);
            return sol;
        }
    }
    unreachable!()
}

fn main() {
    let moves = Sequence {
        //moves: vec![R2, F, RP, U, R, FP, R2, U, R, U2 RP],
        moves: vec![F, R, U, RP, UP, FP],
        //moves: vec![R, U, RP, UP, F, D, FP, DP],
        //moves: vec![R2, U2, R2, U2, R, U, RP, UP],
    };
    let (a, b) = find_comm(moves);

    println!("[{:?}, {:?}]", a, b);
}
