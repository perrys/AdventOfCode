fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }
    let contents = std::fs::read_to_string(&argv[1]).expect("invalid filename");
    println!("part1: {}", part1(&contents));
    println!("part2: {}", part2(&contents));
}

#[derive(Copy, Clone, Debug)]
struct CoOrd {
    x: usize,
    y: usize,
}

trait Intersector {
    fn intersects(&self, val: usize) -> bool;
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
    fn intersects(&self, x: usize) -> bool {
        self.x0.min(self.x1) <= x && self.x1.max(self.x0) >= x
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
    fn intersects(&self, y: usize) -> bool {
        self.y0.min(self.y1) <= y && self.y1.max(self.y0) >= y
    }
    fn sort_key(&self) -> usize {
        self.x
    }
}

fn ray_cast<T>(
    lines: &[T],
    start_key: usize,
    target: usize,
    forwards: bool,
) -> Result<T, &'static str>
where
    T: Intersector + Clone,
{
    let start = lines
        .binary_search_by_key(&start_key, T::sort_key)
        .expect("start line not in lines");
    if forwards {
        for line in lines[start + 1..].iter() {
            if line.intersects(target) {
                return Ok(line.clone());
            }
        }
    } else {
        for line in lines[0..start].iter().rev() {
            if line.intersects(target) {
                return Ok(line.clone());
            }
        }
    }
    Err("ray cast failed to hit anything")
}

fn connected(h_lines: &[HorizontalLine], v_lines: &[VerticalLine], p1: &CoOrd, p2: &CoOrd) -> bool {
    if p1.y == p2.y || p1.x == p2.x {
        return false; // just ignore these, unlikely to be the max area
    }
    let (lhs, rhs) = if p2.x > p1.x { (p1, p2) } else { (p2, p1) };
    let northward = rhs.y > lhs.y;
    let target1 = CoOrd { x: lhs.x, y: rhs.y };
    let target2 = CoOrd { x: rhs.x, y: lhs.y };

    // first point vertical ray cast:
    let h1 = ray_cast(h_lines, lhs.y, lhs.x, northward).expect("ray cast within area");
    if (northward && h1.y < target2.y) || (!northward && h1.y > target1.y) {
        return false;
    }

    // first point horizontal ray cast:
    let v1 = ray_cast(v_lines, lhs.x, lhs.y, true).expect("ray cast within area");
    if v1.x < target2.x {
        return false;
    };

    // second point vertical:
    let h2 = ray_cast(h_lines, rhs.y, rhs.x, !northward).expect("ray cast within area");
    if (!northward && h2.y > target1.y) || (northward && h2.y < target1.y) {
        return false;
    }

    // second point horizontal:
    let v2 = ray_cast(v_lines, rhs.x, rhs.y, false).expect("ray cast within area");
    if v2.x > target1.x {
        return false;
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
    for i in 1..npoints {
        let p0 = points[i - 1];
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
            if connected(&h_lines, &v_lines, &points[i], &points[j]) {
                max = max.max(area(&points[i], &points[j]));
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
        assert_eq!(ray_cast(&hlines, 1, 8, true), Ok(hlines[1].clone()));
        assert_eq!(ray_cast(&hlines, 1, 3, true), Ok(hlines[2].clone()));
        assert!(ray_cast(&hlines, 1, 1, true).is_err());

        assert_eq!(ray_cast(&hlines, 4, 7, false), Ok(hlines[1].clone()));
        assert_eq!(ray_cast(&hlines, 3, 8, false), Ok(hlines[1].clone()));
        assert_eq!(ray_cast(&hlines, 2, 5, false), Ok(hlines[0].clone()));
        assert!(ray_cast(&hlines, 1, 5, false).is_err());
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
