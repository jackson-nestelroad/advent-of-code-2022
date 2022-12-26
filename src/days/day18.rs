use std::{
    collections::{HashSet, VecDeque},
    ops::{Add, AddAssign, Sub},
    str::FromStr,
};

use crate::common::{AocError, AocResult, IntoAocResult};
use itertools::Itertools;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn surrounding<'a>(&'a self) -> Surrounding<'a> {
        Surrounding::new(self)
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

struct Surrounding<'a> {
    point: &'a Point,
    i: usize,
}

impl<'a> Surrounding<'a> {
    pub fn new(point: &'a Point) -> Self {
        Self { point, i: 0 }
    }

    pub fn transformations() -> &'static [Point] {
        lazy_static! {
            static ref TRANFORMATIONS: [Point; 6] = [
                Point::new(0, 0, -1),
                Point::new(0, 0, 1),
                Point::new(0, -1, 0),
                Point::new(0, 1, 0),
                Point::new(-1, 0, 0),
                Point::new(1, 0, 0),
            ];
        }
        &*TRANFORMATIONS
    }

    fn next_item(&self) -> Option<Point> {
        Some(
            self.point
                .add(Self::transformations().get(self.i).copied()?),
        )
    }

    fn update_state(&mut self) {
        self.i += 1
    }
}

impl<'a> Iterator for Surrounding<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        let output = self.next_item()?;
        self.update_state();
        Some(output)
    }
}

impl FromStr for Point {
    type Err = AocError;
    fn from_str(s: &str) -> AocResult<Self> {
        let (x, y, z) = s
            .split(',')
            .collect_tuple()
            .into_aoc_result_msg("invalid cube format")?;
        Ok(Self::new(
            x.parse().into_aoc_result_msg("invalid x-coordinate")?,
            y.parse().into_aoc_result_msg("invalid y-coordinate")?,
            z.parse().into_aoc_result_msg("invalid z-coordinate")?,
        ))
    }
}

struct Cubes {
    cubes: HashSet<Point>,
}

impl Cubes {
    fn from_points(input: &str) -> AocResult<Self> {
        Ok(Self {
            cubes: input
                .lines()
                .map(|line| Point::from_str(line))
                .collect::<AocResult<_>>()?,
        })
    }

    fn surface_area(&self) -> u64 {
        self.cubes
            .iter()
            .map(|cube| (cube, 6))
            .map(|(cube, sides)| {
                sides
                    - cube
                        .surrounding()
                        .filter(|point| self.cubes.contains(&point))
                        .count() as u64
            })
            .sum()
    }

    fn external_surface_area(&self) -> u64 {
        // Flood fill the 3D area around the lava droplet, extending 1 unit out.
        let min_x = self.cubes.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x - 1;
        let max_x = self.cubes.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x + 1;
        let min_y = self.cubes.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y - 1;
        let max_y = self.cubes.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y + 1;
        let min_z = self.cubes.iter().min_by(|a, b| a.z.cmp(&b.z)).unwrap().z - 1;
        let max_z = self.cubes.iter().max_by(|a, b| a.z.cmp(&b.z)).unwrap().z + 1;
        let start = Point::new(min_x, min_y, min_z);

        let mut filled = HashSet::new();
        let mut to_fill = VecDeque::from([start]);
        while let Some(next) = to_fill.pop_front() {
            if !filled.contains(&next)
                && !self.cubes.contains(&next)
                && next.x >= min_x
                && next.x <= max_x
                && next.y >= min_y
                && next.y <= max_y
                && next.z >= min_z
                && next.z <= max_z
            {
                filled.insert(next);
                to_fill.extend(next.surrounding());
            }
        }
        self.cubes
            .iter()
            .map(|cube| {
                cube.surrounding()
                    .filter(|point| filled.contains(point))
                    .count() as u64
            })
            .filter(|neighboring_external| neighboring_external > &0)
            .sum()
    }
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    let cubes = Cubes::from_points(input)?;
    Ok(cubes.surface_area())
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    let cubes = Cubes::from_points(input)?;
    Ok(cubes.external_surface_area())
}
