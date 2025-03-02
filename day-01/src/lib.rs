pub type LocationId = i64;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocationPair {
    Left(LocationId),
    Right(LocationId),
}

impl LocationPair {
    pub fn get_id(&self) -> LocationId {
        match self {
            LocationPair::Left(id) => *id,
            LocationPair::Right(id) => *id,
        }
    }

    pub fn is_left(&self) -> bool {
        matches!(self, LocationPair::Left(_))
    }

    pub fn is_right(&self) -> bool {
        matches!(self, LocationPair::Right(_))
    }
}

pub struct Location {
    pair: Vec<LocationPair>,
}

impl Location {
    pub fn new(input: &str) -> Self {
        let mut result: Vec<LocationPair> = vec![];

        input.lines().for_each(|line| {
            for (i, id) in line.split_whitespace().into_iter().enumerate() {
                let parsed_str = id.parse::<i64>().unwrap();
                match i % 2 != 0 {
                    true => result.push(LocationPair::Left(parsed_str)),
                    false => result.push(LocationPair::Right(parsed_str)),
                }
            }
        });

        Self { pair: result }
    }

    pub fn into_parts(&self) -> (Vec<LocationPair>, Vec<LocationPair>) {
        let mut left: Vec<LocationPair> = self
            .pair
            .iter()
            .filter(|item| matches!(item, LocationPair::Left(_)))
            .cloned() // Add this to clone the filtered items
            .collect();
        let mut right: Vec<LocationPair> = self
            .pair
            .iter()
            .filter(|item| matches!(item, LocationPair::Right(_)))
            .cloned() // Add this to clone the filtered items
            .collect();

        if left.len() != right.len() {
            panic!("This file is not pair of location id. The length doesn't match!");
        }

        left.sort();
        right.sort();

        (left, right)
    }
}
