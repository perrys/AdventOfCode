fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }
    let contents = std::fs::read_to_string(&argv[1]).expect("invalid filename");
    println!("part1: {}", part1(&contents));
    println!("part2: {}", part2(&contents));
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct CoOrd {
    x: usize,
    y: usize,
}

enum Intersection {
    None,
    Edge,
    Middle,
}

trait Intersector {
    fn intersects(&self, val: usize) -> Intersection;
    fn sort_key(&self) -> usize;
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct HorizontalLine {
    y: usize,
    // usable tiles are on the right in direction of travel
    x0: usize,
    x1: usize,
}

impl Intersector for HorizontalLine {
    fn intersects(&self, x: usize) -> Intersection {
        if self.x0.min(self.x1) < x && self.x1.max(self.x0) > x {
            Intersection::Middle
        } else if x == self.x0 || x == self.x1 {
            Intersection::Edge
        } else {
            Intersection::None
        }
    }
    fn sort_key(&self) -> usize {
        self.y
    }
}

#[derive(Copy, Clone, Debug)]
struct VerticalLine {
    x: usize,
    // usable tiles are on the right in direction of travel
    y0: usize,
    y1: usize,
}

impl Intersector for VerticalLine {
    fn intersects(&self, y: usize) -> Intersection {
        if self.y0.min(self.y1) < y && self.y1.max(self.y0) > y {
            Intersection::Middle
        } else if y == self.y0 || y == self.y1 {
            Intersection::Edge
        } else {
            Intersection::None
        }
    }
    fn sort_key(&self) -> usize {
        self.x
    }
}

/**
 * Find the _furthest_ intersecting line
 */
fn ray_cast<T>(lines: &[T], start_key: usize, target: usize, forwards: bool) -> Option<T>
where
    T: Intersector + Clone,
{
    let start = lines
        .binary_search_by_key(&start_key, T::sort_key)
        .expect("start line not in lines");
    let mut direct: Option<T> = None;
    if forwards {
        for line in lines[start + 1..].iter() {
            match line.intersects(target) {
                Intersection::Edge => direct = Some(line.clone()),
                Intersection::Middle => return Some(line.clone()),
                Intersection::None => continue,
            }
        }
    } else {
        for line in lines[0..start].iter().rev() {
            match line.intersects(target) {
                Intersection::Edge => direct = Some(line.clone()),
                Intersection::Middle => return Some(line.clone()),
                Intersection::None => continue,
            }
        }
    }
    direct
}

fn connected(h_lines: &[HorizontalLine], v_lines: &[VerticalLine], p1: &CoOrd, p2: &CoOrd) -> bool {
    if p1.y == p2.y || p1.x == p2.x {
        return false; // just ignore these, unlikely to be the max area
    }
    let (lhs, rhs) = if p2.x > p1.x { (p1, p2) } else { (p2, p1) };
    let mut forwards = rhs.y > lhs.y;
    // let target1 = CoOrd { x: lhs.x, y: rhs.y };
    // let target2 = CoOrd { x: rhs.x, y: lhs.y };

    // first point vertical ray cast:
    match ray_cast(h_lines, lhs.y, lhs.x, forwards) {
        Some(h_line) if (forwards && h_line.y < rhs.y) || (!forwards && h_line.y > rhs.y) => {
            return false;
        }
        None => return false,
        _ => (),
    }

    // first point horizontal ray cast:
    match ray_cast(v_lines, lhs.x, lhs.y, true) {
        Some(v_line) if v_line.x < rhs.x => return false,
        None => return false,
        _ => (),
    }

    forwards = !forwards;
    // second point vertical:
    match ray_cast(h_lines, rhs.y, rhs.x, forwards) {
        Some(h_line) if (forwards && h_line.y < rhs.y) || (!forwards && h_line.y > rhs.y) => {
            return false;
        }
        None => return false,
        _ => (),
    }

    // second point horizontal:
    match ray_cast(v_lines, rhs.x, rhs.y, false) {
        Some(v_line) if v_line.x > rhs.x => return false,
        None => return false,
        _ => (),
    }

    true
}

fn parse_input_line(line: &str) -> CoOrd {
    let mut iter = line.split(',');
    let x = iter
        .next()
        .expect("val 1")
        .parse::<usize>()
        .expect("non-int 1");
    let y = iter
        .next()
        .expect("val 2")
        .parse::<usize>()
        .expect("non-int 2");
    assert!(iter.next().is_none());
    CoOrd { x, y }
}

fn area(p1: &CoOrd, p2: &CoOrd) -> usize {
    (1 + p1.x.abs_diff(p2.x)) * (1 + p1.y.abs_diff(p2.y))
}

fn part1(contents: &str) -> usize {
    let points = contents.lines().map(parse_input_line).collect::<Vec<_>>();
    let mut max = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            max = max.max(area(&points[i], &points[j]));
        }
    }
    max
}

fn part2(contents: &str) -> usize {
    let points = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_input_line)
        .collect::<Vec<_>>();
    let npoints = points.len();

    let mut h_lines = Vec::new();
    let mut v_lines = Vec::new();
    for i in 0..npoints {
        let p0 = if i == 0 {
            points[npoints - 1]
        } else {
            points[i - 1]
        };
        let p1 = points[i];
        if p0.x == p1.x {
            v_lines.push(VerticalLine {
                x: p0.x,
                y0: p0.y,
                y1: p1.y,
            });
        } else {
            h_lines.push(HorizontalLine {
                y: p0.y,
                x0: p0.x,
                x1: p1.x,
            });
        }
    }

    v_lines.sort_by_key(VerticalLine::sort_key);
    h_lines.sort_by_key(HorizontalLine::sort_key);

    let mut max = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = &points[i];
            let p2 = &points[j];
            if *p1 == (CoOrd { x: 9, y: 5 }) && *p2 == (CoOrd { x: 2, y: 3 }) {
                println!("should be max");
            }
            if connected(&h_lines, &v_lines, p1, p2) {
                let area = area(p1, p2);
                // println!("[{p1:?} {p2:?}] connected area = {area}");
                max = max.max(area);
            // } else {
            //     println!("[{p1:?} {p2:?}] not connected");
            }
        }
    }
    max
}

#[cfg(test)]
mod tester {
    use super::*;

    #[test]
    fn test_ray_cast() {
        let hlines = vec![
            HorizontalLine { y: 1, x0: 2, x1: 6 },
            HorizontalLine { y: 2, x0: 7, x1: 9 },
            HorizontalLine { y: 3, x0: 3, x1: 5 },
            HorizontalLine { y: 4, x0: 6, x1: 7 },
        ];
        assert_eq!(ray_cast(&hlines, 1, 8, true), Some(hlines[1].clone()));
        assert_eq!(ray_cast(&hlines, 1, 3, true), Some(hlines[2].clone()));
        assert!(ray_cast(&hlines, 1, 1, true).is_none());

        assert_eq!(ray_cast(&hlines, 4, 7, false), Some(hlines[1].clone()));
        assert_eq!(ray_cast(&hlines, 3, 8, false), Some(hlines[1].clone()));
        assert_eq!(ray_cast(&hlines, 2, 5, false), Some(hlines[0].clone()));
        assert!(ray_cast(&hlines, 1, 5, false).is_none());
    }

    const TEST_DATA: &str = r#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
    "#;
    #[test]
    fn test_part2() {
        let ans = part2(TEST_DATA);
        assert_eq!(ans, 24);
    }
}
