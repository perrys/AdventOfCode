fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }
    let contents = std::fs::read_to_string(&argv[1]).expect("invalid filename");
    let points = parse(contents);
    println!("part1: " part1(points));
}

type Point = (i32, i32, i32);

fn parse(contents: &str) -> Vec<Point> {
    contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let iter = line.split(',');
            let p1 = iter.next().expect("1st point");
            let p2 = iter.next().expect("2nd point");
            let p3 = iter.next().expect("3rd point");
            (p1, p2, p3)
        })
        .collect::<Vec<_>>()
}

fn part1(contents: &str) -> f64 {
    let points = parse(contents);
    for i in 0..points.len() {
        for j in i..points.len() {
            distances.push(distance(points[i], points[j]));
        }
    }
}

fn distance(p1: &Point, p2: &Points) -> f64 {
    std::ops:: 
    let dx = p1.0 - p2.0 ;
    let dy = p1.1 - p2.1 ;
    let dz = p1.2 - p2.2 ;
    sqrt(dx * dx + dy * dy + dz * dz)
}
