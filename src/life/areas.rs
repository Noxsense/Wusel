/**
 * Module areas ( and positions and more).
 *
 * @author Nox
 * @version 2021.0.1
 */

/** Simple position in world. */
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    /** Simple constructor. */
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /** Get the distance between two positions. */
    pub fn distance_to(&self, other: &Self) -> f32 {
        (((self.x as i64 - other.x as i64).pow(2) + (self.y as i64 - other.y as i64).pow(2)) as f32)
            .sqrt()
    }
}

/** Simple position in world. */
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Area {
    anchor: Position,
    width: u32,
    depth: u32,
    iterator_index: u32,
}

impl Area {
    pub fn new(anchor: Position, width: u32, depth: u32) -> Self {
        Self {
            anchor,
            width,
            depth,
            iterator_index: 0,
        }
    }

    /** Create an area, that is spanned by the given positions. */
    pub fn span(a: &Position, b: &Position) -> Self {
        let (min_x, max_x) = (<u32>::min(a.x, b.x), <u32>::max(a.x, b.x));
        let (min_y, max_y) = (<u32>::min(a.y, b.y), <u32>::max(a.y, b.y));

        Area {
            anchor: Position::new(min_x, min_y),
            /* If only one position is spanned: width/depth: 1. */
            width: <u32>::max(1, max_x - min_x),
            depth: <u32>::max(1, max_y - min_y),
            iterator_index: 0,
        }
    }

    /** Check, if the position is in the area. */
    pub fn contains_position(&self, pos: &Position) -> bool {
        (self.anchor.x <= pos.x && pos.x < (self.anchor.x + self.width))
            && (self.anchor.y <= pos.y && pos.y < (self.anchor.y + self.depth))
    }

    /** Get a random position within this area. */
    pub fn position_random(&self) -> Position {
        Position::new(
            self.anchor.x + (rand::random::<u32>() % (self.anchor.x + self.width)),
            self.anchor.y + (rand::random::<u32>() % (self.anchor.y + self.depth)),
        )
    }

    /** Get all valid neighbours of a position within the area. */
    pub fn get_all_neighbours(&self, pos: Position) -> Vec<Position> {
        // TODO (maka a storage, to not calculate it every time. )
        let mut neighbours: Vec<Position> = vec![];

        /* Get all the valid neighbours. */
        for d in Way::NEIGHBOURING.iter() {
            if let Some(n) = self.get_directed_neighbour(pos, *d) {
                neighbours.push(n);
            }
        }
        neighbours
    }

    /** Get a requested neighbour of a given position within this area. */
    pub fn get_directed_neighbour(&self, pos: Position, direction: Way) -> Option<Position> {
        let change = direction.as_direction_tuple();

        let box_width = self.anchor.x + self.width;
        let box_depth = self.anchor.y + self.depth;

        /* On west border => No west neighbours. (None) */
        if pos.x < 1 && change.0 < 0 {
            return None;
        }

        /* On east border => No east neighbours. (None) */
        if pos.x >= box_width && change.0 > 0 {
            return None;
        }

        /* On south border => No south neighbours. (None) */
        if pos.y < 1 && change.1 < 0 {
            return None;
        }

        /* On north border => No north neighbours. (None) */
        if pos.y >= box_depth && change.1 > 0 {
            return None;
        }

        Some(Position::new(
            (pos.x as i64 + change.0 as i64) as u32,
            (pos.y as i64 + change.1 as i64) as u32,
        ))
    }

    /** Get the optional position, which is on the given index. */
    pub fn position_from_index(&self, index: u32) -> Option<Position> {
        if index < self.width * self.depth {
            Some(Position::new(
                index % self.width + self.anchor.x,
                index / self.width + self.anchor.y,
            ))
        } else {
            None
        }
    }
}

impl Iterator for Area {
    type Item = Position;

    /** Iterator over the positions of the field. */
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.iterator_index;
        self.iterator_index += 1;
        self.position_from_index(index)
    }
}

/** Way in the world. */
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Way {
    NW,
    N,
    NE,
    W,
    E,
    SW,
    S,
    SE,
}

impl Way {
    pub const NEIGHBOURING: [Self; 8] = [
        Self::NW,
        Self::N,
        Self::NE, // north
        Self::W,
        Self::E, // same longitude
        Self::SW,
        Self::S,
        Self::SE, // south
    ];
    /** Get the offsets to walk, to get to the way point. */
    pub fn as_direction_tuple(&self) -> (i8, i8) {
        match self {
            /* Go north. */
            Way::NW => (-1, 1),
            Way::N => (0, 1),
            Way::NE => (1, 1),

            /* Stay on longitude. */
            Way::W => (-1, 0),
            Way::E => (1, 0),

            /* Go south. */
            Way::SW => (-1, -1),
            Way::S => (0, -1),
            Way::SE => (1, -1),
        }
    }
}

