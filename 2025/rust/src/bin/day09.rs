fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        panic!("USAGE: {} <input.dat>", argv[0]);
    }
    let contents = std::fs::read_to_string(&argv[1]).expect("invalid filename");
    println!("part1: {}", part1(&contents));
}

#[derive(Copy, Clone, Debug)]
struct CoOrd {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug)]
struct HorizontalLine {
    y: usize,
    // usable tiles are on the right in direction of travel
    x0: usize,
    x1: usize,
}

impl HorizontalLine {
    fn intersects(&self, x: usize) -> bool {
        self.x0.min(self.x1) <= x && self.x1.max(self.x0) >= x
    }
}

#[derive(Copy, Clone, Debug)]
struct VerticalLine {
    x: usize,
    // usable tiles are on the right in direction of travel
    y0: usize,
    y1: usize,
}

impl VerticalLine {
    fn intersects(&self, y: usize) -> bool {
        self.y0.min(self.y1) <= y && self.y1.max(self.y0) >= y
    }
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

fn part2(contents: &str) {
    let points = contents.lines().map(parse_input_line).collect::<Vec<_>>();
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
    v_lines.sort_by_key(|line| line.x);
    h_lines.sort_by_key(|line| line.y);

    let mut next_is_vert = points[0].x == points[1].x;
    // ray casting
    for i in 0..npoints {
        let current = points[i];
        let prev = points[(i + 1) % npoints];
        let next = points[(i + npoints - 1) % npoints];
        if next_is_vert {
            let mut first_intersecting: Option<HorizontalLine> = None;
            let start_row = h_lines
                .binary_search_by_key(&current.y, |line| line.y)
                .expect("hline missing");
            let mut i = start_row;
            loop {
                if h_lines[i].intersects(current.x) {
                    first_intersecting = Some(h_lines[i]);
                    break;
                }

                if next.y > current.y {
                    i += 1;
                } else {
                    i -= 1;
                };
                if i == 0 || i + 1 == h_lines.len() {
                    break;
                }
            }
        }
        next_is_vert = !next_is_vert;
    }
}
