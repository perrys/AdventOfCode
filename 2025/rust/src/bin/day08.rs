fn main() {
    // let argv = std::env::args().collect::<Vec<_>>();
    // if argv.len() != 2 {
    //     panic!("USAGE: {} <input.dat>", argv[0]);
    // }
    //let contents = std::fs::read_to_string(&argv[1]).expect("invalid filename");
    let contents = std::fs::read_to_string("/tmp/foo.dat").expect("invalid filename");
    println!("part1: {}", part1(&contents));
}

type Point = (i32, i32, i32);

#[derive(PartialEq, PartialOrd, Debug)]
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

fn part1(contents: &str) -> usize {
    let points = parse(contents);
    let n = points.len();
    let mut distances = Vec::with_capacity(n / 2 * (n + 1));
    for i in 0..points.len() {
        for j in i..points.len() {
            distances.push(PointPair::new(&points[i], &points[j]));
        }
    }
    distances.sort_by(|lhs, rhs| {
        lhs.distance
            .partial_cmp(&rhs.distance)
            .expect("unexpected NaN!")
    });
    distances.reverse();
    println!("{distances:?}");

    let mut groups = Vec::<Vec<_>>::new();
    for pair in distances {
        let mut found = false;
        for i in 0..groups.len() {
            if groups[i].iter().any(|other_pair: &(Point, Point)| {
                other_pair.0 == pair.points.0
                    || other_pair.1 == pair.points.1
                    || other_pair.0 == pair.points.1
                    || other_pair.1 == pair.points.0
            }) {
                groups[i].push(pair.points);
                found = true;
                break;
            }
        }
        if !found {
            groups.push(vec![pair.points]);
        }
    }
    println!("{groups:?}");
    let mut sizes: Vec<usize> = groups.into_iter().map(|group| group.len()).collect();
    println!("{sizes:?}");
    sizes.sort();
    sizes
        .into_iter()
        .rev()
        .take(3)
        .fold(1, |lhs, rhs| lhs * rhs)
}
