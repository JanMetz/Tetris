use num_traits::abs;

#[derive(Copy, Clone)]
pub(crate) struct Point{
    pub(crate) x: i32, //position on x-axis
    pub(crate) y: i32 //position on y-axis
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn translate_by(point: &mut Point, ref_point : &Point){
    point.x += ref_point.x;
    point.y += ref_point.y;
}

pub fn translate_to_screen(point: &Point, ref_point : &Point) -> Point{
    let mut new_point = point.clone();
    new_point.x += ref_point.x;
    new_point.y = ref_point.y - point.y;

    return new_point;
}

pub fn are_touching(p1 : &Point, p2 : &Point) -> bool {
    (p1.y == p2.y && abs(p1.x - p2.x) <= 1) || (p1.x == p2.x && abs(p1.y - p2.y) <= 1)
}
