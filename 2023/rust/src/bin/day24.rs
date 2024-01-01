//!
//! Advent of code challenge 2023 day 24.
//!
//! See <https://adventofcode.com/2023/day/24>
//!
extern crate nalgebra as na;
use na::{SMatrix, Vector6};
use std::{fs, ops};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Couldn't read file {filename}");

    println!("part1 total is {}", part1(contents.as_str()));
    println!("part2 total is {}", part2(contents.as_str()));
}

fn part1(contents: &str) -> usize {
    let hailstones = parse_file(contents);
    let range = 200000000000000f64..=400000000000000f64;
    get_valid_intercepts(hailstones, range)
}

fn part2(contents: &str) -> usize {
    let hailstones = parse_file(contents);
    let (position, _velocity) = calculate_intersection_trajectory(hailstones);
    position.to_array().into_iter().sum::<f64>().round() as usize
}

fn get_valid_intercepts(hailstones: Vec<HailStone>, range: std::ops::RangeInclusive<f64>) -> usize {
    let test = |hs1: &HailStone, hs2| {
        let point = hs1.intercept_point(hs2);
        match point {
            None => false,
            Some((x, y)) => {
                range.contains(&x)
                    && range.contains(&y)
                    // the "forward in time" clause:
                    && if hs1.velocity.y == 0.0 {
                        y == hs1.position.y
                    } else if hs1.velocity.y > 0.0 {
                        y > hs1.position.y
                    } else {
                        y < hs1.position.y
                    }
                    && if hs2.velocity.y == 0.0 {
                        y == hs2.position.y
                    } else if hs2.velocity.y > 0.0 {
                        y > hs2.position.y
                    } else {
                        y < hs2.position.y
                    }
            }
        }
    };
    let mut count = 0usize;
    for (idx, hs1) in hailstones.iter().enumerate() {
        for hs2 in hailstones[idx + 1..].iter() {
            if test(hs1, hs2) {
                count += 1
            }
        }
    }
    count
}

/**
 * Return the position and velocity vectors for an intersecting trajectory over
 * the given hailstones.
*/
fn calculate_intersection_trajectory(hailstones: Vec<HailStone>) -> (Vec3D, Vec3D) {
    // The solution to this problem is where all hailstones intercept the rock:
    //
    // p_h + v_h t_h = p_r + v_r t_h where p_h, v_h and t_h are the position,
    // velocity, and time of incerception of the hailstone, for all hailstones,
    // and p_r, v_r are the position and velocity of the rock.
    //
    // We can eliminate the time component of this problem by considering the
    // frame of reference of the rock. In that frame, the rock is stationary (at
    // the origin), and everything else moves relative to it with velocity (v_h
    // - v_r) and starting position (p_h - p_r). The hailstones will then hit
    // the rock when their velocity vector points in the opposite direction to
    // their position vector (both in the rock's frame of reference). Time is
    // eliminated because this guarantees that the interception will occur at
    // some point in the future, and we don't care when exactly.
    //
    // The velocity and position vectors point in opposite directions when their
    // cross product is zero (technically this also occurs when they point in
    // the same direction, but because we know that a solution exists for all
    // hailstones we don't have to worry about that eventuality). This means
    // that for all hailstones:
    //
    // p x v = 0  where p (position) = (p_h - p_r) and v (velocity) = (v_h - v_r)
    //
    // We are given all of the p_h and v_h values, and we need to solve for p_r and v_r.
    // Multiplying out the above equation:
    //
    // p_h x v_h - p_r x v_h - p_h x v_r + p_r x v_r = 0
    //
    // The last term contains two unknowns, so we need to eliminate it. We do
    // this by choosing two hailstones and subtracting their equations:
    //
    // p_1 x v_1 - p_r x v_1 - p_1 x v_r + p_r x v_r = 0  }
    // p_2 x v_2 - p_r x v_2 - p_2 x v_r + p_r x v_r = 0  } subtracting these gives:
    //
    // p_1 x v_1 - p_2 x v_2 - p_r x v_1 + p_r x v_2 - p_1 x v_r + p_2 x v_r = 0
    //
    // Similarly, for a third hailstone:
    //
    // p_1 x v_1 - p_3 x v_3 - p_r x v_1 + p_r x v_3 - p_1 x v_r + p_3 x v_r = 0
    //
    // The last two vector equations above generate 6 equations with 6 unknows
    // when written with separate x, y and z coordinates. We will use the
    // nalgebra linear algebra library to solve these equations for p_r and v_r.

    let mut matrix_a = Vec::new();
    let mut vector_b = Vec::new();
    let mut add_point = |idx_1, idx_2| {
        let hs1: &HailStone = &hailstones[idx_1];
        let hs2: &HailStone = &hailstones[idx_2];
        // re-write above so the knowns are on the lhs of the cross product:
        // https://en.wikipedia.org/wiki/Cross_product#Algebraic_properties
        // v_1 x p_r - v_2 x p_r - p_1 x v_r + p_2 x v_r =  p_2 x v_2 - p_1 x v_1
        let mut rhs = hs2.position.cross_product(&hs2.velocity);
        rhs = rhs - hs1.position.cross_product(&hs1.velocity);
        let coeff_p_x = hs1.velocity.cross_coefficients()[0] - hs2.velocity.cross_coefficients()[0];
        let coeff_p_y = hs1.velocity.cross_coefficients()[1] - hs2.velocity.cross_coefficients()[1];
        let coeff_p_z = hs1.velocity.cross_coefficients()[2] - hs2.velocity.cross_coefficients()[2];
        let coeff_v_x = hs2.position.cross_coefficients()[0] - hs1.position.cross_coefficients()[0];
        let coeff_v_y = hs2.position.cross_coefficients()[1] - hs1.position.cross_coefficients()[1];
        let coeff_v_z = hs2.position.cross_coefficients()[2] - hs1.position.cross_coefficients()[2];

        // x terms:
        vector_b.push(rhs.x);
        matrix_a.extend_from_slice(&coeff_p_x.to_array());
        matrix_a.extend_from_slice(&coeff_v_x.to_array());

        // y terms:
        vector_b.push(rhs.y);
        matrix_a.extend_from_slice(&coeff_p_y.to_array());
        matrix_a.extend_from_slice(&coeff_v_y.to_array());

        // z terms:
        vector_b.push(rhs.z);
        matrix_a.extend_from_slice(&coeff_p_z.to_array());
        matrix_a.extend_from_slice(&coeff_v_z.to_array());
    };

    // just use the first 3 points:
    add_point(0, 1);
    add_point(0, 2);

    // solve AX = B for X, where X are the unknown position and velocity of the rock
    let matrix_a = SMatrix::<f64, 6, 6>::from_vec(matrix_a);
    let matrix_a = matrix_a.transpose();
    let vector_b = Vector6::<f64>::from_vec(vector_b);
    println!("matrix_a: {matrix_a}");
    println!("vector_b: {vector_b}");
    let decomp = matrix_a.lu();
    let vector_x = decomp.solve(&vector_b).expect("no solution found");
    println!("result: {vector_x}");
    let position = Vec3D {
        x: vector_x[0],
        y: vector_x[1],
        z: vector_x[2],
    };
    let velocity = Vec3D {
        x: vector_x[3],
        y: vector_x[4],
        z: vector_x[5],
    };
    (position, velocity)
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec3D {
    x: f64,
    y: f64,
    z: f64,
}

impl ops::Add for Vec3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec3D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Vec3D {
    fn to_array(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
    fn cross_coefficients(&self) -> [Vec3D; 3] {
        [
            Self {
                x: 0.0,
                y: -self.z,
                z: self.y,
            },
            Self {
                x: self.z,
                y: 0.0,
                z: -self.x,
            },
            Self {
                x: -self.y,
                y: self.x,
                z: 0.0,
            },
        ]
    }
    fn cross_product(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

#[derive(Debug, PartialEq)]
struct HailStone {
    position: Vec3D,
    velocity: Vec3D,
}

impl HailStone {
    fn new(line: &str) -> Self {
        let values: [f64; 6] = line
            .split(&[' ', ',', '@'])
            .filter(|t| !t.is_empty())
            .map(|s| s.parse::<f64>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_else(|_| panic!("uanble to parse line \"{line}\""))
            .try_into()
            .unwrap_or_else(|_| panic!("incorrect number of tokens in line \"{line}\""));
        Self {
            position: Vec3D {
                x: values[0],
                y: values[1],
                z: values[2],
            },
            velocity: Vec3D {
                x: values[3],
                y: values[4],
                z: values[5],
            },
        }
    }

    fn slope_and_y_intercept(&self) -> (f64, f64) {
        let m = self.velocity.y / self.velocity.x;
        let c = self.position.y - self.position.x * m;
        (m, c)
    }

    fn intercept_point(&self, other: &HailStone) -> Option<(f64, f64)> {
        let (m1, c1) = self.slope_and_y_intercept();
        let (m2, c2) = other.slope_and_y_intercept();
        if m1 == m2 {
            None
        } else {
            let x0 = (c2 - c1) / (m1 - m2);
            let y0 = (c1 * m2 - c2 * m1) / (m2 - m1);
            Some((x0, y0))
        }
    }
}

fn parse_file(contents: &str) -> Vec<HailStone> {
    let hailstones = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(HailStone::new)
        .collect::<Vec<_>>();
    hailstones
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test24 {
    use super::*;

    #[test]
    fn GIVEN_simple_coords_WHEN_calcing_slope_and_y_intercept_THEN_correct() {
        let dotest = |hs, expected| {
            let hs = HailStone::new(hs);
            assert_eq!(expected, hs.slope_and_y_intercept());
        };
        dotest("1, 1, 0 @ 1, 2, 0", (2.0, -1.0));
        dotest("10, -15, 0 @ 1, -3, 0", (-3.0, 15.0));
        dotest("10, -45, 0 @ 2, -6, 0", (-3.0, -15.0));
    }

    #[test]
    fn GIVEN_two_lines_WHEN_calcing_intercept_point_THEN_correct() {
        let dotest = |hs1, hs2, expected| {
            let hs1 = HailStone::new(hs1);
            let hs2 = HailStone::new(hs2);
            assert_eq!(expected, hs1.intercept_point(&hs2));
        };
        dotest("0, 0, 0 @ 2, 2, 0", "0, 0, 0 @ 1, 2, 0", Some((0.0, 0.0)));
        dotest("1, 1, 0 @ 2, 2, 0", "2, 3, 0 @ 1, 2, 0", Some((1.0, 1.0)));
        dotest("1, 1, 0 @ 2, 2, 0", "2, 3, 0 @ 2, 2, 0", None);
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part1_run_THEN_matches_expected() {
        let hailstones = parse_file(EXAMPLE);
        assert_eq!(2, get_valid_intercepts(hailstones, 7.0..=27.0));
    }

    fn approx_equal(a: f64, b: f64, dp: u8) -> bool {
        let p = 10f64.powi(-(dp as i32));
        (a - b).abs() < p
    }

    #[test]
    fn GIVEN_simple_2d_equation_WHEN_solving_with_nalgebra_THEN_correct_solution_returned() {
        type Matrix22 = SMatrix<f64, 2, 2>;
        // 3x+2y=3  }
        // 5x+4y=8  } => x=-2, y=4.5
        let m = Matrix22::new(3.0, 2.0, 5.0, 4.0);
        let b = na::Vector2::new(3.0, 8.0);
        let decomp = m.lu();
        let x = decomp.solve(&b).expect("Linear resolution failed.");
        let x: Vec<_> = Vec::from_iter(x.column(0).iter().copied());
        let expected = [-2.0, 4.5];
        x.iter()
            .zip(expected.iter())
            .for_each(|(&a, &b)| assert!(approx_equal(a, b, 11)));
    }

    #[test]
    fn GIVEN_simple_vectors_WHEN_calculating_cross_product_THEN_known_answer_returned() {
        let v1 = Vec3D {
            x: 1.0,
            y: 2.0,
            z: -1.0,
        };
        let v2 = Vec3D {
            x: 3.0,
            y: 4.0,
            z: 4.0,
        };
        assert_eq!(
            // https://www.analyzemath.com/stepbystep_mathworksheets/vectors/cross_product.html
            Vec3D {
                x: 12.0,
                y: -7.0,
                z: -2.0
            },
            v1.cross_product(&v2)
        );
    }

    #[test]
    fn GIVEN_aoc_example_WHEN_part2_run_THEN_matches_expected() {
        let hailstones = parse_file(EXAMPLE);
        let position = Vec3D {
            x: 24.0,
            y: 13.0,
            z: 10.0,
        };
        let velocity = Vec3D {
            x: -3.0,
            y: 1.0,
            z: 2.0,
        };
        let answer = calculate_intersection_trajectory(hailstones);
        position
            .to_array()
            .iter()
            .zip(answer.0.to_array().iter())
            .for_each(|(&lhs, &rhs)| assert!(approx_equal(lhs, rhs, 11)));
        velocity
            .to_array()
            .iter()
            .zip(answer.1.to_array().iter())
            .for_each(|(&lhs, &rhs)| assert!(approx_equal(lhs, rhs, 11)));
    }

    static EXAMPLE: &str = r#"
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;
}
