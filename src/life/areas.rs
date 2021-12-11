//! # Areas
//!
//! Positions, areas, neighbours and anything around,
//! such as checking if something is on an area.
//!
//! ## Author
//! Ngoc (Nox) Le <noxsense@gmail.com>

/// Simple position in world.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct Position {
    pub x: u32, // left to right (width)
    pub y: u32, // front to back (depth)
    pub z: u32, // bottom to top (height)
}

impl Position {

    pub const ROOT: Self
        = Self { x: 0, y: 0, z: 0 };

    /// Simple constructor.
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    /// Get the distance between two positions.
    pub fn distance_to(&self, other: &Self) -> f32 {
        (((self.x as i64 - other.x as i64).pow(2) + (self.y as i64 - other.y as i64).pow(2)) as f32)
            .sqrt()
    }
}

/// Simple position in world.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Area {
    anchor: Position,
    width: u32,  // left to right
    depth: u32,  // front to back
    height: u32, // bottom to top
    position_index_bound: usize,
    iterator_index: u32,
}

impl Area {
    pub fn new(anchor: Position, width: u32, depth: u32, height: u32) -> Self {
        let position_index_bound = (width as usize) * (depth as usize) * (height as usize);
        Self {
            anchor,
            width,
            depth,
            height,
            position_index_bound,
            iterator_index: 0,
        }
    }

    /// Create an area, that is spanned by the given positions.
    pub fn span(a: &Position, b: &Position) -> Self {
        let (min_x, max_x) = (<u32>::min(a.x, b.x), <u32>::max(a.x, b.x));
        let (min_y, max_y) = (<u32>::min(a.y, b.y), <u32>::max(a.y, b.y));
        let (min_z, max_z) = (<u32>::min(a.z, b.z), <u32>::max(a.z, b.z));

        let width = <u32>::max(1, max_x - min_x);
        let depth = <u32>::max(1, max_y - min_y);
        let height = <u32>::max(1, max_z - min_z);

        Area::new(
            Position { x: min_x, y: min_y, z: min_z },
            width, depth, height
            )
    }

    /// Check, if the position is in the area.
    pub fn contains_position(&self, pos: &Position) -> bool {
        (self.anchor.x <= pos.x && pos.x < (self.anchor.x + self.width))
            && (self.anchor.y <= pos.y && pos.y < (self.anchor.y + self.depth))
            && (self.anchor.z <= pos.z && pos.z < (self.anchor.z + self.height))
    }

    /// Get a random position within this area.
    pub fn position_random(&self) -> Position {
        Position {
            x: self.anchor.x + (rand::random::<u32>() % (self.anchor.x + self.width)),
            y: self.anchor.y + (rand::random::<u32>() % (self.anchor.y + self.depth)),
            z: self.anchor.z + (rand::random::<u32>() % (self.anchor.z + self.height)),
        }
    }

    /// Get all valid neighbours of a position within the area.
    pub fn get_all_neighbours_xy(&self, pos: Position) -> Vec<Position> {
        // TODO (maka a storage, to not calculate it every time. )
        let mut neighbours: Vec<Position> = vec![];

        // Get all the valid neighbours.
        for d in Direction::DIRECTION_LIST_XY.iter() {
            if let Some(n) = self.get_directed_neighbour(pos, *d) {
                neighbours.push(n);
            }
        }
        neighbours
    }

    /// Get a requested neighbour of a given position within this area.
    pub fn get_directed_neighbour(&self, pos: Position, direction: Direction) -> Option<Position> {
        let change = direction.as_offset_tuple();

        let box_width = self.anchor.x + self.width;
        let box_depth = self.anchor.y + self.depth;
        let box_height = self.anchor.z + self.height;

        // On west border => No west neighbours. (None)
        if pos.x < 1 && change.0 < 0 {
            return None;
        }

        // On east border => No east neighbours. (None)
        if pos.x >= box_width && change.0 > 0 {
            return None;
        }

        // On south border => No south neighbours. (None)
        if pos.y < 1 && change.1 < 0 {
            return None;
        }

        // On north border => No north neighbours. (None)
        if pos.y >= box_depth && change.1 > 0 {
            return None;
        }

        // On south border => No south neighbours. (None)
        if pos.z < 1 && change.2 < 0 {
            return None;
        }

        // On north border => No north neighbours. (None)
        if pos.z >= box_height && change.2 > 0 {
            return None;
        }

        Some(Position::new(
            (pos.x as i64 + change.0 as i64) as u32,
            (pos.y as i64 + change.1 as i64) as u32,
            (pos.z as i64 + change.2 as i64) as u32,
        ))
    }

    /// Get the optional position, which is on the given index.
    pub fn position_from_index(&self, index: u32) -> Option<Position> {
        if index < self.width * self.depth {
            Some(Position::new(
                index % self.width + self.anchor.x,
                index / self.width + self.anchor.y,
                0u32, // XXX
            ))
        } else {
            None
        }
    }
}

impl Iterator for Area {
    type Item = Position;

    /// Iterator over the positions of the field.
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.iterator_index;
        self.iterator_index += 1;
        self.position_from_index(index)
    }
}

/// Diretional Offset from any Position (3D)
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct Direction {
    pub x: Step,
    pub y: Step,
    pub z: Step,
}

impl Direction {
    // width x depth slice.
    const DIRECTION_LIST_XY: [Self; 8 ] = [
        Self { x: Step::Backward,  y: Step::Backward, z: Step::Stay },
        Self { x: Step::Stay,      y: Step::Backward, z: Step::Stay },
        Self { x: Step::Forward,   y: Step::Backward, z: Step::Stay },
        // y change
        Self { x: Step::Backward,  y: Step::Stay,     z: Step::Stay },
        // Stay Stay Stay => No actual direction.
        Self { x: Step::Forward,   y: Step::Stay,     z: Step::Stay },
        // y change
        Self { x: Step::Backward,  y: Step::Forward,  z: Step::Stay },
        Self { x: Step::Stay,      y: Step::Forward,  z: Step::Stay },
        Self { x: Step::Forward,   y: Step::Forward,  z: Step::Stay },
    ];

    // depth x height slice.
    #[allow(dead_code)]
    const DIRECTION_LIST_YZ: [Self; 8 ] = [
        Self { x: Step::Stay, y: Step::Backward,  z: Step::Backward, },
        Self { x: Step::Stay, y: Step::Stay,      z: Step::Backward, },
        Self { x: Step::Stay, y: Step::Forward,   z: Step::Backward, },
        // z change
        Self { x: Step::Stay, y: Step::Backward,  z: Step::Stay,     },
        // Stay Stay Stay => No actual direction.
        Self { x: Step::Stay, y: Step::Forward,   z: Step::Stay,     },
        // z change
        Self { x: Step::Stay, y: Step::Backward,  z: Step::Forward,  },
        Self { x: Step::Stay, y: Step::Stay,      z: Step::Forward,  },
        Self { x: Step::Stay, y: Step::Forward,   z: Step::Forward,  },
    ];

    // width x height slice.
    #[allow(dead_code)]
    const DIRECTION_LIST_XZ: [Self; 8 ] = [
        Self { x: Step::Backward, y: Step::Stay, z: Step::Backward },
        Self { x: Step::Stay,     y: Step::Stay, z: Step::Backward },
        Self { x: Step::Forward,  y: Step::Stay, z: Step::Backward },
        // z change
        Self { x: Step::Backward, y: Step::Stay, z: Step::Stay },
        // Stay Stay Stay => No actual direction.
        Self { x: Step::Forward,  y: Step::Stay, z: Step::Stay },
        // z change
        Self { x: Step::Backward, y: Step::Stay, z: Step::Forward },
        Self { x: Step::Stay,     y: Step::Stay, z: Step::Forward },
        Self { x: Step::Forward,  y: Step::Stay, z: Step::Forward },
    ];

    // all directions 3D.
    #[allow(dead_code)]
    const DIRECTION_LIST_XYZ: [Self; 26 ] = [
        Self { x: Step::Backward, y: Step::Backward, z: Step::Backward },
        Self { x: Step::Stay,     y: Step::Backward, z: Step::Backward },
        Self { x: Step::Forward,  y: Step::Backward, z: Step::Backward },
        // y change
        Self { x: Step::Backward, y: Step::Stay, z: Step::Backward },
        Self { x: Step::Stay,     y: Step::Stay, z: Step::Backward },
        Self { x: Step::Forward,  y: Step::Stay, z: Step::Backward },
        // y change
        Self { x: Step::Backward, y: Step::Forward, z: Step::Backward },
        Self { x: Step::Stay,     y: Step::Forward, z: Step::Backward },
        Self { x: Step::Forward,  y: Step::Forward, z: Step::Backward },
        //
        // z change
        Self { x: Step::Backward, y: Step::Backward, z: Step::Stay },
        Self { x: Step::Stay,     y: Step::Backward, z: Step::Stay },
        Self { x: Step::Forward,  y: Step::Backward, z: Step::Stay },
        // y change
        Self { x: Step::Backward, y: Step::Stay, z: Step::Stay },
        // Stay Stay Stay => No actual direction.
        Self { x: Step::Forward,  y: Step::Stay, z: Step::Stay },
        // y change
        Self { x: Step::Backward, y: Step::Forward, z: Step::Stay },
        Self { x: Step::Stay,     y: Step::Forward, z: Step::Stay },
        Self { x: Step::Forward,  y: Step::Forward, z: Step::Stay },
        //
        // z change
        Self { x: Step::Backward, y: Step::Backward, z: Step::Forward },
        Self { x: Step::Stay,     y: Step::Backward, z: Step::Forward },
        Self { x: Step::Forward,  y: Step::Backward, z: Step::Forward },
        // y change
        Self { x: Step::Backward, y: Step::Stay, z: Step::Forward },
        Self { x: Step::Stay,     y: Step::Stay, z: Step::Forward },
        Self { x: Step::Forward,  y: Step::Stay, z: Step::Forward },
        // y change
        Self { x: Step::Backward, y: Step::Forward, z: Step::Forward },
        Self { x: Step::Stay,     y: Step::Forward, z: Step::Forward },
        Self { x: Step::Forward,  y: Step::Forward, z: Step::Forward },
    ];

    pub fn as_offset_tuple(&self) -> (i8, i8, i8) {
        (self.x.as_offset(), self.y.as_offset(), self.z.as_offset())
    }
}

/// Option to go forward, backward or stay.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Step {
    Backward,
    Stay,
    Forward,
}

impl Step {
    pub fn as_offset(&self) -> i8 {
        match self {
            Self::Backward => -1,
            Self::Stay => 0,
            Self::Forward => 1,
        }
    }
}
