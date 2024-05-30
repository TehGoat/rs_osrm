use super::route_leg::RouteLeg;


#[derive(Debug)]
pub struct Route {
    pub duration: f64,
    pub distance: f64,
    pub weight_name: Option<String>,
    pub weight: f64,
    pub geometry: Option<String>,
    pub legs: Vec<RouteLeg>,
}