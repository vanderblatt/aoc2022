const ROWS: usize = 6;
const COLS: usize = 40;
const CYCLES: usize = ROWS * COLS;

enum Cmd {
    Nop,
    Addx(isize),
}

struct State {
    x: usize,
    cycle: usize,
    buffer: [u8; CYCLES],
}

impl State {
    fn new() -> Self {
        State {
            x: 1,
            cycle: 0,
            buffer: [0; CYCLES],
        }
    }
}

fn parse_cmd(line: &str) -> Cmd {
    match &line[..4] {
        "noop" => Cmd::Nop,
        "addx" => {
            const PREFIX_LEN: usize = "addx ".len();
            let count: isize = line[PREFIX_LEN..].parse().unwrap();
            Cmd::Addx(count)
        }
        _ => panic!("unknown command"),
    }
}

fn apply_cmd(state: &mut State, cmd: Cmd) {
    let (latency, dx) = match cmd {
        Cmd::Nop => (1, 0),
        Cmd::Addx(v) => (2, v),
    };

    for _ in 0..latency {
        let x_pos = state.cycle % COLS;
        if x_pos >= state.x - 1 && x_pos <= state.x + 1 {
            state.buffer[state.cycle] = b'#';
        } else {
            state.buffer[state.cycle] = b'.';
        }
        state.cycle += 1;
    }

    state.x = (state.x as isize + dx) as usize;
}

fn main() {
    let mut state = State::new();

    include_str!("input.txt")
        .lines()
        .for_each(|line| apply_cmd(&mut state, parse_cmd(line)));

    for i in 0..ROWS {
        let row = String::from_utf8_lossy(&state.buffer[i * COLS..(i + 1) * COLS]);
        println!("{}", row);
    }
}
