
#[derive(Copy, Clone)]
pub struct Point (pub f32, pub f32);

impl Point {
    pub fn distance(p0: Point, p1: Point) -> f32 {
        let dx = p0.0 - p1.0;
        let dy = p0.1 - p1.1;
        f32::sqrt(dx * dx + dy * dy)
    }

    pub fn add(p0: Point, p1: Point) -> Point {
        Point(p0.0 + p1.0, p0.1 + p1.1)
    }

    pub fn sub(p0: Point, p1: Point) -> Point {
        Point(p0.0 - p1.0, p0.1 - p1.1)
    }

    pub fn mul(p: Point, scalar: f32) -> Point {
        Point(p.0 * scalar, p.1 * scalar)
    }

}

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

pub trait Primitive {
    fn emit(&self);
    fn set_location(&mut self, loc: Point);

    //Technically, we don't know what fields each primitive has, so can't
    //provide a default implementation even though it's going to be the same
    //for most of them. Fml.
    fn center(&self) -> Point;
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

    fn set_location(&mut self, loc: Point) {
        self.center = loc;
    }

    fn center(&self) -> Point {
        self.center
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
    fn set_location(&mut self, loc: Point) {
        self.center = loc;
    }

    fn center(&self) -> Point {
        self.center
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

    fn set_location(&mut self, loc: Point) {
        self.center = loc;
    }

    fn center(&self) -> Point {
        self.center
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

pub struct Line {
    start: Point,
    end: Point
}

impl Line {
    pub fn new(start: Point, dir: Direction) -> Line {
        //The default length for lines is 0.5in
        let length = 0.5;
            
        let end = match dir {
            Direction::Left => Point::add(start, Point(-length, 0.0)),
            Direction::Right => Point::add(start, Point(length, 0.0)),
            Direction::Down => Point::add(start, Point(0.0, length)),
            Direction::Up => Point::add(start, Point(0.0, -length))
        };
        Line { start: start, end: end}
    }
}

impl Primitive for Line {
    fn emit(&self) {
        //These names are a little inconsistent with coordinate names elsewhere, but they're
        //the ones svg uses.
        let Point(x1, y1) = self.start;
        let Point(x2, y2) = self.end;
        println!(r#"<line x1="{}in" y1="{}in" x2="{}in" y2="{}in" stroke="black"/>"#, x1, y1, x2, y2);
    }

    //Move the start point of the line to loc
    fn set_location(&mut self, loc: Point) {
        //Calling code expects loc to be the center of the figure
        let center = Point::mul(Point::add(self.start, self.end), 0.5);

        //Get the vector from the initial point to the new one...
        let offset = Point::sub(loc, center);

        //and use it to move the ends to the right place
        self.start = Point::add(self.start, offset);
        self.end = Point::add(self.end, offset);
    }
            
    fn center(&self) -> Point {
        let Point(x0, y0) = self.start;
        let Point(x1, y1) = self.end;
        Point((x0 + x1) / 2.0, (y0 + y1) / 2.0)
    }
    
    //None of the specifications for Pic I can find say what these mean for lines
    //Fortunately, real groff seems to just use whichever endpoint is farthest in the
    //relevant direction, which is easy enough
    fn north(&self) -> Point {
        if self.start.1 < self.end.1 {
            self.start
        }else{
            self.end
        }
    }

    fn south(&self) -> Point {
        if self.start.1 > self.end.1 {
            self.start
        }else{
            self.end
        }
    }

    fn east(&self) -> Point {
        if self.start.0 > self.start.1 {
            self.end
        }else{
            self.start
        }
    }
        
    fn west(&self) -> Point {
        if self.start.0 < self.start.1 {
            self.end
        }else{
            self.start
        }
    }
}

//This one is just a wrapper around line. It's the same thing, just with a triangle at the end
pub struct Arrow {
    shaft: Line
}

impl Arrow {
    pub fn new(start: Point, dir: Direction) -> Arrow {
        Arrow{
            shaft: Line::new(start, dir)
        }
    }
}

impl Primitive for Arrow {
    fn emit(&self) {
        //Make the head of the arrow

        //Get a normal vector pointing from the end of the arrow to the start
        let back = Point::sub(self.shaft.start, self.shaft.end);
        let magnitude = Point::distance(Point(0.0,0.0), back);
        let back = Point::mul(back, 1.0 / magnitude);
        //Scale to 1/8in
        let back = Point::mul(back, 0.125);

        let angle = 5.0;

        let left = Point::add(self.shaft.end, Point(back.0 * f32::cos(-angle) - back.1 * f32::sin(-angle),
                         back.0 * f32::sin(-angle) + back.1 * f32::cos(-angle)));
        let right = Point::add(self.shaft.end, Point(back.0 * f32::cos(angle) - back.1 * f32::sin(angle),
                         back.0 * f32::sin(angle) + back.1 * f32::cos(angle)));
        
        self.shaft.emit();
        //You can't specify units in the polygon points, so put the arrowhead
        //in a group and scale it
        println!("<g transform=\"scale(96, 96)\">");
        println!(r#"<polygon points="{},{} {},{} {},{}" fill="black" />"#, self.shaft.end.0, self.shaft.end.1, left.0, left.1, right.0, right.1);
        println!("</g>");
    }

    //All these functions are just going to forward to the ones on the internal line
    fn set_location(&mut self, center: Point) {
        self.shaft.set_location(center);
    }

    fn center(&self) -> Point {
        self.shaft.center()
    }

    fn north(&self) -> Point {
        self.shaft.north()
    }
    
    fn south(&self) -> Point {
        self.shaft.south()
    }

    fn east(&self) -> Point {
        self.shaft.east()
    }

    fn west(&self) -> Point {
        self.shaft.west()
    }
}
