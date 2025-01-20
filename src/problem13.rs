use std::fs;

fn main() {
    let filepath = "./inputs/input13.txt";

    let contents = fs::read_to_string(filepath).expect("there should of been a file.");

    let machines = parse(&contents);

    let mut tokens = 0;
    for machine in machines.into_iter() {
        let c1 = machine[4] + 10000000000000.0;
        let c2 = machine[5] + 10000000000000.0;

        let a1 = machine[0];
        let a2 = machine[1];

        let b1 = machine[2];
        let b2 = machine[3];

        let a = (b1 * -c2 - b2 * -c1) / (b2 * a1 - b1 * a2);
        let b = (-c1 * a2 - -c2 * a1) / (b2 * a1 - b1 * a2);

        if (c1 as usize == a as usize * a1 as usize + b as usize * b1 as usize)
            && (c2 as usize == a as usize * a2 as usize + b as usize * b2 as usize)
        {
            tokens += a as usize * 3 + b as usize * 1;
        }
    }
    println!("{tokens}");
}

fn parse(contents: &str) -> Vec<Vec<f64>> {
    let mut machines: Vec<Vec<f64>> = vec![];
    let mut machine: Vec<f64> = vec![];
    for line in contents.lines() {
        if line.starts_with("Button A:") {
            let idx = line.find("X+").expect("Missing pattern.");
            let comma = line.find(",").expect("Missing comma.");

            let a = line[idx + "X+".len()..comma]
                .parse::<f64>()
                .expect("number");

            let idx = line.find("Y+").expect("Missing pattern.");
            let last = line.len();
            let b = line[idx + "Y+".len()..last].parse::<f64>().expect("number");

            machine.push(a);
            machine.push(b)
        } else if line.starts_with("Button B:") {
            let idx = line.find("X+").expect("Missing pattern.");
            let comma = line.find(",").expect("Missing comma.");

            let a = line[idx + "X+".len()..comma]
                .parse::<f64>()
                .expect("number");

            let idx = line.find("Y+").expect("Missing pattern.");
            let last = line.len();
            let b = line[idx + "Y+".len()..last].parse::<f64>().expect("number");

            machine.push(a);
            machine.push(b);
        } else if line.starts_with("Prize") {
            let idx = line.find("X=").expect("Missing pattern.");
            let comma = line.find(",").expect("Missing comma.");

            let a = line[idx + "X=".len()..comma]
                .parse::<f64>()
                .expect("number");

            let idx = line.find("Y=").expect("Missing pattern.");
            let last = line.len();
            let b = line[idx + "Y=".len()..last].parse::<f64>().expect("number");

            machine.push(a);
            machine.push(b);

            machines.push(machine);
            machine = vec![];
        }
    }
    machines
}
