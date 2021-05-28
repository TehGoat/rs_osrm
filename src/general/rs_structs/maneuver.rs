use crate::general::{Coordinate, c_string_to_option_string, c_string_to_string, c_structs::c_maneuver::COsrmManeuver};

#[derive(Debug)]
pub struct Maneuver {
    pub bearing_before: i32,
    pub bearing_after: i32,
    pub coordinate: Coordinate,
    pub maneuver_type: String,
    pub modifer: Option<String>,
}

impl From<&COsrmManeuver> for Maneuver {
    fn from(c_maneuver: &COsrmManeuver) -> Self {
        Maneuver {
            bearing_before: c_maneuver.bearing_before,
            bearing_after: c_maneuver.bearing_after,
            coordinate: (&c_maneuver.coordinate).into(),
            maneuver_type: c_string_to_string(c_maneuver.maneuver_type),
            modifer: c_string_to_option_string(c_maneuver.modifer),
        }
    }
}