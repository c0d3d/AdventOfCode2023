use std::fs::read_to_string;
use std::ops::Add;
use std::u64;

type Red = u64;
type Green = u64;
type Blue = u64;



struct Run(Red, Green, Blue);
struct RunMin(Red, Green, Blue);

fn parse_run_by<F>(r_str: &str, outcome: &mut (Red, Green, Blue), f_cmp: F) -> Option<()>
    where F: Fn(u64, u64) -> u64
{
    for piece in r_str.split(",") {
        let components = piece.trim().split(" ").collect::<Vec<_>>();
        if components.len() != 2 {
            return None;
        }
        let num = components[0].parse().ok()?;
        match components[1] {
            "red" => { outcome.0 = f_cmp(outcome.0, num); },
            "green" => { outcome.1 = f_cmp(outcome.1, num); },
            "blue" => { outcome.2 = f_cmp(outcome.2, num); },
            _ => { return None; }
        }
    }
    return Some(());
}

impl Run {
    fn is_possible(&self) -> bool {
        return self.0 <= 12 && self.1 <= 13 && self.2 <= 14;
    }

    fn parse(run_str: &str) -> Option<Run> {
        let mut t = (0,0,0);
        parse_run_by(run_str, &mut t, std::cmp::max)?;
        return Some(Run(t.0, t.1, t.2));
    }
}

impl RunMin {
    fn parse(run_str: &str) -> Option<RunMin> {
        let mut t = (u64::MAX, u64::MAX, u64::MAX);
        parse_run_by(run_str, &mut t, std::cmp::min)?;
        if t.0 == u64::MAX { t.0 = 0 }
        if t.1 == u64::MAX { t.1 = 0 }
        if t.2 == u64::MAX { t.2 = 0 }
        return Some(RunMin(t.0, t.1, t.2));
    }
}


struct Game(u64, Vec<Run>);
struct GameMin(u64, Vec<RunMin>);


fn parse_with<T, F>(l: &str, f: F) -> Option<(u64, Vec<T>)>
where F: Fn(&str) -> Option<T>
{
    let s: Vec<&str> = l.split(":").collect();
    if s.len() != 2 {
        return None;
    }

    let id = s[0][5..].parse().ok()?;
    return s[1].split(";").map(f).collect::<Option<Vec<_>>>().map(|o| (id, o));

}

impl Game {

    fn parse(l: &str) -> Option<Game> {
        let (id, runs) = parse_with(l, Run::parse)?;
        return Some(Game(id, runs));
    }


    fn is_possible(&self) -> bool {
        self.1.iter().all(Run::is_possible)
    }
}

impl GameMin {
    fn parse(l: &str) -> Option<GameMin> {
        let (id, runs) = parse_with(l, RunMin::parse)?;
        return Some(GameMin(id, runs));
    }

    fn power(self) -> u64 {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for run in self.1 {
            max_red = std::cmp::max(run.0, max_red);
            max_green = std::cmp::max(run.1, max_green);
            max_blue = std::cmp::max(run.2, max_blue);
        }

        return max_red * max_green * max_blue;
    }
}

fn main() {

    let p1_id_total: u64 = read_to_string("input.txt")
        .unwrap()
        .lines()
        .filter_map(|line| {
            let g = Game::parse(line).expect("Valid Game");
            if g.is_possible() { Some(g.0) } else { None }
        })
        .fold(0, Add::add);

    let p2_total = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(GameMin::parse)
        .map(Option::unwrap) // YOLO
        .map(GameMin::power)
        .fold(0, Add::add);

    println!("{p1_id_total}");
    println!("{p2_total}");
}
