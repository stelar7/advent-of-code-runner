use itertools::Itertools;
use std::io::Read;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum State {
    Empty,
    O,
    ON,
    T,
    TW,
    TH,
    THR,
    THRE,
    F,
    FO,
    FOU,
    FI,
    FIV,
    S,
    SI,
    SE,
    SEV,
    SEVE,
    E,
    EI,
    EIG,
    EIGH,
    N,
    NI,
    NIN,
}

fn transition(state: State, b: u8) -> (u8, State) {
    match (state, b) {
        (State::O | State::FO, b'n') => (0, State::ON),
        (State::T, b'w') => (0, State::TW),
        (State::T, b'h') => (0, State::TH),
        (State::F, b'o') => (0, State::FO),
        (State::F, b'i') => (0, State::FI),
        (State::S, b'i') => (0, State::SI),
        (State::S, b'e') => (0, State::SE),
        (State::E | State::SE | State::THRE | State::SEVE, b'i') => (0, State::EI),
        (State::N | State::ON | State::NIN, b'i') => (0, State::NI),
        (State::ON, b'e') => (1, State::E),
        (State::TW, b'o') => (2, State::O),
        (State::TH, b'r') => (0, State::THR),
        (State::FO, b'u') => (0, State::FOU),
        (State::FI, b'v') => (0, State::FIV),
        (State::SI, b'x') => (6, State::Empty),
        (State::SE, b'v') => (0, State::SEV),
        (State::EI, b'g') => (0, State::EIG),
        (State::NI, b'n') => (0, State::NIN),
        (State::THR, b'e') => (3, State::THRE),
        (State::FOU, b'r') => (4, State::Empty),
        (State::FIV, b'e') => (5, State::E),
        (State::SEV, b'e') => (0, State::SEVE),
        (State::EIG, b'h') => (0, State::EIGH),
        (State::NIN, b'e') => (9, State::E),
        (State::THRE, b'e') => (3, State::E),
        (State::SEVE, b'n') => (7, State::N),
        (State::EIGH, b't') => (8, State::T),
        (_, b'o') => (0, State::O),
        (_, b't') => (0, State::T),
        (_, b'f') => (0, State::F),
        (_, b's') => (0, State::S),
        (_, b'e') => (0, State::E),
        (_, b'n') => (0, State::N),
        (_, b'1') => (1, State::Empty),
        (_, b'2') => (2, State::Empty),
        (_, b'3') => (3, State::Empty),
        (_, b'4') => (4, State::Empty),
        (_, b'5') => (5, State::Empty),
        (_, b'6') => (6, State::Empty),
        (_, b'7') => (7, State::Empty),
        (_, b'8') => (8, State::Empty),
        (_, b'9') => (9, State::Empty),
        _ => (0, State::Empty),
    }
}

fn main() {
    let (p1, p2) = std::io::stdin()
        .lock()
        .bytes()
        .map(Result::unwrap)
        .batching(|it| {
            let mut it = it.peekable();
            it.peek()?;

            let mut state = State::Empty;
            let mut a1 = 0;
            let mut z1 = 0;
            let mut a2 = 0;
            let mut z2 = 0;
            while let Some(b) = it.next() {
                if !char::is_ascii_alphanumeric(&b.into()) {
                    let x1 = 10 * a1 + z1;
                    let x2 = 10 * a2 + z2;
                    return Some((x1, x2));
                }

                // p1
                if char::is_ascii_digit(&b.into()) {
                    let val = b - b'0';
                    if 0 == a1 {
                        a1 = val
                    }
                    z1 = val;
                }
                // p2
                {
                    let (val, new_state) = transition(state, b);
                    state = new_state;
                    if 0 != val {
                        if 0 == a2 {
                            a2 = val;
                        }
                        z2 = val;
                    }
                }
            }
            None
        })
        .fold((0, 0), |(p1, p2), (x1, x2)| {
            (p1 + x1 as u32, p2 + x2 as u32)
        });
    println!("{}\n{}", p1, p2);
}
