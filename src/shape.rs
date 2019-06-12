
pub struct Point (f32, f32);

impl Point {
    fn distance(&self, p: &Point) -> f32 {
        let dx = self.0 - p.0;
        let dy = self.1 - p.1;
        f32::sqrt(dx * dx + dy * dy)
    }

}

pub trait Primitive {
    fn emit(&self);
}

pub struct Rect {
    center: Point,
    width: f32,
    height: f32
}


impl Rect {
    pub fn new(x: f32, y: f32) -> Rect {
        Rect {
            center: Point(x, y),
            width: 0.75,
            height: 0.5
        }
    }
}

impl Primitive for Rect {
    fn emit(&self) {
        //Get the upper left corner since that's what svg uses for the position
        let Point(x, y) = self.center;
        let cornerx = x - (self.width / 2.0);
        let cornery = y - (self.height / 2.0);
        println!(r#"<rect x="{}in" y="{}in" width="0.75in" height="0.5in" fill="none" stroke="black"/>"#, cornerx, cornery);
    }
}

pub struct Ellipse {
    center: Point,
    width: f32,
    height: f32
}

impl Ellipse {
    pub fn new(x: f32, y: f32) -> Ellipse{
        Ellipse {
            center: Point(x, y),
            width: 0.375,
            height: 0.25
        }
    }
} 

impl Primitive for Ellipse {

    fn emit(&self) {
        let Point(x, y) = self.center;
        println!(r#"<ellipse cx="{}in" cy="{}in" rx="{}in" ry="{}in" fill="none" stroke="black"/>"#, x, y, self.width, self.height);
    }
}
pub struct Circle {
    center: Point,
    radius: f32
}

impl Circle {
    pub fn new(x: f32, y: f32) -> Circle {
        Circle {
            center: Point(x, y),
            radius: 0.25
        }
    }
}

impl Primitive for Circle {
    fn emit(&self) {
        let Point(x, y) = self.center;
        println!(r#"<circle cx="{}in" cy="{}in" r="{}in" fill="none" stroke="black"/>"#, x, y, self.radius);
    }
}
