fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }
    let contents = std::fs::read_to_string(&argv[1]).expect("invalid filename");
    let points = parse(&contents);
    println!("part1: ", part1(&points));
}

type Point = (i32, i32, i32);

#[derive(PartialEq, PartialOrd)]
struct PointPair {
    distance: f64,
    points: (Point, Point),
}

impl PointPair {
    fn new(p1: &Point, p2: &Point) -> Self {
        let dx = p1.0 - p2.0;
        let dy = p1.1 - p2.1;
        let dz = p1.2 - p2.2;
        let distance = ((dx * dx + dy * dy + dz * dz) as f64).sqrt();
        Self {
            points: (*p1, *p2),
            distance,
        }
    }
}
impl Eq for PointPair {}

impl Ord for PointPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.total_cmp(&other.distance)
    }
}

fn parse(contents: &str) -> Vec<Point> {
    contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut iter = line.split(',');
            let p1 = iter
                .next()
                .expect("1st point")
                .parse::<i32>()
                .expect("non-integer");
            let p2 = iter
                .next()
                .expect("2nd point")
                .parse::<i32>()
                .expect("non-integer");
            let p3 = iter
                .next()
                .expect("3rd point")
                .parse::<i32>()
                .expect("non-integer");
            (p1, p2, p3)
        })
        .collect::<Vec<_>>()
}

fn part1(contents: &str) -> f64 {
    let points = parse(contents);
    let n = points.len();
    let distances = Vec::with_capacity(n / 2 * (n + 1));
    for i in 0..points.len() {
        for j in i..points.len() {
            distances.push(PointPair::new(&points[i], &points[j]));
        }
    }
    distances.sort_by_key(|s| s.distance);
}
