pub mod shape {

    struct Point (f32, f32);

    impl Point {
        fn distance(&self, &Point p) -> f32 {
            let dx = self.0 - p.0;
            let dy = self.1 - p.1;
            f32::sqrt(dx * dx + dy * dy)
        }
    }

    trait Primitive {
        fn emit(&self);

    }

    struct Rect {
        center: Point,
        width: f32,
        height: f32
    }

    impl Primitive for Rect {
        fn emit(&self) {
            println!(r#"<rect x="{}in" y="{}in" width="0.75in" height="0.5in" fill="none" stroke="black"/>"#, self.x, self.y);
        }
    }

    struct Ellipse {
        center: Point,
        width: f32,
        height: f32
    }
    impl Primitive for Ellipse {
        fn emit(&self) {
    println!(r#"<ellipse cx="{}in" cy="{}in" rx="0.375in" ry="0.25in" fill="none" stroke="black"/>"#, self.x, self.y);
        }
    }
    struct Circle {
        center: Point,
        radius: f32
    }
    impl Primitive for Circle {
        fn emit(&self) {
    println!(r#"<circle cx="{}in" cy="{}in" r="0.25in" fill="none" stroke="black"/>"#, self.x, self.y);
        }
    }
}
