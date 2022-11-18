use crate::general::c_structs::c_coordinate::COsrmCoordinate;

#[derive(Debug, Clone)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinate {
    pub fn new(latitude: f64, longitude: f64) -> Coordinate {
        Coordinate {
            latitude,
            longitude,
        }
    }

    pub fn set_latitude<'a>(&'a mut self, latitude: f64) -> &'a mut Self {
        self.latitude = latitude;
        self
    }

    pub fn set_longitude<'a>(&'a mut self, longitude: f64) -> &'a mut Self {
        self.longitude = longitude;
        self
    }
}

impl From<&COsrmCoordinate> for Coordinate {
    fn from(c_coordinate: &COsrmCoordinate) -> Self {
        Coordinate {
            latitude: c_coordinate.latitude,
            longitude: c_coordinate.longitude,
        }
    }
}
