type History = Vec<i64>;

fn parse_line(s: &str) -> History {
    let mut n = vec![];
    for num in s.split_ascii_whitespace() {
        n.push(num.parse().unwrap());
    }
    return n;
}

fn predict(h: &History) -> (i64, i64) {
    let mut prediction = *h.last().unwrap();
    let mut scratch1 = h.clone();
    let mut scratch2 = vec![];
    let mut firsts = vec![];

    let mut all_zero = false;
    while !all_zero {
        all_zero = true;
        for i in 0..scratch1.len() - 1 {
            if i == 0 {
                firsts.push(scratch1[i]);
            }
            // Drop 1 => i+1 is always safe.
            let nxt = scratch1[i + 1] - scratch1[i];
            scratch2.push(nxt);
            if nxt != 0 {
                all_zero = false;
            }
        }
        prediction += *scratch2.last().unwrap();
        scratch1 = scratch2;
        scratch2 = Vec::new();
    }

    firsts.push(0);

    let left_most = project_first(firsts[0], 0, &firsts);

    return (prediction, left_most);
}


fn project_first(right: i64, idx: usize, firsts: &Vec<i64>) -> i64 {
    if idx == firsts.len() - 1 {
        return 0;
    } else {
        return right - project_first(firsts[idx+1], idx+1, firsts);
    }
}

fn main() {
    let sensor_histories: Vec<History> =
        include_str!("input.txt").lines().map(parse_line).collect();

    let (p1, p2): (i64, i64) = sensor_histories
        .iter()
        .map(predict)
        .reduce(|(x, y), (x2, y2)| (x + x2, y + y2))
        .unwrap();
    println!("P1: {p1}");
    println!("P2: {p2}");
}
