#[derive(Debug)]
pub enum Days {
    All,
    Day1,
    Day2,
    Day3,
    Day4,
    Day5
}

impl Days {
    pub fn path(&self) -> &str {
        match self {
            Self::Day5 => "inputs/day05.txt",
            _ => todo!("{self:?}")
        }
    }

    pub fn test_path(&self) -> &str {
        match self {
            Self::Day5 => "inputs/day05-test.txt",
            _ => todo!("{self:?}")
        }
    }
}

impl From<i32> for Days {
    fn from(value: i32) -> Self {
        match value {
            1 => Days::Day1,
            2 => Days::Day2,
            3 => Days::Day3,
            4 => Days::Day4,
            5 => Days::Day5,
            _ => Days::All
        }
    }
}

// pub trait DayImpl {

// }

// impl Into<>
