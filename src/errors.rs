pub enum Error {
    CellAlreadyTaken,
    LocationOutOfBounds,
}

impl Error {
    pub fn message(&self) -> String {
        match self {
            Error::CellAlreadyTaken => String::from("That cell is already taken. Try another."),
            Error::LocationOutOfBounds => String::from("Invalid location."),
        }
    }
}
