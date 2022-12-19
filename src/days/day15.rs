use std::{collections::VecDeque, str::FromStr};

use crate::common::{AocError, AocResult, IntoAocResult, ParseIntegers};
use itertools::{iproduct, Itertools};

// A single point on a 2D plane.
#[derive(Debug, Clone, Copy)]
struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn manhatten_distance(&self, other: &Point) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn tuning_frequency(&self) -> i64 {
        self.x * 4_000_000 + self.y
    }
}

// A range of integers.
#[derive(Debug, Clone, Copy)]
struct Range {
    pub begin: i64,
    pub end: i64,
}

impl Range {
    pub fn new(begin: i64, end: i64) -> Self {
        Self { begin, end }
    }

    pub fn contains(&self, num: i64) -> bool {
        self.begin <= num && num <= self.end
    }

    pub fn size(&self) -> i64 {
        self.end - self.begin
    }
}

// A scanned area in a Manhatten 2D plane.
struct ScannedArea {
    pub center: Point,
    pub radius: u64,
}

impl ScannedArea {
    // Returns the range of numbers that are contained in this scanned area in the
    // given row, if any.
    pub fn range_on_row(&self, row: i64) -> Option<Range> {
        let distance_to_row = self.center.y.abs_diff(row);
        (self.radius >= distance_to_row).then(|| {
            let width = (self.radius - distance_to_row) as i64;
            Range::new(self.center.x - width, self.center.x + width)
        })
    }
}

// A line segment on a 2D plane.
//
// Represented in slope-intercept form.
#[derive(Debug)]
struct LineSegment {
    x_range: Range,
    slope: f64,
    constant: f64,
}

impl LineSegment {
    pub fn new(a: Point, b: Point) -> Self {
        let slope = ((b.y - a.y) as f64) / ((b.x - a.x) as f64);
        Self {
            x_range: Range::new(a.x.min(b.x), a.x.max(b.x)),
            slope,
            constant: (a.y as f64) - slope * (a.x as f64),
        }
    }

    pub fn has_infinite_slope(&self) -> bool {
        self.slope.is_infinite()
    }

    // Returns the point of intersection, if any, between two lines.
    pub fn intersect(&self, other: &Self) -> Option<Point> {
        if self.slope == other.slope {
            // Parallel lines; no intersection or infinitely many.
            return None;
        }

        let intersection_x = (other.constant - self.constant) / (self.slope - other.slope);
        if !self.x_range.contains(intersection_x as i64)
            || !other.x_range.contains(intersection_x as i64)
        {
            return None;
        }

        Some(Point::new(
            intersection_x as i64,
            self.y_from_x(intersection_x as i64),
        ))
    }

    // Returns the y-coordinate on the line segment with respect to the
    // x-coordinate.
    //
    // Does not check if the x-coordinate is actually on the line segment.
    pub fn y_from_x(&self, x: i64) -> i64 {
        (self.slope * (x as f64) + self.constant) as i64
    }
}

// A square on a 2D plane. Represents the perimeter of a square, not the area
// enclosed by it.
//
// Stores all four vertices and line segments that make up the square.
#[derive(Debug)]
struct Square {
    top: Point,
    bottom: Point,
    left: Point,
    right: Point,
    top_right: LineSegment,
    bottom_right: LineSegment,
    bottom_left: LineSegment,
    top_left: LineSegment,
}

impl Square {
    pub fn surrounding(area: &ScannedArea) -> Self {
        let top = Point::new(area.center.x, area.center.y - (area.radius as i64) - 1);
        let bottom = Point::new(area.center.x, area.center.y + (area.radius as i64) + 1);
        let left = Point::new(area.center.x - (area.radius as i64) - 1, area.center.y);
        let right = Point::new(area.center.x + (area.radius as i64) + 1, area.center.y);
        Self {
            top,
            bottom,
            left,
            right,
            top_right: LineSegment::new(top, right),
            bottom_right: LineSegment::new(right, bottom),
            bottom_left: LineSegment::new(bottom, left),
            top_left: LineSegment::new(left, top),
        }
    }

    pub fn edges(&self) -> [&LineSegment; 4] {
        [
            &self.top_right,
            &self.bottom_right,
            &self.bottom_left,
            &self.top_left,
        ]
    }

    // Returns all points of intersection between the two square perimeters.
    pub fn intersect(&self, other: &Self) -> Vec<Point> {
        iproduct!(self.edges(), other.edges())
            .filter_map(|(a, b)| a.intersect(&b))
            .collect()
    }

    // Checks if the point is contained within the square perimeter, excluding the
    // perimeter itself.
    pub fn contains(&self, Point { x, y }: &Point) -> bool {
        if self.edges().iter().any(|edge| edge.has_infinite_slope()) {
            // Square is aligned to the grid, which means we just need to check ranges.
            Range::new(self.left.x, self.right.x).contains(*x)
                && Range::new(self.bottom.y, self.top.y).contains(*y)
        } else {
            // Check that the y-coordinate fits within the bounds of the edges.

            // Below the top-right edge.
            *y > self.top_right.y_from_x(*x)
                // Above the bottom-right edge.
                && *y < self.bottom_right.y_from_x(*x)
                // Above the bottom-left edge.
                && *y < self.bottom_left.y_from_x(*x)
                // Below the top-left edge.
                && *y > self.top_left.y_from_x(*x)
        }
    }
}

struct Reading {
    pub sensor: Point,
    pub closest_beacon: Point,
}

impl Reading {
    pub fn into_scanned_area(self) -> ScannedArea {
        ScannedArea {
            center: self.sensor,
            radius: self.sensor.manhatten_distance(&self.closest_beacon),
        }
    }
}

impl FromStr for Reading {
    type Err = AocError;
    fn from_str(s: &str) -> AocResult<Self> {
        let mut ints = s.parse_signed_integers(10);
        Ok(Reading {
            sensor: Point::new(
                ints.next()
                    .into_aoc_result_msg("missing x coordinate for sensor")?,
                ints.next()
                    .into_aoc_result_msg("missing y coordinate for sensor")?,
            ),
            closest_beacon: Point::new(
                ints.next()
                    .into_aoc_result_msg("missing x coordinate for closest beacon")?,
                ints.next()
                    .into_aoc_result_msg("missing y coordinate for closest beacon")?,
            ),
        })
    }
}

fn parse_readings(input: &str) -> AocResult<Vec<Reading>> {
    input.lines().map(|line| Reading::from_str(line)).collect()
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    const ROW: i64 = 2_000_000;
    let ranges = parse_readings(input)?
        .into_iter()
        .filter_map(|reading| reading.into_scanned_area().range_on_row(ROW))
        .sorted_by(|a, b| a.begin.cmp(&b.begin))
        .collect::<Vec<_>>();
    if ranges.is_empty() {
        return Err(AocError::new("no ranges"));
    }

    // Merge all ranges for this row into a stack of ranges, eliminating any
    // duplicates.
    let no_beacon_ranges = ranges.iter().skip(1).fold(
        VecDeque::from([ranges.first().copied().unwrap()]),
        |mut stack, &range| {
            let top = stack.back_mut().unwrap();
            if top.contains(range.begin) {
                // Merge the two ranges if they overlap.
                top.end = top.end.max(range.end);
            } else {
                // No overlap, add a new range.
                stack.push_back(range);
            }
            stack
        },
    );

    Ok(no_beacon_ranges
        .into_iter()
        .map(|range| range.size() as u64)
        .sum())
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    const BEACON_MIN: i64 = 0;
    const BEACON_MAX: i64 = 4_000_000;
    let beacon_range = Range::new(BEACON_MIN, BEACON_MAX);

    // Convert each reading into its perimeter, which is the square one step outside
    // of the scanned area.
    let squares = parse_readings(input)?
        .into_iter()
        .map(|sensor| Square::surrounding(&sensor.into_scanned_area()))
        .collect::<Vec<_>>();

    // For each pair of squares, find all points of intersection.
    // Return the first point of intersection that does not contained by any square.
    for (square, other) in squares.iter().tuple_combinations() {
        for intersection in square.intersect(other) {
            if beacon_range.contains(intersection.x)
                && beacon_range.contains(intersection.y)
                && squares.iter().all(|square| !square.contains(&intersection))
            {
                return Ok(intersection.tuning_frequency() as u64);
            }
        }
    }

    Err(AocError::new("no beacon found"))
}
