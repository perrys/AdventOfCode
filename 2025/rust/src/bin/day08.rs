use std::collections::HashSet;

fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }
    let contents = std::fs::read_to_string(&argv[1]).expect("invalid filename");
    println!("part1: {}", part1(&contents));
    println!("part2: {}", part2(&contents));
}

type Point = (i64, i64, i64);

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
                .parse::<i64>()
                .expect("non-integer");
            let p2 = iter
                .next()
                .expect("2nd point")
                .parse::<i64>()
                .expect("non-integer");
            let p3 = iter
                .next()
                .expect("3rd point")
                .parse::<i64>()
                .expect("non-integer");
            (p1, p2, p3)
        })
        .collect::<Vec<_>>()
}

fn get_distances(contents: &str) -> Vec<PointPair> {
    let points = parse(contents);
    let n = points.len();
    let mut distances = Vec::with_capacity(n / 2 * (n + 1));
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            distances.push(PointPair::new(&points[i], &points[j]));
        }
    }
    distances.sort_by(|lhs, rhs| {
        lhs.distance
            .partial_cmp(&rhs.distance)
            .expect("unexpected NaN!")
    });
    // for d in distances.iter() {
    //     println!("{d:?}");
    // }
    distances
}

fn part2(contents: &str) -> usize {
    let distances = get_distances(contents);
    let _groups = get_groups(distances, None);
    0
}
fn part1(contents: &str) -> usize {
    let distances = get_distances(contents);
    let groups = get_groups(distances, Some(1000));
    let mut sizes: Vec<usize> = groups.into_iter().map(|group| group.len()).collect();
    sizes.sort();
    // println!("{sizes:?}");
    sizes
        .into_iter()
        .rev()
        .take(3)
        .fold(1, |lhs, rhs| lhs * rhs)
}

fn get_groups(distances: Vec<PointPair>, sample_size: Option<usize>) -> Vec<HashSet<Point>> {
    let mut groups = Vec::<HashSet<Point>>::new();
    let samples = sample_size.unwrap_or(distances.len());
    for PointPair {
        distance: _,
        points,
    } in distances.iter().take(samples)
    {
        let mut group_idxs = (None, None);
        for i in 0..groups.len() {
            if groups[i].contains(&points.0) {
                group_idxs.0 = Some(i);
            }
            if groups[i].contains(&points.1) {
                group_idxs.1 = Some(i);
            }
        }
        if let Some(i) = group_idxs.0
            && let Some(j) = group_idxs.1
        {
            if i != j {
                let last = i.max(j);
                let first = i.min(j);
                let tmp = groups.remove(last);
                groups[first].extend(tmp);
            }
        } else if let Some(i) = group_idxs.0 {
            groups[i].insert(points.1);
        } else if let Some(i) = group_idxs.1 {
            groups[i].insert(points.0);
        } else {
            groups.push(HashSet::from([points.0, points.1]));
        }
        if groups.len() == 1 && groups[0].len() == 1000 {
            println!("part2: {points:?}");
            println!("part2: {}", points.0.0 * points.1.0);
            break;
        }
    }
    // println!("groups: {groups:?}");
    groups
}
