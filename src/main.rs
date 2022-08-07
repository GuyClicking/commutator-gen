// we are trying to find a comm for F R U R' U' F'
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
enum Move {
    R,
    R2,
    Rp,
    U,
    U2,
    Up,
    F,
    F2,
    Fp,
}
use Move::*;

impl Move {
    fn inverse(&self) -> Self {
        match self {
            R => Rp,
            R2 => R2,
            Rp => R,
            U => Up,
            U2 => U2,
            Up => U,
            F => Fp,
            F2 => F2,
            Fp => F,
        }
    }

    fn same_type(&self, mv: &Move) -> bool {
        let term = self.clone();
        match mv {
            R | R2 | Rp => term == R || term == R2 || term == Rp,
            U | U2 | Up => term == U || term == U2 || term == Up,
            F | F2 | Fp => term == F || term == F2 || term == Fp,
        }
    }

    fn apply_same_type(&self, mv: &Move) -> Option<Move> {
        match self {
            R => match mv {
                R => Some(R2),
                R2 => Some(Rp),
                Rp => None,
                _ => unreachable!(),
            },
            R2 => match mv {
                R => Some(Rp),
                R2 => None,
                Rp => Some(R),
                _ => unreachable!(),
            },
            Rp => match mv {
                R => None,
                R2 => Some(R),
                Rp => Some(R2),
                _ => unreachable!(),
            },
            U => match mv {
                U => Some(U2),
                U2 => Some(Up),
                Up => None,
                _ => unreachable!(),
            },
            U2 => match mv {
                U => Some(Up),
                U2 => None,
                Up => Some(U),
                _ => unreachable!(),
            },
            Up => match mv {
                U => None,
                U2 => Some(U),
                Up => Some(U2),
                _ => unreachable!(),
            },
            F => match mv {
                F => Some(F2),
                F2 => Some(Fp),
                Fp => None,
                _ => unreachable!(),
            },
            F2 => match mv {
                F => Some(Fp),
                F2 => None,
                Fp => Some(F),
                _ => unreachable!(),
            },
            Fp => match mv {
                F => None,
                F2 => Some(F),
                Fp => Some(F2),
                _ => unreachable!(),
            },
        }
    }
}

const MOVES: [Move; 9] = [R, R2, Rp, U, U2, Up, F, F2, Fp];

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
        let mut r = vec![];
        let mut a = a.clone();
        let mut ap = a.clone().inverse();
        let mut b = b.clone();
        let mut bp = b.clone().inverse();
        r.append(&mut a.moves);
        r.append(&mut b.moves);
        r.append(&mut ap.moves);
        r.append(&mut bp.moves);
        Sequence { moves: r }.simplify()
    }

    fn simplify(mut self) -> Self {
        // use joey's implementation which creates a new vector
        let mut i: isize = 0;
        while ((i + 1) as usize) < self.len() {
            let mut increment = true;
            if self.moves[i as usize].same_type(&self.moves[i as usize + 1]) {
                if let Some(new_move) =
                    self.moves[i as usize].apply_same_type(&self.moves[i as usize + 1])
                {
                    self.moves[i as usize] = new_move;
                    self.moves.remove(i as usize + 1);
                } else {
                    self.moves.remove(i as usize);
                    self.moves.remove(i as usize);
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
        self
    }

    fn inverse(self) -> Self {
        let mut r = vec![];
        for mv in self.moves.into_iter().rev() {
            r.push(mv.inverse());
        }
        Sequence { moves: r }
    }

    fn push(&mut self, mv: Move) {
        self.moves.push(mv)
    }

    fn len(&self) -> usize {
        self.moves.len()
    }

    fn ends_with_type(&self, mv: &Move) -> bool {
        if self.len() == 0 {
            return false;
        }
        // dies
        let end = self.moves[self.len() - 1].clone();
        end.same_type(mv)
    }
}

// This explodes ram
fn find_comm_1(sol: Sequence) -> (Sequence, Sequence) {
    let mut queue: VecDeque<(Sequence, Sequence)> = VecDeque::new();
    queue.push_back((Sequence::empty(), Sequence::empty()));

    let mut nodes = 0;
    while let Some((a, b)) = queue.pop_front() {
        nodes += 1;
        if nodes % 32768 == 0 {
            println!("{} nodes", nodes);
            println!("{:?} {:?}", a, b);
        }
        if sol == Sequence::from_comm(&a, &b) {
            return (a, b);
        }
        for mv in MOVES {
            // clones are ugly! yikes!
            if !a.ends_with_type(&mv) {
                let mut a = a.clone();
                a.push(mv.clone());
                queue.push_back((a, b.clone()));
            }
            if !b.ends_with_type(&mv) {
                let mut b = b.clone();
                b.push(mv);
                queue.push_back((a.clone(), b));
            }
        }
    }

    unreachable!()
}

fn find_comm_2_search(
    sol: &Sequence,
    a: Sequence,
    b: Sequence,
    depth: usize,
    nodes: &mut usize,
) -> Option<(Sequence, Sequence)> {
    *nodes += 1;
    if *nodes % 32768 == 0 {
        println!("{} nodes", nodes);
    }
    if depth == 0 {
        if *sol == Sequence::from_comm(&a, &b) {
            return Some((a, b));
        } else {
            return None;
        }
    }
    for mv in MOVES {
        // clones are ugly! yikes!
        if !a.ends_with_type(&mv) {
            let mut a = a.clone();
            a.push(mv.clone());
            if let Some((a, b)) = find_comm_2_search(sol, a, b.clone(), depth - 1, nodes) {
                return Some((a, b));
            }
        }
        if !b.ends_with_type(&mv) {
            let mut b = b.clone();
            b.push(mv);
            if let Some((a, b)) = find_comm_2_search(sol, a.clone(), b, depth - 1, nodes) {
                return Some((a, b));
            }
        }
    }
    None
}

fn find_comm_2(sol: Sequence) -> (Sequence, Sequence) {
    let mut nodes = 0;
    for i in 1.. {
        if let Some(sol) =
            find_comm_2_search(&sol, Sequence::empty(), Sequence::empty(), i, &mut nodes)
        {
            println!("Solved in {} nodes!", nodes);
            return sol;
        }
    }
    unreachable!()
}

fn main() {
    let moves = Sequence {
        moves: vec![F, R, U, Rp, Up, Fp],
    };
    let (a, b) = find_comm_2(moves);

    println!("[{:?}, {:?}]", a, b);
}
