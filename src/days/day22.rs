use std::{
    collections::{HashSet, VecDeque},
    ops::{Add, Sub},
    str::FromStr,
};

use crate::common::{AocError, AocResult, IntoAocResult, NewlineBlocks};
use itertools::Itertools;
use num::{FromPrimitive, ToPrimitive};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn in_range(&self, min: &Point, max: &Point) -> bool {
        min.x <= self.x && self.x < max.x && min.y <= self.y && self.y < max.y
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Debug)]
enum Instruction {
    Move(u64),
    RotateLeft,
    RotateRight,
}

fn parse_instructions(input: &str) -> AocResult<Vec<Instruction>> {
    let mut instructions = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            'L' => instructions.push(Instruction::RotateLeft),
            'R' => instructions.push(Instruction::RotateRight),
            '0'..='9' => {
                let mut n = c.to_digit(10).unwrap();
                while let Some(c) = chars.peek() {
                    if c.is_digit(10) {
                        n = 10 * n + c.to_digit(10).unwrap();
                        chars.next();
                    } else {
                        break;
                    }
                }
                instructions.push(Instruction::Move(n as u64));
            }
            _ => {
                return Err(AocError::new(&format!(
                    "invalid instruction character: {c}"
                )))
            }
        }
    }
    Ok(instructions)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(i8)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    pub const COUNT: usize = 4;

    pub fn is_horizontal(&self) -> bool {
        self.to_i8().unwrap() % 2 == 0
    }

    pub fn is_vertical(&self) -> bool {
        !self.is_horizontal()
    }

    pub fn index(&self) -> usize {
        self.to_usize().unwrap()
    }

    pub fn rotate_left(&self) -> Self {
        Self::from_i8((self.to_i8().unwrap() - 1).rem_euclid(Self::COUNT as i8)).unwrap()
    }

    pub fn rotate_right(&self) -> Self {
        Self::from_i8((self.to_i8().unwrap() + 1).rem_euclid(Self::COUNT as i8)).unwrap()
    }

    pub fn inverse(&self) -> Self {
        Self::from_i8((self.to_i8().unwrap() + 2).rem_euclid(Self::COUNT as i8)).unwrap()
    }

    pub fn delta(&self) -> Point {
        match self {
            Self::Right => Point::new(1, 0),
            Self::Down => Point::new(0, 1),
            Self::Left => Point::new(-1, 0),
            Self::Up => Point::new(0, -1),
        }
    }
}

// A single block of uniform width in the monkey map.
#[derive(Debug)]
struct MonkeyMapBlock {
    pub min: Point,
    pub max: Point,
    pub walls: HashSet<Point>,
}

impl MonkeyMapBlock {
    pub fn width(&self) -> i64 {
        self.max.x - self.min.x + 1
    }

    pub fn height(&self) -> i64 {
        self.max.y - self.min.y + 1
    }
}

trait Traversable {
    fn follow(&self, instructions: Vec<Instruction>) -> AocResult<(Point, Direction)>;
}

// A monkey map, which consists of several blocks with wraparounds.
#[derive(Debug)]
struct MonkeyMap {
    blocks: Vec<MonkeyMapBlock>,
}

impl Traversable for MonkeyMap {
    fn follow(&self, instructions: Vec<Instruction>) -> AocResult<(Point, Direction)> {
        if self.blocks.is_empty() {
            return Err(AocError::new("map is empty"));
        }
        let mut current_block_index = 0;
        let mut current_block = &self.blocks[current_block_index];
        let mut position = current_block.min;
        let mut dir = Direction::Right;
        for instruction in instructions {
            match instruction {
                Instruction::RotateLeft => dir = dir.rotate_left(),
                Instruction::RotateRight => dir = dir.rotate_right(),
                Instruction::Move(n) => {
                    let delta = dir.delta();
                    for _ in 0..n {
                        let mut next = position + delta;

                        // Wrap around for x coordinate.
                        // Going off the left or right side will always result in us being in the
                        // same block.
                        if next.x < current_block.min.x {
                            next.x = current_block.max.x;
                        } else if next.x > current_block.max.x {
                            next.x = current_block.min.x;
                        }

                        // Wrap around for y coordinate.
                        // Going off the top or bottom may potentially put us in a new block.
                        if next.y < current_block.min.y {
                            // Went off the top, so we may be in another block or we may wrap around
                            // to the bottom of our current block.
                            let previous_block_index = if current_block_index == 0 {
                                self.blocks.len() - 1
                            } else {
                                current_block_index - 1
                            };
                            let previous_block = &self.blocks[previous_block_index];
                            if previous_block.min.x <= position.x
                                && position.x <= previous_block.max.x
                            {
                                // We are in the previous block.
                                current_block_index = previous_block_index;
                                current_block = previous_block;
                            }

                            next.y = current_block.max.y;
                        } else if next.y > current_block.max.y {
                            // Went off the bottom, so we may be in another block or we may wrap
                            // around to the top of our current block.
                            let next_block_index = current_block_index + 1;
                            let next_block_index = if next_block_index >= self.blocks.len() {
                                0
                            } else {
                                next_block_index
                            };
                            let next_block = &self.blocks[next_block_index];
                            if next_block.min.x <= position.x && position.x <= next_block.max.x {
                                // We are in the next block.
                                current_block_index = next_block_index;
                                current_block = next_block;
                            }

                            next.y = current_block.min.y;
                        }

                        // Now that we know where we are going, we make sure we do not hit a wall.
                        if current_block.walls.contains(&next) {
                            break;
                        }

                        position = next;
                    }
                }
            }
        }

        Ok((position, dir))
    }
}

impl FromStr for MonkeyMap {
    type Err = AocError;
    fn from_str(s: &str) -> AocResult<Self> {
        let mut blocks = Vec::new();
        let mut current_block: Option<MonkeyMapBlock> = None;
        let create_new_block = |y, x_min, x_max| MonkeyMapBlock {
            min: Point::new(x_min, y),
            max: Point::new(x_max, y),
            walls: HashSet::new(),
        };
        let mut y = 0;
        for line in s.lines() {
            // Find left and right bounds of the map.
            let x_min = line.find(|c: char| !c.is_whitespace());
            let x_max = line.rfind(|c: char| !c.is_whitespace());
            let (x_min, x_max) = match (x_min, x_max) {
                (None, _) | (_, None) => return Err(AocError::new("line has no mapped tiles")),
                (Some(x_min), Some(x_max)) => (x_min <= x_max)
                    .then_some((x_min, x_max))
                    .into_aoc_result_msg("invalid minimum and maximum x coordinates")?,
            };
            // This line fits in the same block as the previous line if the left and right
            // bounds are the same as the current block.
            // If not, we must create a new block.
            current_block = match current_block {
                None => Some(create_new_block(y as i64, x_min as i64, x_max as i64)),
                Some(mut block) => {
                    if block.min.x != x_min as i64 || block.max.x != x_max as i64 {
                        block.max.y = y as i64 - 1;
                        blocks.push(block);
                        Some(create_new_block(y as i64, x_min as i64, x_max as i64))
                    } else {
                        Some(block)
                    }
                }
            };
            for (x, _) in line[x_min..=x_max]
                .char_indices()
                .filter(|&(_, c)| c == '#')
            {
                current_block
                    .as_mut()
                    .unwrap()
                    .walls
                    .insert(Point::new((x_min + x) as i64, y as i64));
            }

            y += 1;
        }

        // Push last block in.
        if let Some(mut block) = current_block {
            block.max.y = y as i64 - 1;
            blocks.push(block);
        }

        Ok(Self { blocks })
    }
}

fn parse_map_and_instructions(input: &str) -> AocResult<(MonkeyMap, Vec<Instruction>)> {
    let (map, instructions) = input
        .newline_blocks(2)
        .collect_tuple()
        .into_aoc_result_msg("invalid input")?;
    Ok((
        MonkeyMap::from_str(map)?,
        parse_instructions(instructions.trim())?,
    ))
}

// The direction a cube face is facing when laying down parallel to the ground.
//
// It is either face up or face down (mirrored).
#[derive(Debug, Default, Clone, Copy)]
#[repr(u8)]
enum Facing {
    #[default]
    FaceUp,
    FaceDown,
}

// An even rotation (multiple of 90 degrees) of a cube face.
//
// Rotatations follow a counterclockwise direction:
//      0
//  90    270
//     180
#[derive(Debug, Default, Clone, Copy, FromPrimitive, ToPrimitive)]
#[repr(i8)]
enum Rotation {
    #[default]
    Zero = 0,
    Ninety = 1,
    OneEighty = 2,
    TwoSeventy = 3,
}

impl Rotation {
    pub const COUNT: usize = 4;

    // Applies the rotation to the given direction in the counterclockwise
    // direction.
    pub fn apply(&self, mut dir: Direction) -> Direction {
        for _ in 0..self.to_u8().unwrap() {
            dir = dir.rotate_left();
        }
        dir
    }

    // Increments the rotation.
    pub fn rotate_left(&self) -> Rotation {
        Self::from_i8((self.to_i8().unwrap() + 1).rem_euclid(Self::COUNT as i8)).unwrap()
    }

    // Mirrors the rotation across the relevant axis.
    pub fn mirror(&self) -> Rotation {
        Self::from_i8((self.to_i8().unwrap() + 2).rem_euclid(Self::COUNT as i8)).unwrap()
    }

    // Calculates the rotational difference between two directions.
    pub fn difference(from: Direction, to: Direction) -> Self {
        let mut dir = from;
        let mut diff = Self::Zero;
        while dir != to {
            dir = dir.rotate_left();
            diff = diff.rotate_left();
        }
        diff
    }
}

// A single cube face that rotates around, imitiating how a cube net is folded.
#[derive(Debug, Clone, Copy)]
enum RotatingCubeFace {
    // When a cube face is lying flat, we must know whether it is face up or down and how it is
    // rotated.
    Flat(Facing, Rotation),
    // When a cube is standing up, we must know what edge it is standing on and which direction it
    // is facing. The facing direction always faces the inside of the cube.
    Standing(Direction, Direction),
}

impl Default for RotatingCubeFace {
    fn default() -> Self {
        Self::Flat(Facing::default(), Rotation::default())
    }
}

impl RotatingCubeFace {
    // Rotate the cube face in the given direction.
    //
    // To be honest, this code is pretty fragile. Some of the logic is quite sound,
    // but some of the conditions are not intuitive from a first glance.
    pub fn rotate(&self, dir: Direction) -> Self {
        match &self {
            // The cube is laying flat. Any rotation will stand it up.
            Self::Flat(facing, rotated) => Self::Standing(
                match facing {
                    // Face up, rotate clockwise.
                    Facing::FaceUp => rotated.mirror().apply(dir.inverse()),
                    // Face down, rotate counterclockwise.
                    Facing::FaceDown => rotated.apply(if dir.is_horizontal() {
                        // Direction is flipped if we are rotating horizontal.
                        dir.inverse()
                    } else {
                        dir
                    }),
                },
                match facing {
                    Facing::FaceUp => dir,
                    Facing::FaceDown => dir.inverse(),
                },
            ),
            // The cube is standing up. It could potentially be laid flat, or it could just rotate
            // to another standing position.
            Self::Standing(standing_on, facing) => {
                if facing.is_vertical() && dir.is_vertical() {
                    // If we rotate in the direction we are already facing, we will be face down.
                    let flat_facing = if dir == *facing {
                        Facing::FaceDown
                    } else {
                        Facing::FaceUp
                    };

                    let mut rotated = Rotation::difference(*standing_on, *facing);
                    if standing_on == &dir {
                        rotated = rotated.mirror();
                    }
                    Self::Flat(flat_facing, rotated)
                } else if facing.is_horizontal() && dir.is_horizontal() {
                    // If we rotate in the direction we are already facing, we will be face down.
                    let flat_facing = if dir == *facing {
                        Facing::FaceDown
                    } else {
                        Facing::FaceUp
                    };

                    let rotated = if standing_on == facing {
                        Rotation::Zero
                    } else if standing_on == &facing.inverse() {
                        Rotation::OneEighty
                    } else {
                        Rotation::difference(dir, *standing_on)
                    };
                    Self::Flat(flat_facing, rotated)
                } else {
                    // Rotating to another standing state.
                    let rotation = Rotation::difference(*facing, dir);
                    Self::Standing(rotation.apply(*standing_on), *facing)
                }
            }
        }
    }
}

// A single cube of the monkey map folded as a cube.
#[derive(Debug)]
struct MonkeyCubeFace {
    pub min: Point,
    pub max: Point,
    pub walls: HashSet<Point>,
    pub neighbors: [usize; Direction::COUNT],
}

// The monkey map correctly folded as a cube.
#[derive(Debug)]
struct MonkeyCube {
    face_length: i64,
    faces: [MonkeyCubeFace; 6],
}

impl TryFrom<MonkeyMap> for MonkeyCube {
    type Error = AocError;
    fn try_from(map: MonkeyMap) -> AocResult<Self> {
        let cube_face_length = map.blocks.iter().map(|block| block.height()).max().unwrap();

        // First, convert all blocks to faces.
        let mut faces = Vec::new();
        for block in &map.blocks {
            // One block can contain multiple cube faces, so we need to segment it into even
            // cube faces.
            let x_blocks = block.width() / cube_face_length;
            let y_blocks = block.height() / cube_face_length;
            for i in 0..x_blocks {
                for j in 0..y_blocks {
                    // Get the start and end range for this face.
                    let min = Point::new(
                        block.min.x + i * cube_face_length,
                        block.min.y + j * cube_face_length,
                    );
                    let max = min + Point::new(cube_face_length, cube_face_length);
                    // Get all walls in this block that belong on this cube face.
                    let walls_on_face = block
                        .walls
                        .iter()
                        .filter(|point| point.in_range(&min, &max))
                        .map(|&point| point - min)
                        .collect();
                    // Add the cube face, along with its coordinates in the block map for use in the
                    // next step. The neighbors field is a placeholder until it is fully
                    // constructed.
                    faces.push(MonkeyCubeFace {
                        min,
                        max,
                        walls: walls_on_face,
                        neighbors: [usize::MAX; Direction::COUNT],
                    })
                }
            }
        }
        if faces.len() != 6 {
            return Err(AocError::new(&format!(
                "expected 6 faces, found {}",
                faces.len()
            )));
        }

        // Next, we need to construct how each face relates to one another. We use the
        // block map again, because it should represent a cube net.
        //
        // Start by constructing the cube net as it is represented by the flat monkey
        // map. This creates a cube net.
        let mut cube_net = [[None; Direction::COUNT]; 6];
        for i in 0..faces.len() {
            let MonkeyCubeFace { min, max, .. } = faces[i];

            if cube_net[i][Direction::Right.index()].is_none() {
                let right = Point::new(max.x, min.y);
                if let Some(right_index) = faces
                    .iter()
                    .position(|face| right.in_range(&face.min, &face.max))
                {
                    cube_net[i][Direction::Right.index()] = Some(right_index);
                    cube_net[right_index][Direction::Left.index()] = Some(i);
                }
            }

            if cube_net[i][Direction::Left.index()].is_none() {
                let left = min - Point::new(1, 0);
                if let Some(left_index) = faces
                    .iter()
                    .position(|face| left.in_range(&face.min, &face.max))
                {
                    cube_net[i][Direction::Left.index()] = Some(left_index);
                    cube_net[left_index][Direction::Right.index()] = Some(i);
                }
            }

            if cube_net[i][Direction::Up.index()].is_none() {
                let up = min - Point::new(0, 1);
                if let Some(up_index) = faces
                    .iter()
                    .position(|face| up.in_range(&face.min, &face.max))
                {
                    cube_net[i][Direction::Up.index()] = Some(up_index);
                    cube_net[up_index][Direction::Down.index()] = Some(i);
                }
            }

            if cube_net[i][Direction::Down.index()].is_none() {
                let down = Point::new(min.x, max.y);
                if let Some(down_index) = faces
                    .iter()
                    .position(|face| down.in_range(&face.min, &face.max))
                {
                    cube_net[i][Direction::Down.index()] = Some(down_index);
                    cube_net[down_index][Direction::Up.index()] = Some(i);
                }
            }
        }

        // At this point, the whole cube net is connected by some order of edges.
        // Now, we run BFS from each face, walking along the cube map and rotating that
        // cube face as we go.
        //
        // Each rotation should clue us into a new neighbor: if the cube face is
        // standing on a given edge after a move, we have found a new neighbor.
        let mut folded_cube_net = cube_net;
        for i in 0..faces.len() {
            let mut queue = VecDeque::from([i]);
            let mut seen = [false; 6];
            let mut state = [RotatingCubeFace::default(); 6];
            seen[i] = true;
            state[i] = RotatingCubeFace::Flat(Facing::FaceUp, Rotation::Zero);
            while let Some(position) = queue.pop_front() {
                for edge in 0..Direction::COUNT {
                    if let Some(neighbor) = cube_net[position][edge] {
                        if seen[neighbor] {
                            continue;
                        }

                        let next_state =
                            state[position].rotate(Direction::from_usize(edge).unwrap());
                        if let RotatingCubeFace::Standing(standing_on, _) = next_state {
                            // New neighbor in the direction of the edge we have rotated to stand
                            // on.
                            folded_cube_net[i][standing_on.to_usize().unwrap()] = Some(neighbor);
                        }

                        seen[neighbor] = true;
                        state[neighbor] = next_state;
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        // Assign our completed neighbor map to each cube face.
        for i in 0..faces.len() {
            for dir in 0..Direction::COUNT {
                faces[i].neighbors[dir] = folded_cube_net[i][dir].into_aoc_result_msg(&format!(
                    "missing neighbor on {:?} edge for face {i}",
                    Direction::from_usize(dir).unwrap()
                ))?;
            }
        }

        Ok(Self {
            face_length: cube_face_length,
            faces: faces.try_into().unwrap(),
        })
    }
}

impl MonkeyCube {
    // Returns the cube face and edge that neighbors the given face on the given
    // edge.
    fn get_neighbor(&self, face: usize, edge: Direction) -> (usize, Direction) {
        let next_face = self.faces[face].neighbors[edge.index()];
        let next_edge = Direction::from_usize(
            self.faces[next_face]
                .neighbors
                .iter()
                .position(|&neighbor| neighbor == face)
                .unwrap(),
        )
        .unwrap();
        (next_face, next_edge)
    }
}

impl Traversable for MonkeyCube {
    fn follow(&self, instructions: Vec<Instruction>) -> AocResult<(Point, Direction)> {
        // Traverse the cube with each cube face having its own coordinate space.
        // The point we land on will be converted to the original coordinate space in
        // the end.
        let mut current_face = 0;
        let mut position = Point::new(0, 0);
        let mut dir = Direction::Right;
        for instruction in instructions {
            match instruction {
                Instruction::RotateLeft => dir = dir.rotate_left(),
                Instruction::RotateRight => dir = dir.rotate_right(),
                Instruction::Move(n) => {
                    for _ in 0..n {
                        let next_position = position + dir.delta();

                        // Check if we have wrapped around the cube.
                        let wrapped = if next_position.x < 0 {
                            Some(self.get_neighbor(current_face, Direction::Left))
                        } else if next_position.x >= self.face_length {
                            Some(self.get_neighbor(current_face, Direction::Right))
                        } else if next_position.y < 0 {
                            Some(self.get_neighbor(current_face, Direction::Up))
                        } else if next_position.y >= self.face_length {
                            Some(self.get_neighbor(current_face, Direction::Down))
                        } else {
                            None
                        };

                        let (next_face, next_position, next_dir) = match wrapped {
                            None => (current_face, next_position, dir),
                            Some((next_face, on_edge)) => {
                                let next_dir = on_edge.inverse();
                                let next_x = match on_edge {
                                    Direction::Right => self.face_length - 1,
                                    Direction::Left => 0,
                                    Direction::Down | Direction::Up => {
                                        match Rotation::difference(dir, next_dir) {
                                            Rotation::Zero => position.x,
                                            Rotation::Ninety => position.y,
                                            Rotation::OneEighty => {
                                                self.face_length - position.x - 1
                                            }
                                            Rotation::TwoSeventy => {
                                                self.face_length - position.y - 1
                                            }
                                        }
                                    }
                                };
                                let next_y = match on_edge {
                                    Direction::Down => self.face_length - 1,
                                    Direction::Up => 0,
                                    Direction::Right | Direction::Left => {
                                        match Rotation::difference(dir, next_dir) {
                                            Rotation::Zero => position.y,
                                            Rotation::Ninety => self.face_length - position.x - 1,
                                            Rotation::OneEighty => {
                                                self.face_length - position.y - 1
                                            }
                                            Rotation::TwoSeventy => position.x,
                                        }
                                    }
                                };
                                (next_face, Point::new(next_x, next_y), next_dir)
                            }
                        };

                        // Now that we know where we are going, we make sure we do not hit a wall.
                        if self.faces[next_face].walls.contains(&next_position) {
                            break;
                        }

                        (current_face, position, dir) = (next_face, next_position, next_dir);
                    }
                }
            }
        }

        // Position is relative to the current cube face.
        Ok((position + self.faces[current_face].min, dir))
    }
}

fn final_password(position: Point, dir: Direction) -> AocResult<u64> {
    let password = 1000 * (position.y + 1) + 4 * (position.x + 1) + dir.to_i64().unwrap();
    password.try_into().into_aoc_result()
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    let (map, instructions) = parse_map_and_instructions(input)?;
    let (position, dir) = map.follow(instructions)?;
    final_password(position, dir)
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    let (map, instructions) = parse_map_and_instructions(input)?;
    let cube = MonkeyCube::try_from(map)?;
    let (position, dir) = cube.follow(instructions)?;
    final_password(position, dir)
}
