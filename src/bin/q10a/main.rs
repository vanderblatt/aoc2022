enum Cmd {
    Nop,
    Addx(isize),
}

struct State<'a> {
    x: isize,
    cycle: usize,
    breakpoints: &'a [usize],
    intensity: isize,
}

impl<'a> State<'a> {
    fn new(breakpoints: &'a [usize]) -> Self {
        State {
            x: 1,
            cycle: 0,
            intensity: 0,
            breakpoints,
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
    let next_cycle = state.cycle + latency;

    while let Some(&breakpoint) = state.breakpoints.first() {
        if breakpoint <= next_cycle {
            state.intensity += breakpoint as isize * state.x;
            state.breakpoints = &state.breakpoints[1..];
        } else {
            break;
        }
    }

    state.cycle = next_cycle;
    state.x += dx;
}

fn main() {
    let breakpoints = [20, 60, 100, 140, 180, 220];
    let mut state = State::new(&breakpoints);

    include_str!("input.txt")
        .lines()
        .for_each(|line| apply_cmd(&mut state, parse_cmd(line)));

    println!("{}", state.intensity);
}
