use std::env;

use crate::Operation::*;

const HEIGHT: isize = 60;

enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(u8),
}

fn parse(input: &str) -> Vec<Operation> {
    let mut steps = Vec::<Operation>::new();

    for byte in input.bytes() {
        let step = match byte {
            b'0' => Home,
            b'1'..=b'9' => {
                let distance = (byte - 0x30) as isize;
                Forward(distance * (HEIGHT / 10))
            }
            b'a' | b'b' | b'c' => TurnLeft,
            b'd' | b'e' | b'f' => TurnRight,
            _ => Noop(byte),
        };

        steps.push(step);
    }

    steps
}

fn convert() {}

fn generate_svg() {}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = args.get(1).unwrap();
    let default = format!("{}.svg", input);
    let output = args.get(2).unwrap_or(&default);

    let operations = parse(input);
    let path_data = convert(&operations);
    let file_content = generate_svg(&path_data);
    svg::save(output, &file_content).unwrap();
}
