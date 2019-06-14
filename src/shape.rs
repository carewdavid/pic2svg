
#[derive(Copy, Clone)]
pub struct Point (pub f32, pub f32);

impl Point {
    pub fn distance(p0: Point, p1: Point) -> f32 {
        let dx = p0.0 - p1.0;
        let dy = p0.1 - p1.1;
        f32::sqrt(dx * dx + dy * dy)
    }

}

pub trait Primitive {
    fn emit(&self);

    fn north(&self) -> Point;
    fn east(&self) -> Point;
    fn south(&self) -> Point;
    fn west(&self) -> Point;
}

pub struct Rect {
    center: Point,
    width: f32,
    height: f32
}


impl Rect {
    pub fn new(position: Point) -> Rect {
        Rect {
            center: position,
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

    fn north(&self) -> Point {
        let Point(x, y) = self.center;
        Point(x, y - self.height / 2.0)
    }

    fn east(&self) -> Point {
        let Point(x, y) = self.center;
        Point(x + self.width / 2.0, y)
    }

    fn south(&self) -> Point {
        let Point (x, y) = self.center;
        Point(x, y + self.height / 2.0)
    }

    fn west(&self) -> Point {
        let Point(x, y) = self.center;
        Point(x - self.width / 2.0, y)
    }
}

pub struct Ellipse {
    center: Point,
    width: f32,
    height: f32
}

impl Ellipse {
    pub fn new(position: Point) -> Ellipse{
        Ellipse {
            center: position,
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
    fn north(&self) -> Point {
        let Point(x, y) = self.center;
        Point(x, y - self.height)
    }

    fn east(&self) -> Point {
        let Point(x, y) = self.center;
        Point(x + self.width, y)
    }

    fn south(&self) -> Point {
        let Point (x, y) = self.center;
        Point(x, y + self.height)
    }

    fn west(&self) -> Point {
        let Point(x, y) = self.center;
        Point(x - self.width, y)
    }
}
pub struct Circle {
    center: Point,
    radius: f32
}

impl Circle {
    pub fn new(position: Point) -> Circle {
        Circle {
            center: position,
            radius: 0.25
        }
    }
}

impl Primitive for Circle {
    fn emit(&self) {
        let Point(x, y) = self.center;
        println!(r#"<circle cx="{}in" cy="{}in" r="{}in" fill="none" stroke="black"/>"#, x, y, self.radius);
    }

    fn north(&self) -> Point {
        let Point(x, y) = self.center;
        Point(x, y - self.radius)
    }

    fn east(&self) -> Point {
        let Point(x, y) = self.center;
        Point(x + self.radius, y)
    }

    fn south(&self) -> Point {
        let Point (x, y) = self.center;
        Point(x, y + self.radius)
    }

    fn west(&self) -> Point {
        let Point(x, y) = self.center;
        Point(x - self.radius, y)
    }
}
