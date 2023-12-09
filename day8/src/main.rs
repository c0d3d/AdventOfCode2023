use std::collections::HashMap;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// I looked this shit up.
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn main() {
    let mut net = HashMap::new();
    let mut starts = Vec::new();

    let s = include_str!("input.txt");
    let mut lines = s.lines();
    let dir_line = lines.next().unwrap();
    let _ = lines.next().unwrap();
    for l in lines {
        let label = &l[..3];
        let left = &l[7..10];
        let right = &l[12..15];
        if label.ends_with("A") {
            starts.push(label);
        }
        //println!("'{label}', ('{left}', '{right}')");
        net.insert(label, (left, right));
    }

    let mut p1_steps = 0;
    let mut completion_times = Vec::new();

    for start in starts.iter() {
        let mut cur = start;
        let mut cur_steps = 0;
        let dirs = dir_line.chars().cycle();

        for d in dirs {
            if cur.ends_with('Z') {
                if *start == "AAA" {
                    p1_steps = cur_steps;
                }
                completion_times.push(cur_steps);
                break;
            } else {
                let (left, right) = net.get(cur).unwrap();
                if d == 'L' {
                    cur = left;
                } else if d == 'R' {
                    cur = right;
                }
            }
            cur_steps += 1;
        }
    }

    let overall_lcm = completion_times.into_iter().reduce(lcm).unwrap();

    println!("P1: {p1_steps}");
    println!("P2: {overall_lcm}");
}
